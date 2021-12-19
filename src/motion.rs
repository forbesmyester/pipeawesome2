// use types::{Pull, MotionResult, IOData};
use std::time::Instant;
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

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Journey {
    pub src: usize,
    pub dst: usize,
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct JourneySource {
    pub src: usize,
}

#[derive(Debug)]
pub enum Pull {
    CmdStdout(Journey, aip::ChildStdout, ReadSplitControl),
    CmdStderr(Journey, aip::ChildStderr, ReadSplitControl),
    Stdin(Journey, aio::Stdin, ReadSplitControl),
    File(Journey, async_std::fs::File, ReadSplitControl),
    Receiver(Journey, Receiver<IOData>),
    Mock(Journey, VecDeque<IOData>),
    None,
}


impl Pull {
    pub fn journey(&self) -> Option<&Journey> {
        match self {
            Pull::CmdStdout(j, ..) => Some(j),
            Pull::CmdStderr(j, ..) => Some(j),
            Pull::Stdin(j, ..) => Some(j),
            Pull::File(j, ..) => Some(j),
            Pull::Receiver(j, ..) => Some(j),
            Pull::Mock(j, ..) => Some(j),
            Pull::None => None
        }
    }
}


#[derive(Debug)]
pub enum Push {
    IoMock(Journey, VecDeque<IOData>),
    CmdStdin(Journey, aip::ChildStdin),
    Stdout(Journey, aio::Stdout),
    Stderr(Journey, aio::Stderr),
    File(Journey, aio::BufWriter<async_std::fs::File>),
    Sender(Journey, Sender<IOData>),
    IoSender(Journey, Sender<IOData>),
    None,
}

impl Push {
    pub fn journey(&self) -> Option<&Journey> {
        match self {
            Push::IoMock(j, ..) => Some(j),
            Push::CmdStdin(j, ..) => Some(j),
            Push::Stdout(j, ..) => Some(j),
            Push::Stderr(j, ..) => Some(j),
            Push::File(j, ..) => Some(j),
            Push::Sender(j, ..) => Some(j),
            Push::IoSender(j, ..) => Some(j),
            Push::None => None,
        }
    }

}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum MotionError {
    ReadIOError(Journey, Instant, std::io::Error),
    CloseIOError(Journey, Instant, std::io::Error),
    WriteIOError(Journey, Instant, std::io::Error, IOData),
    RecvError(Journey, Instant, RecvError),
    SendError(Journey, Instant, SendError<IOData>),
    OutputClosed(Journey, Instant, IOData),
    MonitorReadError(JourneySource, Instant, SendError<MonitorMessage>),
    MonitorWriteError(Journey, Instant, SendError<MonitorMessage>),
    NoneError,
}

impl std::fmt::Display for MotionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MotionError::ReadIOError(_, _, a) => write!(f, "{}", a),
            MotionError::CloseIOError(_, _, a) => write!(f, "{}", a),
            MotionError::WriteIOError(_, _, a, _) => write!(f, "{}", a),
            MotionError::RecvError(_, _, a) => write!(f, "{}", a),
            MotionError::SendError(_, _, a) => write!(f, "{}", a),
            MotionError::OutputClosed(_, _, _) => write!(f, "Output Closed"),
            MotionError::MonitorReadError(_, _, a) => write!(f, "{}", a),
            MotionError::MonitorWriteError(_, _, a) => write!(f, "{}", a),
            MotionError::NoneError => write!(f, "None Error"),
        }
    }
}

impl MotionError {
    pub fn instant(&self) -> Option<&Instant> {
        match self {
            MotionError::ReadIOError(_, i, ..) => Some(i),
            MotionError::CloseIOError(_, i, ..) => Some(i),
            MotionError::WriteIOError(_, i, ..) => Some(i),
            MotionError::RecvError(_, i, ..) => Some(i),
            MotionError::SendError(_, i, ..) => Some(i),
            MotionError::OutputClosed(_, i, ..) => Some(i),
            MotionError::MonitorReadError(_, i, ..) => Some(i),
            MotionError::MonitorWriteError(_, i, ..) => Some(i),
            MotionError::NoneError => None
        }
    }

    pub fn journey(&self) -> Option<&Journey> {
        match self {
            MotionError::ReadIOError(j, ..) => Some(j),
            MotionError::CloseIOError(j, ..) => Some(j),
            MotionError::WriteIOError(j, ..) => Some(j),
            MotionError::RecvError(j, ..) => Some(j),
            MotionError::SendError(j, ..) => Some(j),
            MotionError::OutputClosed(j, ..) => Some(j),
            MotionError::MonitorReadError(..) => None,
            MotionError::MonitorWriteError(j, ..) => Some(j),
            MotionError::NoneError => None
        }
    }

    pub fn journey_source(&self) -> Option<&usize> {
        match self {
            MotionError::ReadIOError(Journey { src, .. }, ..) => Some(src),
            MotionError::CloseIOError(Journey { src, .. }, ..) => Some(src),
            MotionError::WriteIOError(Journey { src, .. }, ..) => Some(src),
            MotionError::RecvError(Journey { src, .. }, ..) => Some(src),
            MotionError::SendError(Journey { src, .. }, ..) => Some(src),
            MotionError::OutputClosed(Journey { src, .. }, ..) => Some(src),
            MotionError::MonitorReadError(JourneySource { src }, ..) => Some(src),
            MotionError::MonitorWriteError(Journey { src, .. }, ..) => Some(src),
            MotionError::NoneError => None
        }
    }
}


impl PartialEq for MotionError {
    fn eq(&self, b: &MotionError) -> bool {
        match (&self, b) {
            (MotionError::RecvError(j, _, a), MotionError::RecvError(j2, _, a2)) => (a == a2) && (j == j2),
            (MotionError::WriteIOError(j, _, _a, b), MotionError::WriteIOError(j2, _, _a2, b2)) => (b == b2) && (j == j2),
            (MotionError::ReadIOError(j, _, _a), MotionError::ReadIOError(j2, _, _a2)) => (j == j2),
            (MotionError::SendError(j, _, a), MotionError::SendError(j2, _, a2)) => (j == j2) && (a == a2),
            (MotionError::MonitorReadError(j, _, a), MotionError::MonitorReadError(j2, _, a2)) => (j == j2) && (a == a2),
            (MotionError::MonitorWriteError(j, _, a), MotionError::MonitorWriteError(j2, _, a2)) => (j == j2) && (a == a2),
            (MotionError::NoneError, MotionError::NoneError) => true,
            _ => false,
        }
    }
}

pub type MotionResult<T> = Result<T, MotionError>;

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

fn is_closed(push: &Push) -> bool {
    match push {
        Push::IoSender(_id, s) => s.is_closed(),
        Push::Sender(_id, s) => s.is_closed(),
        _ => false,
    }
}

async fn motion_read_buffer(rd: &mut (dyn AsyncRead + Unpin + Send), overflow: &mut ReadSplitControl) -> Result<Vec<u8>, std::io::Error> {

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

    async fn motion_read_buffer_wrapper(rd: &mut (dyn AsyncRead + Unpin + Send), overflow: &mut ReadSplitControl, j: Journey) -> MotionResult<IODataWrapper> {
        motion_read_buffer(rd, overflow).await.map(do_match_stuff).map_err(|e| MotionError::ReadIOError(j, Instant::now(), e))
    }

    // let id: Option<usize> = match &stdin {
    //     Pull::None => None,
    //     Pull::Mock(id, ..) => Some(*id),
    //     Pull::Receiver(id, ..) => Some(*id),
    //     Pull::CmdStderr(id, ..) => Some(*id),
    //     Pull::CmdStdout(id, ..) => Some(*id),
    //     Pull::Stdin(id, ..) => Some(*id),
    //     Pull::File(id, ..) => Some(*id),
    // };

    match (stdin, do_try) {
        (Pull::None, false) => Ok(IODataWrapper::Finished),
        (Pull::Mock(j, v), _) => Ok(v.pop_front().map(IODataWrapper::IOData).unwrap_or(IODataWrapper::Finished)),
        (Pull::Receiver(j, rd), false) => match rd.recv().await {
            Ok(d) => Ok(IODataWrapper::IOData(d)),
            Err(RecvError) => Ok(IODataWrapper::Finished)
        },
        (Pull::Receiver(j, rd), true) => match rd.try_recv() {
            Ok(d) => Ok(IODataWrapper::IOData(d)),
            Err(TryRecvError::Empty) => Ok(IODataWrapper::Skipped),
            Err(TryRecvError::Closed) => Ok(IODataWrapper::Finished),
        },
        (Pull::CmdStderr(j, rd, overflow), false) => motion_read_buffer_wrapper(rd, overflow, *j).await,
        (Pull::CmdStdout(j, rd, overflow), false) => motion_read_buffer_wrapper(rd, overflow, *j).await,
        (Pull::Stdin(j, rd, overflow), false) => motion_read_buffer_wrapper(rd, overflow, *j).await,
        (Pull::File(j, rd, overflow), false) => motion_read_buffer_wrapper(rd, overflow, *j).await,
        (_, true) => panic!("Only Pull::Receiver and Pull::Mock can do a motion_read with do_try")
    }
    // match &out {
    //     Ok(IODataWrapper::IOData(o)) => { println!(
    //         "motion_read({:?}, {:?}) - iodata",
    //         id,
    //         String::from_utf8_lossy(&o.0)
    //     ) },
    //     Ok(IODataWrapper::Finished) => { println!("motion_read() - finished") }
    //     Ok(IODataWrapper::Skipped) => { println!("motion_read() - skipped") }
    //     Err(e) => { println!("motion_read({:?}) - error", e) }
    // };
}


pub async fn motion_write(stdout: &mut Push, data: IOData) -> MotionResult<()> {

    if is_closed(stdout) {
        if let Some(j) = stdout.journey() {
            return MotionResult::Err(MotionError::OutputClosed(*j, Instant::now(), data));
        }
        panic!("The {:?} is closed, but does not have a Journey", stdout);
    }

    fn e_map_io(j: Journey, x: std::io::Error, d: IOData) -> MotionError {
        MotionError::WriteIOError(j, Instant::now(), x, d)
    }

    fn e_map_chan(j: Journey, e: SendError<IOData>) -> MotionError {
        MotionError::SendError(j, Instant::now(), e)
    }

    match (stdout, data) {
        (Push::None, IOData(_data)) => MotionResult::Ok(()),
        (Push::IoMock(_, v), IOData(data)) => { v.push_back(IOData(data)); Ok(()) },
        (Push::Sender(j, wr), IOData(data)) => Ok(wr.send(IOData(data)).await.map_err(|e| e_map_chan(*j, e))?),
        (Push::IoSender(j, wr), IOData(data)) => Ok(wr.send(IOData(data)).await.map_err(|e| e_map_chan(*j, e))?),
        (Push::CmdStdin(j, wr), IOData(data)) => Ok(wr.write_all(&data).await.map_err(|e| e_map_io(*j, e, IOData(data)))?),
        (Push::Stdout(j, wr), IOData(data)) => Ok(wr.write_all(&data).await.map_err(|e| e_map_io(*j, e, IOData(data)))?),
        (Push::File(j, wr), IOData(data)) => Ok(wr.write_all(&data).await.map_err(|e| e_map_io(*j, e, IOData(data)))?),
        (Push::Stderr(j, wr), IOData(data)) => Ok(wr.write_all(&data).await.map_err(|e| e_map_io(*j, e, IOData(data)))?),
    }
}


pub async fn motion_close(stdout: &mut Push) -> MotionResult<()> {
    // println!("motion_close({:?})", stdout);

    fn e_map(j: Journey, x: std::io::Error) -> MotionError {
        MotionError::CloseIOError(j, Instant::now(), x)
    }

    match stdout {
        Push::None => MotionResult::Ok(()),
        Push::IoMock(_, _v) => MotionResult::Ok(()),
        Push::Sender(_, wr) => { wr.close(); Ok(()) },
        Push::IoSender(_, wr) => { wr.close(); Ok(()) },
        Push::CmdStdin(j, wr) => Ok(wr.flush().await.map_err(|e| e_map(*j, e))?),
        Push::Stdout(j, wr) => Ok(wr.flush().await.map_err(|e| e_map(*j, e))?),
        Push::Stderr(j, wr) => Ok(wr.flush().await.map_err(|e| e_map(*j, e))?),
        Push::File(j, wr) => Ok(wr.flush().await.map_err(|e| e_map(*j, e))?),
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

pub async fn motion_worker(pulls: &mut Vec<Pull>, monitor: &mut MotionNotifications, pushs: &mut Vec<Push>, do_try_read: bool) -> MotionResult<MotionOneResult> {

    fn e_map_chan_read(j: Journey, e: SendError<IOData>) -> MotionError {
        MotionError::SendError(j, Instant::now(), e)
    }

    let mut finished_pulls = vec![];
    let mut read_from = vec![];
    let mut skipped: Vec<usize> = vec![];

    for (pull_index, pull) in pulls.iter_mut().enumerate() {

        // If we can finished reading that particular pull
        if finished_pulls.contains(&pull_index) { continue; }

        // Try read and do housekeeping
        let data = motion_read(pull, do_try_read).await?;
        if data == IODataWrapper::Skipped {
            skipped.push(pull_index);
            continue;
        }
        read_from.push(pull_index);
        if data == IODataWrapper::Finished {
            finished_pulls.push(pull_index);
            continue;
        }
        match &monitor.read {
            Some(m) => {
                m.send(MonitorMessage::Read(pull_index)).await
            }
            None => {
                Ok(())
            }
        }.map_err(|e| { MotionError::MonitorReadError( JourneySource { src: (*pull.journey().expect(&format!("motion::motion_worker monitor.read for pull {:?} had no journey", pull))).src }, Instant::now(), e ) })?;
        let was_finished = data == IODataWrapper::Finished;

        for (push_index, push) in pushs.iter_mut().enumerate() {
            match data.clone() {
                IODataWrapper::Finished => { panic!("If we find an IODataWrapper::Finished we `continue` before we get here"); },
                IODataWrapper::IOData(iodata) => motion_write(push, iodata).await,
                IODataWrapper::Skipped => MotionResult::Ok(()),
            }?;
            match (was_finished, &monitor.written) {
                (false, Some(m)) => {
                    m.send(MonitorMessage::Wrote(push_index)).await
                },
                _ => {
                    Ok(())
                }
            }.map_err(|e| { MotionError::MonitorWriteError( *pull.journey().expect(&format!("motion::motion_worker monitor.read for pull {:?} had no journey", pull)), Instant::now(), e ) })?;
        }
    }
    MotionResult::Ok(MotionOneResult { finished_pulls, read_from, skipped })
}


pub async fn motion(pull: Pull, mut monitor: MotionNotifications, push: Push) -> MotionResult<usize> {
    let mut read_count = 0;
    let mut pulls = vec![pull];
    let mut pushs = vec![push];

    loop {
        if pulls.is_empty() {
            for push in pushs.iter_mut() {
                motion_close(push).await?;
            }
            monitor.read.map(|m| m.close());
            monitor.written.map(|m| m.close());
            return MotionResult::Ok(read_count);
        }

        let motion_one_result = motion_worker(&mut pulls, &mut monitor, &mut pushs, false).await?;
        read_count += motion_one_result.read_from.len();

        for i in motion_one_result.finished_pulls.into_iter().rev() {
            pulls.remove(i);
        }
    }

}


// pub async fn motion_for_launch(pulls: &mut Vec<Pull>, push: Push) -> MotionResult<usize> {
//     let mut read_count = 0;
//     let mut pushs = vec![push];
//     let mut monitor = MotionNotifications::empty();
// 
//     loop {
//         if pulls.is_empty() {
//             for push in pushs.iter_mut() {
//                 motion_close(push).await?;
//             }
//             return MotionResult::Ok(read_count);
//         }
// 
//         let motion_one_result = motion_worker(pulls, &mut monitor, &mut pushs, false).await?;
//         read_count += motion_one_result.read_from.len();
// 
//     }
// 
// }


// #[test]
// fn test_motion_one() {
// 
//     use async_std::channel::bounded;
// 
//     async fn test_motion_one_impl() -> MotionResult<Vec<u8>> {
// 
//         let mut source_1 = VecDeque::new();
//         for i in 90..96 {
//             source_1.push_back(IOData(vec![i]));
//         }
// 
//         let mut source_2 = VecDeque::new();
//         for i in 20..81 {
//             source_2.push_back(IOData(vec![i]));
//         }
// 
//         let (output_send, output_read) = bounded(128);
// 
//         let pull_config = vec![Pull::Mock(0, source_1), Pull::Mock(0, source_2)];
// 
//         motion(pull_config, MotionNotifications::empty(), vec![Push::Sender(1, output_send)]).await?;
// 
//         let mut stdout = vec![];
//         let mut mock_stdout_pull_2_pull = Pull::Receiver(0, output_read);
// 
//         loop {
//             let msg = motion_read(&mut mock_stdout_pull_2_pull, false).await.unwrap();
//             match msg {
//                 IODataWrapper::IOData(IOData(mut d)) => {
//                     stdout.append(&mut d);
//                 }
//                 _ => {
//                     break;
//                 }
//             }
//         }
// 
//         stdout.sort();
//         MotionResult::Ok(stdout)
// 
//     }
// 
// 
//     let mut expected_result = vec![];
//     for i in 20..81 { expected_result.push(i); }
//     for i in 90..96 { expected_result.push(i); }
// 
//     let r = async_std::task::block_on(test_motion_one_impl());
// 
//     assert_eq!(r.is_ok(), true);
//     assert_eq!(r.unwrap(), expected_result);
// }

// #[test]
// fn test_motion() {
// 
//     use async_std::channel::{ bounded, unbounded };
// 
//     async fn test_motion_impl() -> ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>) {
// 
//         // source -->> s_snd0 -> s_rcv0 -->> snd1 -> rcv1 -->> mock_stdout_push_2 -> mock_stdout_pull_2
//         //                              -->> snd2 -> rcv1 ⇗
// 
//         let (chan_0_read_snd, chan_0_read_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
//         let (chan_0_writ_snd, chan_0_writ_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
//         let (chan_1_read_snd, chan_1_read_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
//         let (chan_1_writ_snd, chan_1_writ_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
//         let (chan_2_read_snd, chan_2_read_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
//         let (chan_2_writ_snd, chan_2_writ_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
// 
//         let mut source = VecDeque::new();
//         source.push_back(IOData(vec![1]));
//         source.push_back(IOData(vec![2]));
//         let (s_snd0, s_rcv0) = bounded(1);
//         let pull_config_1 = vec![Pull::Mock(0, source)];
//         let push_config_1 = vec![Push::IoSender(0, s_snd0)];
// 
//         let motion1 = motion(
//             pull_config_1,
//             MotionNotifications::both(chan_0_read_snd, chan_0_writ_snd),
//             push_config_1
//         );
// 
//         // let (sndi1, rcvi1) = bounded(1);
//         let pull_config_splitter = vec![
//             Pull::Receiver(0, s_rcv0),
//         ];
//         let (snd1, rcv1) = unbounded();
//         let (snd2, rcv2) = unbounded();
//         let push_config_splitter = vec![Push::Sender(0, snd1), Push::Sender(0, snd2)];
//         let motion2 = motion(
//             pull_config_splitter,
//             MotionNotifications::both(chan_1_read_snd, chan_1_writ_snd),
//             push_config_splitter
//         );
// 
//         let joiner_pull_configs = vec![
//             Pull::Receiver(0, rcv1),
//             Pull::Receiver(0, rcv2),
//         ];
//         let (mock_stdout_push_2, mock_stdout_pull_2)  = bounded(8);
//         let motion3 = motion(
//             joiner_pull_configs,
//             MotionNotifications::both(chan_2_read_snd, chan_2_writ_snd),
//             vec![Push::Sender(0, mock_stdout_push_2)]
//         );
// 
//         let f: ((MotionResult<usize>, MotionResult<usize>), MotionResult<usize>) = motion1.join(motion2).join(motion3).await;
// 
//         let _expected_vecdequeue: VecDeque<IOData> = VecDeque::new();
//         let mut stdout = vec![];
//         let mut mock_stdout_pull_2_pull = Pull::Receiver(0, mock_stdout_pull_2);
//         loop {
//             let msg = motion_read(&mut mock_stdout_pull_2_pull, false).await.unwrap();
//             if msg == IODataWrapper::Finished {
//                 stdout.push(msg);
//                 break;
//             }
//             stdout.push(msg);
//         }
//         assert_eq!(
//             stdout,
//             &[
//                 IODataWrapper::IOData(IOData(vec![1])),
//                 IODataWrapper::IOData(IOData(vec![1])),
//                 IODataWrapper::IOData(IOData(vec![2])),
//                 IODataWrapper::IOData(IOData(vec![2])),
//                 IODataWrapper::Finished,
//             ]
//         );
// 
//         assert_eq!(chan_0_read_rcv.recv().await.unwrap(), MonitorMessage::Read(0));
//         assert_eq!(chan_0_read_rcv.recv().await.unwrap(), MonitorMessage::Read(0));
//         assert_eq!(chan_0_read_rcv.is_closed(), true);
// 
//         assert_eq!(chan_0_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
//         assert_eq!(chan_0_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
//         assert_eq!(chan_0_writ_rcv.is_closed(), true);
// 
//         assert_eq!(chan_1_read_rcv.recv().await.unwrap(), MonitorMessage::Read(0));
//         assert_eq!(chan_1_read_rcv.recv().await.unwrap(), MonitorMessage::Read(0));
//         assert_eq!(chan_1_read_rcv.is_closed(), true);
// 
//         assert_eq!(chan_1_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
//         assert_eq!(chan_1_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(1));
//         assert_eq!(chan_1_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
//         assert_eq!(chan_1_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(1));
//         assert_eq!(chan_1_writ_rcv.is_closed(), true);
// 
//         assert_eq!(chan_2_read_rcv.recv().await.unwrap(), MonitorMessage::Read(0));
//         assert_eq!(chan_2_read_rcv.recv().await.unwrap(), MonitorMessage::Read(1));
//         assert_eq!(chan_2_read_rcv.recv().await.unwrap(), MonitorMessage::Read(0));
//         assert_eq!(chan_2_read_rcv.recv().await.unwrap(), MonitorMessage::Read(1));
//         assert_eq!(chan_2_read_rcv.is_closed(), true);
// 
//         assert_eq!(chan_2_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
//         assert_eq!(chan_2_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
//         assert_eq!(chan_2_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
//         assert_eq!(chan_2_writ_rcv.recv().await.unwrap(), MonitorMessage::Wrote(0));
//         assert_eq!(chan_2_read_rcv.is_closed(), true);
// 
//         f
//     }
// 
//     println!("R: {:?}", async_std::task::block_on(test_motion_impl()));
// }

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
            data_1.unwrap(),
            "Hows you?\r\n".as_bytes().iter().copied().collect::<Vec<u8>>()
        );
        assert_eq!(
            overflow.overflow,
            "Great".as_bytes().iter().copied().collect::<Vec<u8>>()
        );

        println!("===========================");
        overflow.split_at = vec![vec![13, 10], vec![10]];
        let data_2 = motion_read_buffer(&mut fake_reader, &mut overflow).await;
        assert_eq!(
            data_2.unwrap(),
            "Great, I had big lunch!\n".as_bytes().iter().copied().collect::<Vec<u8>>()
        );
        assert_eq!(
            overflow.overflow,
            "Wow!\nYes!".as_bytes().iter().copied().collect::<Vec<u8>>()
        );

        println!("===========================");
        let data_3 = motion_read_buffer(&mut fake_reader, &mut overflow).await;
        assert_eq!(
            data_3.unwrap(),
            "Wow!\n".as_bytes().iter().copied().collect::<Vec<u8>>()
        );
        assert_eq!(
            overflow.overflow,
            "Yes!".as_bytes().iter().copied().collect::<Vec<u8>>()
        );

        println!("===========================");
        let data_4 = motion_read_buffer(&mut fake_reader, &mut overflow).await;
        assert_eq!(
            data_4.unwrap(),
            "Yes!".as_bytes().iter().copied().collect::<Vec<u8>>()
        );
        assert!(overflow.overflow.is_empty());

        println!("===========================");
        let data_5 = motion_read_buffer(&mut fake_reader, &mut overflow).await;
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

