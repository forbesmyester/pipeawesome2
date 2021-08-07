use async_std::channel::{SendError, Receiver, bounded};
use super::motion::{ MotionResult, MotionNotifications, Pull, Push, motion_close };

pub struct Faucet {
    started: bool,
    stdout_size: usize,
    stdout: Vec<Push>,
    stdin: Vec<Pull>,
    control: Option<async_std::channel::Receiver<()>>
}

impl Faucet {

    pub fn new(pull: Pull) -> Faucet {
        Faucet {
            started: false,
            stdout_size: 8,
            stdin: vec![pull],
            stdout: vec![],
            control: None,
        }
    }


    pub fn set_stdout_size(&mut self, size: usize) {
        self.stdout_size = size;
    }


    pub fn add_stdout(&mut self) -> Pull {
        assert!(self.started == false);
        assert!(self.stdout.len() == 0);
        let (child_stdout_push_channel, stdout_io_reciever_channel) = bounded(self.stdout_size);
        self.stdout.push(Push::IoSender(child_stdout_push_channel));
        Pull::IoReceiver(stdout_io_reciever_channel)
    }


    pub async fn start(&mut self) -> MotionResult<usize> {
        assert!(self.started == false);
        self.started = true;
        let mut read_count = 0;

        let mut notifications = MotionNotifications::empty();

        async fn control(opt_rec: &mut Option<Receiver<()>>) -> () {
            if let Some(rec) = opt_rec {
                match rec.try_recv() {
                    Ok(_) => { let _x = rec.recv().await; }
                    _ => (),
                }
            }
            ()
        }

        loop {
            let r = crate::motion::motion_one(
                &mut self.stdin,
                &mut notifications,
                &mut self.stdout
            ).await?;
            read_count = read_count + 1;
            if r.len() > 0 {
                for push in &mut self.stdout {
                    motion_close(push).await?
                }
                return MotionResult::Ok(read_count);
            }

            control(&mut self.control).await;
        }

    }

    pub fn get_control(&mut self) -> FaucetControl {

        let (send, recv) = bounded(1);
        self.control = Some(recv);
        FaucetControl {
            paused: false,
            control: send,
        }
    }


}

pub struct FaucetControl {
    paused: bool,
    control: async_std::channel::Sender<()>,
}


impl FaucetControl {

    pub async fn pause(&mut self) -> Result<bool, SendError<()>> {
        if self.paused == true { return Ok(false); }
        self.paused = true;
        self.control.send(()).await.map(|_x| true)
    }


    pub async fn resume(&mut self) -> Result<bool, SendError<()>> {
        if self.paused == false { return Ok(false); }
        self.paused = false;
        self.control.send(()).await.map(|_x| true)
    }


}


#[test]
fn do_stuff() {

    pub async fn test_tap_impl() -> MotionResult<usize>  {
        use async_std::channel::Sender;
        use std::time::Instant;
        use std::time::Duration;
        use async_std::{channel::RecvError, prelude::*};
        use crate::motion::IOData;

        async fn read_data_item(output: &mut Pull) -> Result<(IOData, Instant), RecvError> {
            match output {
                Pull::IoReceiver(rcv) => {
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

        async fn write_data_1(input_chan_snd: &mut Sender<IOData>, tapcontrol: &mut FaucetControl) {
            input_chan_snd.send(IOData(8, [65; 255])).await.unwrap();
            input_chan_snd.send(IOData(8, [66; 255])).await.unwrap();
            async_std::task::sleep(Duration::from_millis(100)).await;
            tapcontrol.pause().await.unwrap();
            input_chan_snd.send(IOData(8, [67; 255])).await.unwrap();
            input_chan_snd.send(IOData(8, [68; 255])).await.unwrap();
            input_chan_snd.send(IOData(8, [69; 255])).await.unwrap();
            input_chan_snd.send(IOData(8, [70; 255])).await.unwrap();
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
        let input = Pull::Receiver(input_chan_rcv);

        let mut tap = Faucet::new(input);
        let mut tapcontrol = tap.get_control();
        tap.set_stdout_size(1);
        let output_1 = tap.add_stdout();

        let w0 = tap.start();
        let w1 = write_data_1(&mut input_chan_snd, &mut tapcontrol);

        for (index, vt) in read_data(output_1).join(w0.join(w1)).await.0.iter().enumerate() {
            let diff = vt.1.duration_since(my_timings.instant);
            if diff > my_timings.duration {
                my_timings.duration = diff;
                my_timings.index = index;
                println!("UT: {:?}", my_timings);
            }
            my_timings.instant = vt.1;
            my_timings.count = my_timings.count + 1;
        }
        assert_eq!(my_timings.index, 2);
        assert_eq!(my_timings.count, 6);
        assert!(my_timings.duration > Duration::from_millis(90));
        assert!(my_timings.duration < Duration::from_millis(110));

        MotionResult::Ok(1)
    }

    use async_std::task;
    println!("R: {:?}", task::block_on(test_tap_impl()));
}
