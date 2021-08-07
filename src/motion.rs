// use types::{Pull, MotionResult, IOData};
use async_std::channel::{ Sender, SendError, RecvError, Receiver};
use std::collections::VecDeque;
use async_std::io as aio;
use async_std::process as aip;
use async_std::prelude::*;
use super::monitor::MonitorMessage;

#[derive(Debug, PartialEq, Clone)]
pub struct IOData (pub usize, pub [u8; 255]);

#[derive(Debug, PartialEq, Clone)]
pub enum IODataWrapper {
   IOData(IOData),
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


pub async fn motion_read<'a>(stdin: &mut Pull) -> MotionResult<IODataWrapper> {
    let mut buf: [u8; 255] = [0; 255];
    match stdin {
        Pull::None => Ok(IODataWrapper::Finished),
        Pull::IoMock(v) => MotionResult::Ok(v.pop_front().map(|d| IODataWrapper::IOData(d)).unwrap_or(IODataWrapper::Finished)),
        Pull::Receiver(rd) => match rd.recv().await {
            Ok(d) => Ok(IODataWrapper::IOData(d)),
            Err(RecvError) => Ok(IODataWrapper::Finished)
        },
        Pull::IoReceiver(rd) => match rd.recv().await {
            Ok(d) => Ok(IODataWrapper::IOData(d)),
            Err(RecvError) => Ok(IODataWrapper::Finished)
        },
        Pull::CmdStderr(rd) => {
            match rd.read(&mut buf).await? {
                0 => Ok(IODataWrapper::Finished),
                n => Ok(IODataWrapper::IOData(IOData(n, buf))),
            }
        }
        Pull::CmdStdout(rd) => {
            match rd.read(&mut buf).await? {
                0 => Ok(IODataWrapper::Finished),
                n => Ok(IODataWrapper::IOData(IOData(n, buf))),
            }
        }
        Pull::Stdin(rd) => {
            match rd.read(&mut buf).await? {
                0 => Ok(IODataWrapper::Finished),
                n => Ok(IODataWrapper::IOData(IOData(n, buf))),
            }
        }
    }
}

pub async fn motion_write(stdout: &mut Push, data: IOData) -> MotionResult<()> {
    match stdout {
        Push::None => MotionResult::Ok(()),
        Push::IoMock(v) => MotionResult::Ok(v.push_back(data)),
        Push::Sender(wr) => Ok(wr.send(data).await?),
        Push::IoSender(wr) => Ok(wr.send(data).await?),
        Push::CmdStdin(wr) => Ok(wr.write_all(&data.1[0..data.0]).await?),
        Push::Stdout(wr) => Ok(wr.write_all(&data.1[0..data.0]).await?),
        Push::Stderr(wr) => Ok(wr.write_all(&data.1[0..data.0]).await?),
    }
}


pub async fn motion_close(stdout: &mut Push) -> MotionResult<()> {
    match stdout {
        Push::None => MotionResult::Ok(()),
        Push::IoMock(_v) => MotionResult::Ok(()),
        Push::Sender(wr) => { wr.close(); Ok(()) },
        Push::IoSender(wr) => { wr.close(); Ok(()) },
        Push::CmdStdin(wr) => Ok(wr.flush().await?),
        Push::Stdout(wr) => Ok(wr.flush().await?),
        Push::Stderr(wr) => Ok(wr.flush().await?),
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

pub async fn motion_one(pulls: &mut Vec<Pull>, monitor: &mut MotionNotifications, pushs: &mut Vec<Push>) -> MotionResult<Vec<usize>> {
    let mut finished_pulls = vec![];
    for (pull_config_index, pull_config) in pulls.into_iter().enumerate() {
        if finished_pulls.contains(&pull_config_index) { continue; }
        let data = motion_read(pull_config).await?;
        if data == IODataWrapper::Finished {
            finished_pulls.push(pull_config_index);
            continue;
        }
        match &monitor.read {
            Some(m) => {
                m.send(MonitorMessage::Index(pull_config_index)).await
            }
            None => Ok(())
        }?;
        let was_finished = data == IODataWrapper::Finished;
        match pushs.len() == 1 {
            true => {
                match data {
                    IODataWrapper::Finished => motion_close(&mut pushs[0]).await,
                    IODataWrapper::IOData(iodata) => motion_write(&mut pushs[0], iodata).await,
                }?;
                // motion_write(&mut pushs[0], data).await?;
                match (was_finished, &monitor.written) {
                    (false, Some(m)) => m.send(MonitorMessage::Index(0)).await,
                    _ => Ok(())
                }?;
                was_finished
            },
            false => {
                for (index, push) in pushs.iter_mut().enumerate() {
                    match data.clone() {
                        IODataWrapper::Finished => motion_close(push).await,
                        IODataWrapper::IOData(iodata) => motion_write(push, iodata).await,
                    }?;
                    match (was_finished, &monitor.written) {
                        (false, Some(m)) => m.send(MonitorMessage::Index(index)).await,
                        _ => Ok(())
                    }?;
                }
                was_finished
            }
        };
    }
    MotionResult::Ok(finished_pulls)
}


pub async fn motion(mut pulls: Vec<Pull>, mut monitor: MotionNotifications, mut pushs: Vec<Push>) -> MotionResult<usize> {
    let mut read_count = 0;

    loop {
        if pulls.len() == 0 {
            for push in pushs.iter_mut() {
                motion_close(push).await?;
            }
            monitor.read.map(|m| m.close());
            monitor.written.map(|m| m.close());
            return MotionResult::Ok(read_count);
        }

        let finished_pulls = motion_one(&mut pulls, &mut monitor, &mut pushs).await?;
        read_count += 1;

        for i in finished_pulls.into_iter().rev() {
            pulls.remove(i);
        }
    }

}


#[test]
fn test_motion() {

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
        source.push_back(IOData(1, [1; 255]));
        source.push_back(IOData(1, [2; 255]));
        let (s_snd0, s_rcv0) = bounded(1);
        let pull_config_1 = vec![Pull::IoMock(source)];
        let push_config_1 = vec![Push::IoSender(s_snd0)];

        let motion1 = motion(
            pull_config_1,
            MotionNotifications::both(chan_0_read_snd, chan_0_writ_snd),
            push_config_1
        );

        // let (sndi1, rcvi1) = bounded(1);
        let pull_config_splitter = vec![
            Pull::IoReceiver(s_rcv0),
        ];
        let (snd1, rcv1) = unbounded();
        let (snd2, rcv2) = unbounded();
        let push_config_splitter = vec![Push::Sender(snd1), Push::Sender(snd2)];
        let motion2 = motion(
            pull_config_splitter,
            MotionNotifications::both(chan_1_read_snd, chan_1_writ_snd),
            push_config_splitter
        );

        let joiner_pull_configs = vec![
            Pull::Receiver(rcv1),
            Pull::Receiver(rcv2),
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
        let mut mock_stdout_pull_2_pull = Pull::Receiver(mock_stdout_pull_2);
        loop {
            let msg = motion_read(&mut mock_stdout_pull_2_pull).await.unwrap();
            if msg == IODataWrapper::Finished {
                stdout.push(msg);
                break;
            }
            stdout.push(msg);
        }
        assert_eq!(
            stdout,
            &[
                IODataWrapper::IOData(IOData(1, [1; 255])),
                IODataWrapper::IOData(IOData(1, [1; 255])),
                IODataWrapper::IOData(IOData(1, [2; 255])),
                IODataWrapper::IOData(IOData(1, [2; 255])),
                IODataWrapper::Finished,
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

    println!("R: {:?}", async_std::task::block_on(test_motion_impl()));

    // assert_eq!(1, 0);
}
