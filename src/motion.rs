// use types::{Pull, MotionResult, IOData};
use async_std::channel::{Receiver, RecvError, SendError, Sender, TryRecvError};
use futures::AsyncRead;
use std::collections::VecDeque;
use async_std::io as aio;
use async_std::process as aip;
use async_std::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct IOData(pub Vec<u8>);

#[derive(PartialEq,Debug)]
pub enum MonitorMessage {
    Read(usize),
    Wrote(usize),
}

#[derive(Debug, PartialEq, Clone)]
pub enum IODataWrapper {
   IOData(IOData),
   Finished,
   Skipped,
}

#[derive(Debug)]
pub struct ReadSplitControl {
    split_at: Vec<Vec<u8>>,
    overflow: Vec<u8>,
}

impl ReadSplitControl {
    pub fn new() -> ReadSplitControl {
        ReadSplitControl { split_at: vec!["\r\n".as_bytes().iter().copied().collect(), "\n".as_bytes().iter().copied().collect()], overflow: vec![] }
    }
}


impl Default for ReadSplitControl {
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Debug)]
pub enum Pull {
    CmdStdout(aip::ChildStdout, ReadSplitControl),
    CmdStderr(aip::ChildStderr, ReadSplitControl),
    Stdin(aio::Stdin, ReadSplitControl),
    File(async_std::fs::File, ReadSplitControl),
    Receiver(Receiver<IOData>),
    Mock(VecDeque<IOData>),
    None,
}

#[derive(Debug)]
pub enum Push {
    IoMock(VecDeque<IOData>),
    CmdStdin(aip::ChildStdin),
    Stdout(aio::Stdout),
    Stderr(aio::Stderr),
    File(aio::BufWriter<async_std::fs::File>),
    Sender(Sender<IOData>),
    IoSender(Sender<IOData>),
    None,
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
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

fn is_split(buf: &[u8], splits: &[Vec<u8>]) -> Option<usize>
{
    for v in splits {
        if v.len() > buf.len() {
            continue;
        }
        let (pre, _) = buf.split_at(v.len());
        if pre == v {
            return Some(v.len());
        }
    }
    None
}


async fn motion_read_buffer(rd: &mut (dyn AsyncRead + Unpin + Send), overflow: &mut ReadSplitControl) -> Result<Vec<u8>, MotionError> {

    loop {

        // It might already have been read into the buffer
        for overflow_pos in 0..overflow.overflow.len() {
            if let Some(split_length) = is_split(overflow.overflow.split_at(overflow_pos).1, &overflow.split_at) {
                let mut new_overflow = overflow.overflow.split_off(overflow_pos + split_length);
                std::mem::swap(&mut new_overflow, &mut overflow.overflow);
                return Ok(new_overflow)
            }
        }

        let mut buf: [u8; 255] = [0; 255];
        let bytes_read = rd.read(&mut buf).await?;

        // If end of stream
        if bytes_read == 0
        {
            return Ok(std::mem::take(&mut overflow.overflow));
        }

        overflow.overflow.extend_from_slice(buf.split_at(bytes_read).0);


        // // Read the data then break and store at seperator
        // for buf_pos in 0..bytes_read {
        //     // If we found a split point
        //     if let Some(split_length) = is_split(buf.split_at(buf_pos).1, &overflow.split_at) {
        //         // Split it
        //         let (pre, post) = buf.split_at(buf_pos + split_length);
        //         // println!("(buf_pos, split_length, pre, post): ({:?}, {:?}, {:?}, {:?})", buf_pos, split_length, pre, post);

        //         // Take what is in the overflow and append up to the split point, this is what we will return
        //         let mut r = std::mem::take(&mut overflow.overflow);
        //         r.extend_from_slice(pre);

        //         // The new overflow is everything past the split point
        //         overflow.overflow.extend_from_slice(post.split_at(bytes_read - (buf_pos + split_length)).0);

        //         return Ok(r)
        //     }
        // }

        // overflow.overflow.extend_from_slice(buf.split_at(bytes_read).0);
    }

}

pub async fn motion_read(stdin: &mut Pull, do_try: bool) -> MotionResult<IODataWrapper> {

    fn do_match_stuff(v: Vec<u8>) -> IODataWrapper {
        if v.is_empty() {
            return IODataWrapper::Finished;
        }
        IODataWrapper::IOData(IOData(v))
    }

    match (stdin, do_try) {
        (Pull::None, false) => Ok(IODataWrapper::Finished),
        (Pull::Mock(v), _) => MotionResult::Ok(v.pop_front().map(IODataWrapper::IOData).unwrap_or(IODataWrapper::Finished)),
        (Pull::Receiver(rd), false) => match rd.recv().await {
            Ok(d) => Ok(IODataWrapper::IOData(d)),
            Err(RecvError) => Ok(IODataWrapper::Finished)
        },
        (Pull::Receiver(rd), true) => match rd.try_recv() {
            Ok(d) => Ok(IODataWrapper::IOData(d)),
            Err(TryRecvError::Empty) => Ok(IODataWrapper::Skipped),
            Err(TryRecvError::Closed) => Ok(IODataWrapper::Finished),
        },
        (Pull::CmdStderr(rd, overflow), false) => motion_read_buffer(rd, overflow).await.map(do_match_stuff),
        (Pull::CmdStdout(rd, overflow), false) => motion_read_buffer(rd, overflow).await.map(do_match_stuff),
        (Pull::Stdin(rd, overflow), false) => motion_read_buffer(rd, overflow).await.map(do_match_stuff),
        (Pull::File(rd, overflow), false) => motion_read_buffer(rd, overflow).await.map(do_match_stuff),
        (_, true) => panic!("Only Pull::Receiver and Pull::Mock can do a motion_read with do_try")
    }
}


pub async fn motion_write(stdout: &mut Push, data: IOData) -> MotionResult<()> {
    match (stdout, data) {
        (Push::None, IOData(_data)) => MotionResult::Ok(()),
        (Push::IoMock(v), IOData(data)) => { v.push_back(IOData(data)); Ok(()) },
        (Push::Sender(wr), IOData(data)) => Ok(wr.send(IOData(data)).await?),
        (Push::IoSender(wr), IOData(data)) => Ok(wr.send(IOData(data)).await?),
        (Push::CmdStdin(wr), IOData(data)) => Ok(wr.write_all(&data).await?),
        (Push::Stdout(wr), IOData(data)) => Ok(wr.write_all(&data).await?),
        (Push::File(wr), IOData(data)) => Ok(wr.write_all(&data).await?),
        (Push::Stderr(wr), IOData(data)) => Ok(wr.write_all(&data).await?),
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
        Push::File(wr) => Ok(wr.flush().await?),
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

#[derive(Debug)]
pub struct MotionOneResult {
    pub finished_pulls: Vec<usize>,
    pub read_from: Vec<usize>,
    pub skipped: Vec<usize>,
}

pub async fn motion_one(pulls: &mut Vec<Pull>, monitor: &mut MotionNotifications, pushs: &mut Vec<Push>, do_try_read: bool) -> MotionResult<MotionOneResult> {
    let mut finished_pulls = vec![];
    let mut read_from = vec![];
    let mut skipped: Vec<usize> = vec![];
    for (pull_config_index, pull_config) in pulls.iter_mut().enumerate() {
        if finished_pulls.contains(&pull_config_index) { continue; }
        let data = motion_read(pull_config, do_try_read).await?;
        if data == IODataWrapper::Skipped {
            skipped.push(pull_config_index);
            continue;
        }
        read_from.push(pull_config_index);
        if data == IODataWrapper::Finished {
            finished_pulls.push(pull_config_index);
            continue;
        }
        match &monitor.read {
            Some(m) => {
                m.send(MonitorMessage::Read(pull_config_index)).await
            }
            None => {
                Ok(())
            }
        }?;
        let was_finished = data == IODataWrapper::Finished;

        for (index, push) in pushs.iter_mut().enumerate() {
            match data.clone() {
                IODataWrapper::Finished => motion_close(push).await,
                IODataWrapper::IOData(iodata) => motion_write(push, iodata).await,
                IODataWrapper::Skipped => MotionResult::Ok(()),
            }?;
            match (was_finished, &monitor.written) {
                (false, Some(m)) => {
                    m.send(MonitorMessage::Wrote(index)).await
                },
                _ => {
                    Ok(())
                }
            }?;
        }

    }
    MotionResult::Ok(MotionOneResult { finished_pulls, read_from, skipped })
}


pub async fn motion(mut pulls: Vec<Pull>, mut monitor: MotionNotifications, mut pushs: Vec<Push>) -> MotionResult<usize> {
    let mut read_count = 0;

    loop {
        if pulls.is_empty() {
            for push in pushs.iter_mut() {
                motion_close(push).await?;
            }
            monitor.read.map(|m| m.close());
            monitor.written.map(|m| m.close());
            return MotionResult::Ok(read_count);
        }

        let motion_one_result  = motion_one(&mut pulls, &mut monitor, &mut pushs, false).await?;
        read_count += motion_one_result.read_from.len();

        for i in motion_one_result.finished_pulls.into_iter().rev() {
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
        source.push_back(IOData(vec![1]));
        source.push_back(IOData(vec![2]));
        let (s_snd0, s_rcv0) = bounded(1);
        let pull_config_1 = vec![Pull::Mock(source)];
        let push_config_1 = vec![Push::IoSender(s_snd0)];

        let motion1 = motion(
            pull_config_1,
            MotionNotifications::both(chan_0_read_snd, chan_0_writ_snd),
            push_config_1
        );

        // let (sndi1, rcvi1) = bounded(1);
        let pull_config_splitter = vec![
            Pull::Receiver(s_rcv0),
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

        let _expected_vecdequeue: VecDeque<IOData> = VecDeque::new();
        let mut stdout = vec![];
        let mut mock_stdout_pull_2_pull = Pull::Receiver(mock_stdout_pull_2);
        loop {
            let msg = motion_read(&mut mock_stdout_pull_2_pull, false).await.unwrap();
            if msg == IODataWrapper::Finished {
                stdout.push(msg);
                break;
            }
            stdout.push(msg);
        }
        assert_eq!(
            stdout,
            &[
                IODataWrapper::IOData(IOData(vec![1])),
                IODataWrapper::IOData(IOData(vec![1])),
                IODataWrapper::IOData(IOData(vec![2])),
                IODataWrapper::IOData(IOData(vec![2])),
                IODataWrapper::Finished,
            ]
        );

        assert_eq!(chan_0_read_rcv.recv().await.unwrap(), MonitorMessage::Read(0));
        assert_eq!(chan_0_read_rcv.recv().await.unwrap(), MonitorMessage::Read(0));
        assert_eq!(chan_0_read_rcv.is_closed(), true);

        assert_eq!(chan_0_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
        assert_eq!(chan_0_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
        assert_eq!(chan_0_writ_rcv.is_closed(), true);

        assert_eq!(chan_1_read_rcv.recv().await.unwrap(), MonitorMessage::Read(0));
        assert_eq!(chan_1_read_rcv.recv().await.unwrap(), MonitorMessage::Read(0));
        assert_eq!(chan_1_read_rcv.is_closed(), true);

        assert_eq!(chan_1_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
        assert_eq!(chan_1_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(1));
        assert_eq!(chan_1_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
        assert_eq!(chan_1_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(1));
        assert_eq!(chan_1_writ_rcv.is_closed(), true);

        assert_eq!(chan_2_read_rcv.recv().await.unwrap(), MonitorMessage::Read(0));
        assert_eq!(chan_2_read_rcv.recv().await.unwrap(), MonitorMessage::Read(1));
        assert_eq!(chan_2_read_rcv.recv().await.unwrap(), MonitorMessage::Read(0));
        assert_eq!(chan_2_read_rcv.recv().await.unwrap(), MonitorMessage::Read(1));
        assert_eq!(chan_2_read_rcv.is_closed(), true);

        assert_eq!(chan_2_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
        assert_eq!(chan_2_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
        assert_eq!(chan_2_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
        assert_eq!(chan_2_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
        assert_eq!(chan_2_read_rcv.is_closed(), true);

        f
    }

    println!("R: {:?}", async_std::task::block_on(test_motion_impl()));
}

#[test]
fn test_motion_read_buffer() {
use crate::fake_read::FakeReader;

    use async_std::task;

    async fn test_motion_read_buffer_impl() {

        let mut fake_reader = FakeReader::new_by_size("Hows you?\r\nGreat, I had big lunch!\nWow!\nYes!".to_string(), 16);
        let mut overflow = ReadSplitControl { split_at: vec![vec![13, 10]], overflow: vec![]};

        let data_1 = motion_read_buffer(&mut fake_reader, &mut overflow).await;
        println!("data_1: {:?}", data_1);
        assert_eq!(
            data_1,
            Ok("Hows you?\r\n".as_bytes().iter().copied().collect())
        );
        assert_eq!(
            overflow.overflow,
            "Great".as_bytes().iter().copied().collect::<Vec<u8>>()
        );

        println!("===========================");
        overflow.split_at = vec![vec![13, 10], vec![10]];
        let data_2 = motion_read_buffer(&mut fake_reader, &mut overflow).await;
        println!("data_1: {:?}", data_1);
        assert_eq!(
            data_2,
            Ok("Great, I had big lunch!\n".as_bytes().iter().copied().collect())
        );
        assert_eq!(
            overflow.overflow,
            "Wow!\nYes!".as_bytes().iter().copied().collect::<Vec<u8>>()
        );

        println!("===========================");
        let data_3 = motion_read_buffer(&mut fake_reader, &mut overflow).await;
        println!("data_1: {:?}", data_1);
        assert_eq!(
            data_3,
            Ok("Wow!\n".as_bytes().iter().copied().collect())
        );
        assert_eq!(
            overflow.overflow,
            "Yes!".as_bytes().iter().copied().collect::<Vec<u8>>()
        );

        println!("===========================");
        let data_4 = motion_read_buffer(&mut fake_reader, &mut overflow).await;
        println!("data_1: {:?}", data_1);
        assert_eq!(
            data_4,
            Ok("Yes!".as_bytes().iter().copied().collect())
        );
        assert!(overflow.overflow.is_empty());

        println!("===========================");
        let data_5 = motion_read_buffer(&mut fake_reader, &mut overflow).await;
        println!("data_1: {:?}", data_1);
        assert!(data_5.unwrap().is_empty());
        assert!(overflow.overflow.is_empty());

    }

    task::block_on(test_motion_read_buffer_impl());

}

#[test]
fn test_is_split() {
    assert_eq!(is_split("hello".as_bytes(), &vec![vec!['h' as u8]]), Some(1));
    assert_eq!(is_split("hello".as_bytes(), &vec!["he".as_bytes().iter().copied().collect()]), Some(2));
    assert_eq!(is_split("hello".as_bytes(), &vec!["hello".as_bytes().iter().copied().collect()]), Some(5));
    assert_eq!(is_split("hello".as_bytes(), &vec!["hello bob".as_bytes().iter().copied().collect()]), None);
}

