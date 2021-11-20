use crate::connectable::ConnectableAddInputError;
use crate::connectable::ConnectableAddOutputError;
use crate::connectable::OutputPort;
use crate::connectable::Connectable;
use async_std::channel::bounded;
use crate::motion::motion_close;
use crate::utils::{ remove_vec_vec, remove_vec_vec_index };

use super::motion::{ MotionResult, MotionNotifications, Pull, Push, };
use crate::startable_control::StartableControl;
use async_trait::async_trait;
use crate::back_off::BackOff;

pub struct Junction {
    stdout_size: usize,
    started: bool,
    stdout: Vec<Push>,
    stdin: Vec<Vec<Pull>>,
    inputs: Vec<(Pull, isize)>,
}

#[allow(clippy::new_without_default)]
impl Junction {
    pub fn new() -> Junction {
        Junction {
            stdout_size: 8,
            started: false,
            stdin: vec![],
            stdout: vec![],
            inputs: vec![],
        }
    }


    pub fn set_stdout_size(&mut self, size: usize) {
        self.stdout_size = size;
    }


    pub fn initialize_stdin(&mut self) {
        self.inputs.sort_by(|(_, a), (_, b)| {
            a.cmp(b)
        });

        let mut old_priority = 0;
        for (pull, priority) in std::mem::take(&mut self.inputs) {
            if self.stdin.is_empty() {
                old_priority = priority;
                self.stdin.push(vec![])
            }
            if priority > old_priority {
                self.stdin.push(vec![])
            }
            if let Some(v) = self.stdin.last_mut() {
                v.push(pull);
            }
        }
    }


    async fn iteration(&mut self, notifications: &mut MotionNotifications, back_off: &mut BackOff) -> MotionResult<(bool, usize)> {
        let mut finished = vec![];
        let mut any_read = false;
        let mut read_count = 0;

        for (si_index, mut si) in self.stdin.iter_mut().enumerate() {
            let motion_one_results = crate::motion::motion_one(
                &mut si,
                notifications,
                &mut self.stdout,
                true
            ).await?;
            read_count += motion_one_results.read_from.len();
            for fin in motion_one_results.finished_pulls.iter() {
                finished.push((si_index, *fin));
            }
            if motion_one_results.skipped.len() < si.len() {
                any_read = true;
                break;
            }
        }

        for (fin_0, fin_1) in finished.iter().rev() {
            remove_vec_vec_index(&mut self.stdin, *fin_0, *fin_1);
        }

        remove_vec_vec(&mut self.stdin);
        if self.stdin.is_empty() {
            for out in self.stdout.iter_mut() {
                motion_close(out).await?;
            }
            return MotionResult::Ok((true, read_count));
        }
        match any_read {
            false => back_off.wait().await,
            true => back_off.reset(),
        };
        MotionResult::Ok((false, read_count))
    }

}


impl Connectable for Junction {

    fn add_output(&mut self, _port: OutputPort) -> std::result::Result<Pull, ConnectableAddOutputError> {
        let (child_stdout_push_channel, stdout_io_reciever_channel) = bounded(self.stdout_size);
        self.stdout.push(Push::IoSender(child_stdout_push_channel));
        Ok(Pull::Receiver(stdout_io_reciever_channel))
    }

    fn add_input(&mut self, pull: Pull, priority: isize) -> std::result::Result<(), ConnectableAddInputError> {
        self.inputs.push((pull, priority));
        Ok(())
    }

}


#[async_trait]
impl StartableControl for Junction {

    async fn start(&mut self) -> MotionResult<usize> {
        assert!(!self.started);
        self.started = true;

        self.initialize_stdin();

        let mut back_off = BackOff::new();
        let mut read_count = 0;
        let mut notifications = MotionNotifications::empty();

        loop {
            match self.iteration(&mut notifications, &mut back_off).await {
                Ok((true, n)) => {
                    return Ok(read_count + n);
                }
                Ok((false, n)) => { read_count += n }
                Err(e) => { return MotionResult::Err(e); }
            }
        }

    }
}




#[test]
fn do_stuff() {

    pub async fn test_junction_impl() -> MotionResult<usize>  {

        async fn read_data(mut output: &mut Pull) -> Vec<IOData> {
            let mut v = vec![];
            loop {
                let x: MotionResult<crate::motion::IODataWrapper> = crate::motion::motion_read(&mut output, true).await;
                match x {
                    Ok(crate::motion::IODataWrapper::IOData(x)) => {
                        v.push(x)
                    }
                    _ => {
                        return v;
                    }
                }
            }
        }

        let (chan_0_0_snd, chan_0_0_rcv) = bounded(256);
        let (chan_0_1_snd, chan_0_1_rcv) = bounded(256);
        let (chan_1_0_snd, chan_1_0_rcv) = bounded(256);

        chan_0_0_snd.send(IOData(vec![65; 8])).await.unwrap();
        chan_0_0_snd.send(IOData(vec![66; 8])).await.unwrap();
        chan_0_1_snd.send(IOData(vec![70; 8])).await.unwrap();
        chan_0_1_snd.send(IOData(vec![71; 8])).await.unwrap();
        chan_1_0_snd.send(IOData(vec![75; 8])).await.unwrap();

        // chan_0_0_snd.close();
        // chan_0_1_snd.close();
        // chan_1_0_snd.close();

        let pull_0_0 = Pull::Receiver(chan_0_0_rcv);
        let pull_0_1 = Pull::Receiver(chan_0_1_rcv);
        let pull_1_0 = Pull::Receiver(chan_1_0_rcv);

        let mut junction = Junction::new();
        junction.set_stdout_size(8);
        junction.add_input(pull_0_0, 0).unwrap();
        junction.add_input(pull_0_1, 0).unwrap();
        junction.add_input(pull_1_0, 1).unwrap();
        junction.initialize_stdin();
        let mut output_1 = junction.add_output(OutputPort::Out).unwrap();
        let mut output_2 = junction.add_output(OutputPort::Out).unwrap();

        let mut back_off = BackOff::new();
        let mut notifications = MotionNotifications::empty();

        assert_eq!(
            junction.iteration(&mut notifications, &mut back_off).await,
            Ok((false, 2))
        );

        let output_1_result_0 = read_data(&mut output_1).await;
        assert_eq!(output_1_result_0, read_data(&mut output_2).await);
        assert_eq!(
            output_1_result_0,
            vec![
                IOData(vec![65; 8]),
                IOData(vec![70; 8]),
            ]
        );

        assert_eq!(
            junction.iteration(&mut notifications, &mut back_off).await,
            Ok((false, 2))
        );
        let output_1_result_1 = read_data(&mut output_1).await;
        assert_eq!(output_1_result_1, read_data(&mut output_2).await);
        assert_eq!(
            output_1_result_1,
            vec![
                IOData(vec![66; 8]),
                IOData(vec![71; 8]),
            ]
        );

        assert_eq!(
            junction.iteration(&mut notifications, &mut back_off).await,
            Ok((false, 1))
        );
        let output_1_result_2 = read_data(&mut output_1).await;
        assert_eq!(output_1_result_2, read_data(&mut output_2).await);
        assert_eq!(
            output_1_result_2,
            vec![IOData(vec![75; 8])],
        );

        chan_0_0_snd.send(IOData(vec![67; 8])).await.unwrap();
        chan_0_0_snd.close();

        assert_eq!(
            junction.iteration(&mut notifications, &mut back_off).await,
            Ok((false, 1))
        );
        let output_1_result_2 = read_data(&mut output_1).await;
        assert_eq!(output_1_result_2, read_data(&mut output_2).await);
        assert_eq!(
            output_1_result_2,
            vec![IOData(vec![67; 8])],
        );

        assert_eq!(
            junction.iteration(&mut notifications, &mut back_off).await,
            Ok((false, 1))
        );
        let output_1_result_2 = read_data(&mut output_1).await;
        assert_eq!(output_1_result_2, read_data(&mut output_2).await);
        assert_eq!(
            output_1_result_2,
            vec![]
        );

        chan_0_1_snd.close();
        chan_1_0_snd.close();
        assert_eq!(
            junction.iteration(&mut notifications, &mut back_off).await,
            Ok((false, 1))
        );
        let output_1_result_2 = read_data(&mut output_1).await;
        assert_eq!(output_1_result_2, read_data(&mut output_2).await);
        assert_eq!(
            output_1_result_2,
            vec![]
        );

        assert_eq!(
            junction.iteration(&mut notifications, &mut back_off).await,
            Ok((true, 1))
        );

        MotionResult::Ok(1)
    }

    use async_std::task;
    println!("R: {:?}", task::block_on(test_junction_impl()));
}
