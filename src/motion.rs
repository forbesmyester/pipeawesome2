// use types::{Pull, MotionResult, IOData};
use async_std::channel::{ Sender, SendError, RecvError, Receiver};
use std::collections::VecDeque;
use async_std::io as aio;
use async_std::process as aip;
use async_std::prelude::*;
use super::monitor::MonitorMessage;
use super::types::InputId;

#[derive(Debug, PartialEq, Clone)]
pub enum IOData {
   Data(usize,[u8; 255]),
   Finished,
}

#[derive(Debug)]
pub enum Pull {
    CmdStdout(aip::ChildStdout),
    CmdStderr(aip::ChildStderr),
    Stdin(aio::Stdin),
    Receiver(Receiver<IOData>),
    IoReceiver(Receiver<IOData>),
    IoMock(VecDeque<IOData>),
    None,
}

#[derive(Debug)]
pub enum Push {
    IoMock(VecDeque<IOData>),
    CmdStdin(aip::ChildStdin),
    Stdout(aio::Stdout),
    Stderr(aio::Stderr),
    Sender(Sender<IOData>),
    IoSender(Sender<IOData>),
    None,
}

#[derive(Debug)]
pub enum MotionError {
    IOError(std::io::Error),
    RecvError(RecvError),
    SendError(SendError<IOData>),
    MonitorError(SendError<MonitorMessage>),
    NoneError,
}

impl PartialEq for MotionError {
    fn eq(&self, b: &MotionError) -> bool {
        match (&self, b) {
            (MotionError::RecvError(a), MotionError::RecvError(b)) => a == b,
            (MotionError::IOError(_a), MotionError::IOError(_b)) => true,
            (MotionError::NoneError, MotionError::NoneError) => true,
            (MotionError::SendError(a), MotionError::SendError(b)) => a == b,
            (MotionError::MonitorError(a), MotionError::MonitorError(b)) => a == b,
            _ => false,
        }
    }
}

pub type MotionResult<T> = Result<T, MotionError>;

impl From<std::io::Error> for MotionError {
    fn from(x: std::io::Error) -> Self {
        MotionError::IOError(x)
    }
}

impl From<RecvError> for MotionError {
    fn from(x: RecvError) -> Self {
        MotionError::RecvError(x)
    }
}

impl From<SendError<IOData>> for MotionError {
    fn from(x: SendError<IOData>) -> Self {
        MotionError::SendError(x)
    }
}

impl From<SendError<MonitorMessage>> for MotionError {
    fn from(x: SendError<MonitorMessage>) -> Self {
        MotionError::MonitorError(x)
    }
}

#[derive(Debug)]
pub struct PullConfiguration {
    pub priority: u8,
    pub id: InputId,
    pub pull: Pull,
}

pub async fn motion_read<'a>(stdin: &mut Pull) -> MotionResult<IOData> {
    let mut buf: [u8; 255] = [0; 255];
    match stdin {
        Pull::None => MotionResult::Ok(IOData::Finished),
        Pull::IoMock(v) => MotionResult::Ok(v.pop_front().unwrap_or(IOData::Finished)),
        Pull::Receiver(rd) => Ok(rd.recv().await?),
        Pull::IoReceiver(rd) => Ok(rd.recv().await?),
        Pull::CmdStderr(rd) => {
            match rd.read(&mut buf).await? {
                0 => Ok(IOData::Finished),
                n => Ok(IOData::Data(n, buf)),
            }
        }
        Pull::CmdStdout(rd) => {
            match rd.read(&mut buf).await? {
                0 => {
                println!("MOTION_READ: Pull::CmdStdout: FIN");
                    Ok(IOData::Finished)
                },
                n => {
                    println!("MOTION_READ: Pull::CmdStdout: {} {:?}", n, buf);
                    Ok(IOData::Data(n, buf))
                },
            }
        }
        Pull::Stdin(rd) => {
            match rd.read(&mut buf).await? {
                0 => Ok(IOData::Finished),
                n => Ok(IOData::Data(n, buf)),
            }
        }
    }
}

async fn motion_write(stdout: &mut Push, data: IOData) -> MotionResult<()> {
    match stdout {
        Push::None => Err(MotionError::NoneError),
        Push::IoMock(v) => MotionResult::Ok(v.push_back(data)),
        Push::Sender(wr) => Ok(wr.send(data).await?),
        Push::IoSender(wr) => Ok(wr.send(data).await?),
        Push::CmdStdin(wr) => {
            match data {
                IOData::Finished => Ok(wr.flush().await?),
                IOData::Data(count, buf) => Ok(wr.write_all(&buf[0..count]).await?),
            }
        }
        Push::Stdout(wr) => {
            match data {
                IOData::Finished => Ok(wr.flush().await?),
                IOData::Data(count, buf) => Ok(wr.write_all(&buf[0..count]).await?),
            }
        }
        Push::Stderr(wr) => {
            match data {
                IOData::Finished => Ok(wr.flush().await?),
                IOData::Data(count, buf) => Ok(wr.write_all(&buf[0..count]).await?),
            }
        }
    }
}

pub struct MotionNotifications {
    read: Option<Sender<MonitorMessage>>,
    written: Option<Sender<MonitorMessage>>,
}

impl MotionNotifications {
    pub fn empty() -> MotionNotifications {
        MotionNotifications { read: None, written: None }
    }
    pub fn read(s: Sender<MonitorMessage>) -> MotionNotifications {
        MotionNotifications { read: Some(s), written: None }
    }
    pub fn written(s: Sender<MonitorMessage>) -> MotionNotifications {
        MotionNotifications { read: None, written: Some(s) }
    }
    pub fn both(read: Sender<MonitorMessage>, written: Sender<MonitorMessage>) -> MotionNotifications {
        MotionNotifications { read: Some(read), written: Some(written) }
    }
}

pub async fn motion(mut pull_configs: Vec<PullConfiguration>, monitor: MotionNotifications, mut pushs: Vec<Push>) -> MotionResult<usize> {
    println!("MOTION: {:?}", &pull_configs);
    let mut read_count = 0;
    let mut found_priority: Option<u8> = None;
    let mut finished_pulls: std::collections::HashSet<usize> = std::collections::HashSet::new();

    loop {
        if finished_pulls.len() == pull_configs.len() {
            // println!("FINISHED: {:?} {:?} {}/{}", &pull_configs, &finished_pulls, &finished_pulls.len(), &pull_configs.len());
            for push in pushs.iter_mut() {
                motion_write(push, IOData::Finished).await?;
            }
            monitor.read.map(|m| m.close());
            monitor.written.map(|m| m.close());
            return MotionResult::Ok(read_count);
        }
        for (pull_config_index, pull_config) in pull_configs.iter_mut().enumerate() {
            if finished_pulls.contains(&pull_config_index) { continue; }
            if found_priority.map_or(false, |n| n < pull_config.priority) { continue; }
            let data = motion_read(&mut pull_config.pull).await?;
            if data == IOData::Finished {
                println!("FINISHED: {:?}", &pull_config);
                finished_pulls.insert(pull_config_index);
                continue;
            }
            match &monitor.read {
                Some(m) => {
                    // println!("R: {:?}", &data);
                    m.send(MonitorMessage::Index(pull_config_index)).await
                }
                None => Ok(())
            }?;
            found_priority = Some(pull_config.priority);
            read_count += 1;
            let was_finished = data == IOData::Finished;
            match pushs.len() == 1 {
                true => {
                    // println!("QDATA({}): {:?}", pull_config.id, &data);
                    // println!("W: {:?}", &data);
                    motion_write(&mut pushs[0], data).await?;
                    match (was_finished, &monitor.written) {
                        (false, Some(m)) => m.send(MonitorMessage::Index(0)).await,
                        _ => Ok(())
                    }?;
                    was_finished
                },
                false => {
                    for (index, push) in pushs.iter_mut().enumerate() {
                        // println!("DATA({}): {:?}", pull_config.id, &data);
                        // println!("W: {:?}", &data);
                        motion_write(push, data.clone()).await?;
                        match (was_finished, &monitor.written) {
                            (false, Some(m)) => m.send(MonitorMessage::Index(index)).await,
                            _ => Ok(())
                        }?;
                    }
                    data == IOData::Finished
                }
            };
        }
    }

}


#[test]
fn test_motion() {

    use async_std::task;
    use async_std::channel::{ bounded, unbounded };

    async fn test_motion_impl() -> ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>) {

        // source -->> s_snd0 -> s_rcv0 -->> snd1 -> rcv1 -->> mock_stdout_push_2 -> mock_stdout_pull_2
        //                              -->> snd2 -> rcv1 ⇗

        let (chan_0_read_snd, chan_0_read_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
        let (chan_0_writ_snd, chan_0_writ_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
        let (chan_1_read_snd, chan_1_read_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
        let (chan_1_writ_snd, chan_1_writ_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
        let (chan_2_read_snd, chan_2_read_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
        let (chan_2_writ_snd, chan_2_writ_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);

        let mut source = VecDeque::new();
        source.push_back(IOData::Data(1, [1; 255]));
        source.push_back(IOData::Data(1, [2; 255]));
        let (s_snd0, s_rcv0) = bounded(1);
        let pull_config_1 = vec![PullConfiguration { priority: 1, id: 0, pull: Pull::IoMock(source) } ];
        let push_config_1 = vec![Push::IoSender(s_snd0)];
        println!("## READ");

        let motion1 = motion(
            pull_config_1,
            MotionNotifications::both(chan_0_read_snd, chan_0_writ_snd),
            push_config_1
        );

        println!("## SPLIT");
        // let (sndi1, rcvi1) = bounded(1);
        let pull_config_splitter = vec![
            PullConfiguration { priority: 1, id: 1, pull: Pull::IoReceiver(s_rcv0) },
        ];
        let (snd1, rcv1) = unbounded();
        let (snd2, rcv2) = unbounded();
        let push_config_splitter = vec![Push::Sender(snd1), Push::Sender(snd2)];
        let motion2 = motion(
            pull_config_splitter,
            MotionNotifications::both(chan_1_read_snd, chan_1_writ_snd),
            push_config_splitter
        );

        println!("## JOIN");
        let joiner_pull_configs = vec![
            PullConfiguration { priority: 1, id: 2, pull: Pull::Receiver(rcv1) },
            PullConfiguration { priority: 1, id: 3, pull: Pull::Receiver(rcv2) },
        ];
        let (mock_stdout_push_2, mock_stdout_pull_2)  = bounded(8);
        let motion3 = motion(
            joiner_pull_configs,
            MotionNotifications::both(chan_2_read_snd, chan_2_writ_snd),
            vec![Push::Sender(mock_stdout_push_2)]
        );

        let f: ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>) = motion1.join(motion2).join(motion3).await;
        // let ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>)
        // let r: ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>) = task::block_on(motion1.join(motion2).join(motion3));


        // struct CommandStats {
        // }
        // fn structure_motion_result(input: ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>)) -> CommandStats {
        // }
        // join!(motion1, motion2, motion3);
        //
        //Future::join

        //assert_eq!(task::block_on(motion1).unwrap(), 1);
        //assert_eq!(task::block_on(motion2).unwrap(), 1);
        // assert_eq!(task::block_on(recvm.recv()).unwrap(), MonitorMessage { id: 1, change: 2 });
        // assert_eq!(task::block_on(recvm.recv()).unwrap(), MonitorMessage { id: 1, change: 2 });
        //assert_eq!(task::block_on(motion3).unwrap(), 2);
        // assert_eq!(task::block_on(recvm.recv()).unwrap(), MonitorMessage { id: 0, change: -1 });
        // assert_eq!(task::block_on(recvm.recv()).unwrap(), MonitorMessage { id: 0, change: -1 });
        let _expected_vecdequeue: VecDeque<IOData> = VecDeque::new();
        let mut stdout = vec![];
        loop {
            let msg = mock_stdout_pull_2.recv().await.unwrap();
            if msg == IOData::Finished {
                stdout.push(msg);
                break;
            }
            stdout.push(msg);
        }
        assert_eq!(
            stdout,
            &[
                IOData::Data(1, [1; 255]),
                IOData::Data(1, [1; 255]),
                IOData::Data(1, [2; 255]),
                IOData::Data(1, [2; 255]),
                IOData::Finished,
            ]
        );

        assert_eq!(chan_0_read_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_0_read_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_0_read_rcv.is_closed(), true);

        assert_eq!(chan_0_writ_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_0_writ_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_0_writ_rcv.is_closed(), true);

        assert_eq!(chan_1_read_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_1_read_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_1_read_rcv.is_closed(), true);

        assert_eq!(chan_1_writ_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_1_writ_rcv.recv().await.unwrap(), MonitorMessage::Index(1));
        assert_eq!(chan_1_writ_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_1_writ_rcv.recv().await.unwrap(), MonitorMessage::Index(1));
        assert_eq!(chan_1_writ_rcv.is_closed(), true);

        assert_eq!(chan_2_read_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_2_read_rcv.recv().await.unwrap(), MonitorMessage::Index(1));
        assert_eq!(chan_2_read_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_2_read_rcv.recv().await.unwrap(), MonitorMessage::Index(1));
        assert_eq!(chan_2_read_rcv.is_closed(), true);

        assert_eq!(chan_2_writ_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_2_writ_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_2_writ_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_2_writ_rcv.recv().await.unwrap(), MonitorMessage::Index(0));
        assert_eq!(chan_2_read_rcv.is_closed(), true);

        f
    }


    println!("R: {:?}", task::block_on(test_motion_impl()));
    // assert_eq!(1, 0);
}
