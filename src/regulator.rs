use crate::motion::MotionError;
use crate::motion::Journey;
use crate::connectable::Connectable;
use crate::connectable::Breakable;
use crate::connectable::OutputPort;
use crate::connectable::ConnectableAddOutputError;
use crate::connectable::ConnectableAddInputError;
use crate::motion::SpyMessage;
use async_std::channel::Sender;
use async_std::channel::{SendError, Receiver, bounded};
use super::motion::{ MotionResult, MotionNotifications, Pull, Push, motion_close, motion_worker };
use crate::startable_control::StartableControl;
use async_trait::async_trait;

pub struct Regulator {
    started: bool,
    stdout_size: usize,
    stdout: Option<Push>,
    stdin: Option<Pull>,
    control: Option<async_std::channel::Receiver<()>>,
}

impl Regulator {

    pub fn new() -> Regulator {
        Regulator {
            started: false,
            stdout_size: 8,
            stdin: None,
            stdout: None,
            control: None,
        }
    }


    pub fn set_stdout_size(&mut self, size: usize) {
        self.stdout_size = size;
    }


    pub fn get_control(&mut self) -> RegulatorControl {
        assert!(self.control.is_none(), "Each regulator can only have one control");
        let (send, recv) = bounded(1);
        self.control = Some(recv);
        RegulatorControl {
            paused: false,
            control: send,
        }
    }

}


impl Default for Regulator {
    fn default() -> Self {
        Self::new()
    }
}


impl Connectable for Regulator {

    fn add_output(&mut self, port: OutputPort, breakable: Breakable, src_id: usize, dst_id: usize) -> std::result::Result<Pull, ConnectableAddOutputError> {
        if port != OutputPort::Out {
            return Err(ConnectableAddOutputError::UnsupportedPort(port));
        }
        if self.stdout.is_some() {
            return Err(ConnectableAddOutputError::AlreadyAllocated(port));
        }
        let (child_stdout_push_channel, stdout_io_reciever_channel) = bounded(self.stdout_size);
        let journey = Journey { src: src_id, src_port: Some(port), dst: dst_id, breakable };
        self.stdout = Some(Push::IoSender(journey.clone(), child_stdout_push_channel));
        Ok(Pull::Receiver(journey.as_pull_journey(), stdout_io_reciever_channel))
    }

    fn add_input(&mut self, pull: Pull, unused_priority: isize) -> std::result::Result<(), ConnectableAddInputError> {
        if unused_priority != 0 {
            return Err(ConnectableAddInputError::UnsupportedPriority(unused_priority));
        }
        if self.stdin.is_some() {
            return Err(ConnectableAddInputError::AlreadyAllocated);
        }
        self.stdin = Some(pull);
        Ok(())
    }

}



#[async_trait]
impl StartableControl for Regulator {

    async fn start(&mut self, spy: Option<Sender<SpyMessage>>) -> MotionResult<usize> {
        self.start_secret(spy).await
    }

}

impl Regulator {

    async fn start_secret(&mut self, mut spy: Option<Sender<SpyMessage>>) -> MotionResult<usize> {
        assert!(!self.started);

        self.started = true;
        let mut read_count = 0;

        let mut notifications = MotionNotifications::empty();
        async fn control(opt_rec: &mut Option<Receiver<()>>) {
            if let Some(rec) = opt_rec {
                if rec.try_recv().is_ok() {
                    let _x = rec.recv().await;
                }
            }
        }

        if let (Some(mut stdin), Some(stdout)) = (std::mem::take(&mut self.stdin), std::mem::take(&mut self.stdout)) {
            let breakable = stdout.journey().map(|j| j.breakable).unwrap_or(Breakable::Terminate);
            let mut stdouts = vec![stdout];
            loop {
                match motion_worker(&mut stdin, &mut notifications, &mut stdouts, &mut spy, false).await {
                    Ok(result) => {
                        if result.finished {
                            for push in &mut self.stdout {
                                motion_close(push).await?
                            }
                            if let Some(c) = &self.control {
                                c.close();
                            }
                            return MotionResult::Ok(read_count);
                        }
                        read_count += 1;
                        Ok(())
                    },
                    Err(e @ MotionError::OutputClosed(_, _, _, _)) => {
                        if breakable == Breakable::Terminate {
                            return Err(e);
                        }
                        if breakable == Breakable::Finish {
                            return Ok(read_count)
                        }
                        read_count += 1;
                        Ok(())
                    },
                    Err(e) => {
                        Err(e)
                    }
                }?;

                control(&mut self.control).await;
            }
        }

        Ok(0)

    }

}

#[derive(Debug)]
pub struct RegulatorControl {
    paused: bool,
    control: async_std::channel::Sender<()>,
}


impl RegulatorControl {

    pub async fn pause(&mut self) -> Result<bool, SendError<()>> {
        if self.paused { return Ok(false); }
        self.paused = true;
        self.control.send(()).await.map(|_x| true).or(Ok(false))
    }


    pub async fn resume(&mut self) -> Result<bool, SendError<()>> {
        if !self.paused { return Ok(false); }
        self.paused = false;
        self.control.send(()).await.map(|_x| true).or(Ok(false))
    }


}


#[test]
fn do_stuff() {

    use crate::motion::PullJourney;

    pub async fn test_tap_impl() -> MotionResult<usize>  {
        use async_std::channel::Sender;
        use std::time::Instant;
        use std::time::Duration;
        use async_std::{channel::RecvError, prelude::*};
        use crate::motion::IOData;

        async fn read_data_item(output: &mut Pull) -> Result<(IOData, Instant), RecvError> {
            match output {
                Pull::Receiver(_, rcv) => {
                    rcv.recv().await.map(|x| (x, Instant::now()))
                },
                _ => Err(RecvError),
            }
        }

        async fn read_data(mut output: Pull) -> Vec<(IOData, Instant)> {
            let mut v = vec![];
            loop {
                let x = read_data_item(&mut output).await;
                match x {
                    Ok(x) => {
                        v.push(x)
                    }
                    _ => {
                        return v;
                    }
                }
            }
        }

        async fn write_data_1(input_chan_snd: &mut Sender<IOData>, tapcontrol: &mut RegulatorControl) {
            input_chan_snd.send(IOData(vec![65; 8])).await.unwrap();
            input_chan_snd.send(IOData(vec![66; 8])).await.unwrap();
            async_std::task::sleep(Duration::from_millis(100)).await;
            tapcontrol.pause().await.unwrap();
            input_chan_snd.send(IOData(vec![67; 8])).await.unwrap();
            input_chan_snd.send(IOData(vec![68; 8])).await.unwrap();
            input_chan_snd.send(IOData(vec![69; 8])).await.unwrap();
            input_chan_snd.send(IOData(vec![70; 8])).await.unwrap();
            tapcontrol.resume().await.unwrap();
            input_chan_snd.close();
        }

        #[derive(Debug)]
        struct MyTimings {
            duration: Duration,
            instant: Instant,
            index: usize,
            count: usize,
        }

        let mut my_timings = MyTimings {
            instant: Instant::now(),
            duration: Duration::from_micros(0),
            index: 0,
            count: 0,
        };

        let (mut input_chan_snd, input_chan_rcv) = bounded(8);
        let input = Pull::Receiver(PullJourney { src: 0, dst: 0 }, input_chan_rcv);

        let mut tap = Regulator {
            started: false,
            stdout_size: 8,
            stdin: Some(input),
            stdout: None,
            control: None,
        };

        let mut tapcontrol = tap.get_control();
        tap.set_stdout_size(1);
        let output_1 = tap.add_output(OutputPort::Out, Breakable::Terminate, 0, 0).unwrap();

        let w0 = tap.start_secret(None);
        let w1 = write_data_1(&mut input_chan_snd, &mut tapcontrol);

        for (index, vt) in read_data(output_1).join(w0.join(w1)).await.0.iter().enumerate() {
            let diff = vt.1.duration_since(my_timings.instant);
            if diff > my_timings.duration {
                my_timings.duration = diff;
                my_timings.index = index;
            }
            my_timings.instant = vt.1;
            my_timings.count = my_timings.count + 1;
        }
        assert_eq!(my_timings.index, 2);
        assert_eq!(my_timings.count, 6);
        assert!(my_timings.duration > Duration::from_millis(90));

        MotionResult::Ok(1)
    }

    use async_std::task;
    println!("R: {:?}", task::block_on(test_tap_impl()));
}

