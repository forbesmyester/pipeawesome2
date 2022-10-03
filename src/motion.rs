// use types::{Pull, MotionResult, IOData};
use crate::connectable::Breakable;
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
        // TODO: Make the split configurable.
        ReadSplitControl {
            split_at: vec![
                "\r\n".as_bytes().iter().copied().collect(),
                "\n".as_bytes().iter().copied().collect()
            ],
            overflow: vec![]
        }
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
    pub breakable: Breakable,
}

impl Journey {
    pub fn as_pull_journey(&self) -> PullJourney {
        PullJourney { src: self.src, dst: self.dst }
    }
}

impl PullJourney {
    pub fn as_journey(&self, breakable: Breakable) -> Journey {
        Journey { src: self.src, dst: self.dst, breakable }
    }
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct PullJourney {
    pub src: usize,
    pub dst: usize,
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct JourneySource {
    pub src: usize,
}

#[derive(Debug)]
pub enum Pull {
    CmdStdout(PullJourney, aip::ChildStdout, ReadSplitControl),
    CmdStderr(PullJourney, aip::ChildStderr, ReadSplitControl),
    Stdin(PullJourney, aio::Stdin, ReadSplitControl),
    File(PullJourney, async_std::fs::File, ReadSplitControl),
    Receiver(PullJourney, Receiver<IOData>),
    Mock(PullJourney, VecDeque<IOData>),
    None,
}


impl Pull {
    pub fn journey(&self) -> Option<&PullJourney> {
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
pub enum OutputClosedReason {
    BrokenPipe,
    ChannelClosed
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum MotionError {
    ReadIOError(PullJourney, Instant, std::io::Error),
    CloseIOError(Journey, Instant, std::io::Error),
    OpenIOError(PullJourney, Instant, std::io::Error),
    WriteIOError(Journey, Instant, std::io::Error, IOData),
    RecvError(Journey, Instant, RecvError),
    SendError(Journey, Instant, SendError<IOData>),
    OutputClosed(Journey, Instant, OutputClosedReason, IOData),
    MonitorReadError(JourneySource, Instant, SendError<MonitorMessage>),
    MonitorWriteError(PullJourney, Instant, SendError<MonitorMessage>),
    LaunchSpawnError(Option<String>),
    NoneError,
}

impl std::fmt::Display for MotionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MotionError::ReadIOError(_, _, a) => write!(f, "MotionError::ReadIOError: {}", a),
            MotionError::CloseIOError(_, _, a) => write!(f, "MotionError::CloseIOError: {}", a),
            MotionError::OpenIOError(_, _, a) => write!(f, "MotionError::OpenIOError: {}", a),
            MotionError::WriteIOError(_, _, a, _) => write!(f, "MotionError::WriteIOError: {}", a),
            MotionError::RecvError(_, _, a) => write!(f, "MotionError::RecvError: {}", a),
            MotionError::SendError(_, _, a) => write!(f, "MotionError::SendError: {}", a),
            MotionError::OutputClosed(_, _, OutputClosedReason::BrokenPipe, _) => write!(f, "MotionError::OutputClosed: BrokenPipe"),
            MotionError::OutputClosed(_, _, OutputClosedReason::ChannelClosed, _) => write!(f, "MotionError::OutputClosed: ChannelClosed"),
            MotionError::MonitorReadError(_, _, a) => write!(f, "MotionError::MonitorReadError: {}", a),
            MotionError::MonitorWriteError(_, _, a) => write!(f, "MotionError::MonitorWriteError: {}", a),
            MotionError::LaunchSpawnError(cmd) => {
                match cmd {
                    Some(cmd) => write!(f, "MotionError::LaunchSpawnError: Could not run program {}", cmd),
                    None => write!(f, "MotionError::LaunchSpawnError: Could not run program UNKNOWN (could not represent in UTF-8)"),
                }
            }
            MotionError::NoneError => write!(f, "MotionError::NoneError: None Error"),
        }
    }
}

impl MotionError {
    pub fn instant(&self) -> Option<&Instant> {
        match self {
            MotionError::ReadIOError(_, i, ..) => Some(i),
            MotionError::CloseIOError(_, i, ..) => Some(i),
            MotionError::OpenIOError(_, i, ..) => Some(i),
            MotionError::WriteIOError(_, i, ..) => Some(i),
            MotionError::RecvError(_, i, ..) => Some(i),
            MotionError::SendError(_, i, ..) => Some(i),
            MotionError::OutputClosed(_, i, ..) => Some(i),
            MotionError::MonitorReadError(_, i, ..) => Some(i),
            MotionError::MonitorWriteError(_, i, ..) => Some(i),
            MotionError::LaunchSpawnError(_) => None,
            MotionError::NoneError => None
        }
    }

    pub fn journey(&self) -> Option<&Journey> {
        match self {
            MotionError::ReadIOError(_j, ..) => None,
            MotionError::CloseIOError(j, ..) => Some(j),
            MotionError::OpenIOError(_j, ..) => None,
            MotionError::WriteIOError(j, ..) => Some(j),
            MotionError::RecvError(j, ..) => Some(j),
            MotionError::SendError(j, ..) => Some(j),
            MotionError::OutputClosed(j, ..) => Some(j),
            MotionError::MonitorReadError(..) => None,
            MotionError::MonitorWriteError(_j, ..) => None,
            MotionError::LaunchSpawnError(_) => None,
            MotionError::NoneError => None
        }
    }

    pub fn pull_journey(&self) -> Option<&PullJourney> {
        match self {
            MotionError::ReadIOError(j, ..) => Some(j),
            MotionError::CloseIOError(_j, ..) => None,
            MotionError::OpenIOError(j, ..) => Some(j),
            MotionError::WriteIOError(_j, ..) => None,
            MotionError::RecvError(_j, ..) => None,
            MotionError::SendError(_j, ..) => None,
            MotionError::OutputClosed(_j, ..) => None,
            MotionError::MonitorReadError(..) => None,
            MotionError::MonitorWriteError(j, ..) => Some(j),
            MotionError::LaunchSpawnError(_) => None,
            MotionError::NoneError => None
        }
    }

    pub fn journey_source(&self) -> Option<&usize> {
        match self {
            MotionError::ReadIOError(PullJourney { src, .. }, ..) => Some(src),
            MotionError::CloseIOError(Journey { src, .. }, ..) => Some(src),
            MotionError::OpenIOError(PullJourney { src, .. }, ..) => Some(src),
            MotionError::WriteIOError(Journey { src, .. }, ..) => Some(src),
            MotionError::RecvError(Journey { src, .. }, ..) => Some(src),
            MotionError::SendError(Journey { src, .. }, ..) => Some(src),
            MotionError::OutputClosed(Journey { src, .. }, ..) => Some(src),
            MotionError::MonitorReadError(JourneySource { src }, ..) => Some(src),
            MotionError::MonitorWriteError(PullJourney { src, .. }, ..) => Some(src),
            MotionError::LaunchSpawnError(_) => None,
            MotionError::NoneError => None
        }
    }
}


impl PartialEq for MotionError {
    fn eq(&self, b: &MotionError) -> bool {
        match (&self, b) {
            (MotionError::RecvError(j, _, a), MotionError::RecvError(j2, _, a2)) => (a == a2) && (j == j2),
            (MotionError::WriteIOError(j, _, _a, b), MotionError::WriteIOError(j2, _, _a2, b2)) => (b == b2) && (j == j2),
            (MotionError::ReadIOError(j, _, _a), MotionError::ReadIOError(j2, _, _a2)) => j == j2,
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

async fn motion_read_buffer(rd: &mut (dyn AsyncRead + Unpin + Send), overflow: &mut ReadSplitControl) -> Result<Vec<u8>, std::io::Error> {
    let mut start_from = 0;

    loop {

        // It might already have been read into the buffer
        #[allow(clippy::mut_range_bound)]
        for overflow_pos in start_from..overflow.overflow.len() {
            start_from = overflow_pos;
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

    async fn motion_read_buffer_wrapper(rd: &mut (dyn AsyncRead + Unpin + Send), overflow: &mut ReadSplitControl, j: PullJourney) -> MotionResult<IODataWrapper> {
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
        (Pull::Mock(_j, v), _) => Ok(v.pop_front().map(IODataWrapper::IOData).unwrap_or(IODataWrapper::Finished)),
        (Pull::Receiver(_j, rd), false) => match rd.recv().await {
            Ok(d) => Ok(IODataWrapper::IOData(d)),
            Err(RecvError) => Ok(IODataWrapper::Finished)
        },
        (Pull::Receiver(_j, rd), true) => match rd.try_recv() {
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

    fn e_map_io(j: Journey, x: std::io::Error, d: IOData) -> MotionError {
        match x.kind() {
            std::io::ErrorKind::BrokenPipe => MotionError::OutputClosed(j, Instant::now(), OutputClosedReason::BrokenPipe, d),
            _ => MotionError::WriteIOError(j, Instant::now(), x, d)
        }
    }

    fn e_map_chan(j: Journey, e: SendError<IOData>) -> MotionError {
        MotionError::OutputClosed(j, Instant::now(), OutputClosedReason::ChannelClosed, e.0)
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
}

#[derive(Debug, PartialEq)]
pub struct MotionWorkerResult {
    pub finished: bool,
    pub read: bool,
}

pub async fn motion_worker(pull: &mut Pull, monitor: &mut MotionNotifications, pushs: &mut Vec<Push>, do_try_read: bool) -> MotionResult<MotionWorkerResult> {

    let data = motion_read(pull, do_try_read).await?;
    if data == IODataWrapper::Skipped {
        return MotionResult::Ok(MotionWorkerResult { finished: false, read: false })
    }

    match &monitor.read {
        Some(m) => m.send(MonitorMessage::Read(0)).await,
        None => Ok(()),
    }.map_err(|e| {
        MotionError::MonitorReadError(
            JourneySource {
                src: (
                    *pull.journey().unwrap_or_else(|| panic!("motion::motion_worker monitor.read for pull {:?} had no journey", pull))
                    ).src
            },
            Instant::now(),
            e
        )
    })?;

    let was_finished = data == IODataWrapper::Finished;

    let mut unfinished_push_count = pushs.len();

    for push in pushs {
        match (push.journey().map(|j| j.breakable).unwrap_or(Breakable::Terminate), data.clone()) {
            (_, IODataWrapper::Finished) => Ok(()),
            (_, IODataWrapper::Skipped) => MotionResult::Ok(()),
            (Breakable::Terminate, IODataWrapper::IOData(iodata)) => motion_write(push, iodata).await,
            (breakable, IODataWrapper::IOData(iodata)) => {
                match motion_write(push, iodata).await {
                    Err(MotionError::OutputClosed(_, _, _, _)) => {
                        if breakable == Breakable::Finish {
                            unfinished_push_count -= 1;
                        }
                        Ok(())
                    },
                    x => x,
                }
            },
        }?;
    }

    if let Some(m) = monitor.written.as_ref() {
        m.send(MonitorMessage::Wrote(0)).await
            .map_err(|e| { MotionError::MonitorWriteError(
                *pull.journey().unwrap_or_else(|| panic!("motion::motion_worker monitor.read for pull {:?} had no journey", pull)),
                Instant::now(),
                e
            ) })?;
    }

    if unfinished_push_count == 0 {
        return MotionResult::Ok(MotionWorkerResult { finished: true, read: true })
    }

    MotionResult::Ok(MotionWorkerResult { finished: was_finished, read: true })
}


pub async fn motion(mut pull: Pull, mut monitor: MotionNotifications, push: Push) -> MotionResult<usize> {
    let mut read_count = 0;
    let mut pushs = vec![push];

    loop {

        let motion_one_result = motion_worker(&mut pull, &mut monitor, &mut pushs, false).await?;
        if motion_one_result.read {
            read_count += 1;
        }

        if motion_one_result.finished {
            for mut push in pushs {
                motion_close(&mut push).await?;
            }
            monitor.read.map(|m| m.close());
            monitor.written.map(|m| m.close());
            return MotionResult::Ok(read_count);
        }
    }

}


#[test]
fn test_motion_worker_output_closed_unbreakable() {

    use async_std::channel::bounded;

    struct TestMotionWorker {
        data_5: Vec<IODataWrapper>,
        data_6: Vec<IODataWrapper>,
        monitor_read_data: Vec<MonitorMessage>,
        monitor_written_data: Vec<MonitorMessage>,
        read: bool,
        finished: bool,
    }

    async fn test_motion_worker_output_closed_unbreakable() -> MotionResult<(TestMotionWorker, MotionResult<MotionWorkerResult>)> {

        let mut source_1 = VecDeque::new();
        for i in 90..92 {
            source_1.push_back(IOData(vec![i]));
        }

        let (output_send_1, output_read_1) = bounded(2);
        let (output_send_2, output_read_2) = bounded(2);

        let (monitor_read_sender, montitor_read_receiver) = bounded(2);
        let (monitor_written_sender, montitor_written_receiver) = bounded(2);

        let mut motion_pull = Pull::Mock(PullJourney { src: 0, dst: 1 }, source_1);
        let mut motion_push = vec![
            Push::Sender(Journey { src: 0, dst: 1, breakable: Breakable::Terminate }, output_send_1),
            Push::Sender(Journey { src: 0, dst: 2, breakable: Breakable::Terminate }, output_send_2)
        ];

        let mut notifications = MotionNotifications {
            read: Some(monitor_read_sender),
            written: Some(monitor_written_sender),
        };

        let r1 = motion_worker(&mut motion_pull, &mut notifications, &mut motion_push, false).await?;

        let mut pull_reader_5 = Pull::Receiver(PullJourney { src: 1, dst: 5 }, output_read_1);
        let mut pull_reader_6 = Pull::Receiver(PullJourney { src: 2, dst: 6 }, output_read_2);

        let mut r_5 = vec![];
        let mut r_6 = vec![];
        r_5.push(motion_read(&mut pull_reader_5, false).await.unwrap());
        r_6.push(motion_read(&mut pull_reader_6, false).await.unwrap());
        drop(pull_reader_6);

        let mut monitor_read_data = vec![];
        let mut monitor_written_data = vec![];
        monitor_read_data.push(montitor_read_receiver.recv().await.unwrap());
        monitor_written_data.push(montitor_written_receiver.recv().await.unwrap());


        Ok(
            (
                TestMotionWorker {
                    data_5: r_5,
                    data_6: r_6,
                    monitor_read_data,
                    monitor_written_data,
                    read: r1.read,
                    finished: r1.finished,
                },
                motion_worker(&mut motion_pull, &mut notifications, &mut motion_push, false).await
            )
        )

    }


    let (r1, r2) = async_std::task::block_on(test_motion_worker_output_closed_unbreakable()).unwrap();
    assert_eq!(r1.data_5, vec![IODataWrapper::IOData(IOData(vec![90]))]);
    assert_eq!(r1.data_6, vec![IODataWrapper::IOData(IOData(vec![90]))]);
    assert_eq!(r1.monitor_read_data, vec![MonitorMessage::Read(0)]);
    assert_eq!(r1.monitor_written_data, vec![MonitorMessage::Wrote(0)]);
    assert_eq!(r1.read, true);
    assert_eq!(r1.finished, false);
    println!("{:?}", r2);
    assert_eq!(r2.is_ok(), false);

}


#[test]
fn test_motion_worker_output_closed_breakable() {

    use async_std::channel::bounded;

    async fn test_motion_worker_output_closed_breakable(breakable: Breakable) -> MotionResult<(Vec<IODataWrapper>, Vec<IODataWrapper>, MotionWorkerResult, MotionWorkerResult, MotionWorkerResult)> {

        let mut source_1 = VecDeque::new();
        for i in 90..99 {
            source_1.push_back(IOData(vec![i]));
        }

        let (output_send_1, output_read_1) = bounded(8);
        let (output_send_2, output_read_2) = bounded(8);

        let (monitor_read_sender, montitor_read_receiver) = bounded(8);
        let (monitor_written_sender, montitor_written_receiver) = bounded(8);

        let mut motion_pull = Pull::Mock(PullJourney { src: 0, dst: 1 }, source_1);
        let mut motion_push = vec![
            Push::Sender(Journey { src: 0, dst: 1, breakable }, output_send_1),
            Push::Sender(Journey { src: 0, dst: 2, breakable }, output_send_2)
        ];

        let mut notifications = MotionNotifications {
            read: Some(monitor_read_sender),
            written: Some(monitor_written_sender),
        };

        let r1 = motion_worker(&mut motion_pull, &mut notifications, &mut motion_push, false).await?;

        let mut pull_reader_5 = Pull::Receiver(PullJourney { src: 1, dst: 5 }, output_read_1);
        let mut pull_reader_6 = Pull::Receiver(PullJourney { src: 2, dst: 6 }, output_read_2);

        let mut r_5 = vec![];
        let mut r_6 = vec![];
        r_5.push(motion_read(&mut pull_reader_5, false).await.unwrap());
        r_6.push(motion_read(&mut pull_reader_6, false).await.unwrap());

        let mut monitor_read_data = vec![];
        let mut monitor_written_data = vec![];
        monitor_read_data.push(montitor_read_receiver.recv().await.unwrap());
        monitor_written_data.push(montitor_written_receiver.recv().await.unwrap());

        drop(pull_reader_5);
        let r2 = motion_worker(&mut motion_pull, &mut notifications, &mut motion_push, false).await?;
        r_6.push(motion_read(&mut pull_reader_6, false).await.unwrap());
        drop(pull_reader_6);
        let r3 = motion_worker(&mut motion_pull, &mut notifications, &mut motion_push, true).await?;

        Ok((r_5, r_6, r1, r2, r3))

    }

    let (a_output_data_1, a_output_data_2, a_r1, a_r2, a_r3) = async_std::task::block_on(test_motion_worker_output_closed_breakable(Breakable::Consume)).unwrap();
    assert_eq!(a_output_data_1, vec![IODataWrapper::IOData(IOData(vec![90]))]);
    assert_eq!(a_output_data_2, vec![IODataWrapper::IOData(IOData(vec![90])), IODataWrapper::IOData(IOData(vec![91]))]);
    assert_eq!(a_r1, MotionWorkerResult { read: true, finished: false });
    assert_eq!(a_r2, MotionWorkerResult { read: true, finished: false });
    assert_eq!(a_r3, MotionWorkerResult { read: true, finished: false });

    let (b_output_datb_1, b_output_datb_2, b_r1, b_r2, b_r3) = async_std::task::block_on(test_motion_worker_output_closed_breakable(Breakable::Finish)).unwrap();
    assert_eq!(b_output_datb_1, vec![IODataWrapper::IOData(IOData(vec![90]))]);
    assert_eq!(b_output_datb_2, vec![IODataWrapper::IOData(IOData(vec![90])), IODataWrapper::IOData(IOData(vec![91]))]);
    assert_eq!(b_r1, MotionWorkerResult { read: true, finished: false });
    assert_eq!(b_r2, MotionWorkerResult { read: true, finished: false });
    assert_eq!(b_r3, MotionWorkerResult { read: true, finished: true });

}


#[test]
fn test_motion_worker_skipped_input() {

    use async_std::channel::bounded;

    struct TestMotionWorker {
        data_5: Vec<IODataWrapper>,
        data_6: Vec<IODataWrapper>,
        monitor_read_data: Vec<MonitorMessage>,
        monitor_written_data: Vec<MonitorMessage>,
    }

    async fn test_motion_worker_skipped_input() -> MotionResult<(TestMotionWorker, MotionResult<MotionWorkerResult>, MotionResult<MotionWorkerResult>)> {

        let (motion_pull_send, motion_pull_read) = bounded(2);
        let mut motion_pull = Pull::Receiver(PullJourney { src: 1, dst: 5 }, motion_pull_read);
        motion_pull_send.send(IOData(vec![1])).await.unwrap();
        let (output_send_1, output_read_1) = bounded(2);
        let (output_send_2, output_read_2) = bounded(2);

        let (monitor_read_sender, montitor_read_receiver) = bounded(2);
        let (monitor_written_sender, montitor_written_receiver) = bounded(2);

        let mut motion_push = vec![
            Push::Sender(Journey { src: 0, dst: 1, breakable: Breakable::Terminate }, output_send_1),
            Push::Sender(Journey { src: 0, dst: 2, breakable: Breakable::Terminate }, output_send_2)
        ];

        let mut notifications = MotionNotifications {
            read: Some(monitor_read_sender),
            written: Some(monitor_written_sender),
        };

        motion_worker(&mut motion_pull, &mut notifications, &mut motion_push, false).await?;

        let mut pull_reader_5 = Pull::Receiver(PullJourney { src: 1, dst: 5 }, output_read_1);
        let mut pull_reader_6 = Pull::Receiver(PullJourney { src: 2, dst: 6 }, output_read_2);

        let mut r_5 = vec![];
        let mut r_6 = vec![];
        r_5.push(motion_read(&mut pull_reader_5, false).await.unwrap());
        r_6.push(motion_read(&mut pull_reader_6, false).await.unwrap());
        drop(pull_reader_6);

        let mut monitor_read_data = vec![];
        let mut monitor_written_data = vec![];
        monitor_read_data.push(montitor_read_receiver.recv().await.unwrap());
        monitor_written_data.push(montitor_written_receiver.recv().await.unwrap());


        let r2 = motion_worker(&mut motion_pull, &mut notifications, &mut motion_push, true).await;
        drop(motion_pull_send);

        Ok(
            (
                TestMotionWorker {
                    data_5: r_5,
                    data_6: r_6,
                    monitor_read_data,
                    monitor_written_data,
                },
                r2,
                motion_worker(&mut motion_pull, &mut notifications, &mut motion_push, true).await
            )
        )

    }


    let (r1, r2, r3) = async_std::task::block_on(test_motion_worker_skipped_input()).unwrap();
    assert_eq!(r1.data_5, vec![IODataWrapper::IOData(IOData(vec![1]))]);
    assert_eq!(r1.data_6, vec![IODataWrapper::IOData(IOData(vec![1]))]);
    assert_eq!(r1.monitor_read_data, vec![MonitorMessage::Read(0)]);
    assert_eq!(r1.monitor_written_data, vec![MonitorMessage::Wrote(0)]);
    assert_eq!(r2.is_ok(), true);
    let r2_ok = r2.unwrap();
    assert_eq!(r2_ok.read, false);
    assert_eq!(r2_ok.finished, false);
    assert_eq!(r3.is_ok(), true);
    let r3_ok = r3.unwrap();
    assert_eq!(r3_ok.read, true);
    assert_eq!(r3_ok.finished, true);

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

