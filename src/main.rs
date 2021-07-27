use std::collections::HashMap;
// use std::time::{ Duration, Instant };
use async_std::io as aio;
use async_std::prelude::*;
use async_std::task;
use pipeawesome2::{buffer, motion::MotionNotifications};

use pipeawesome2::motion::{PullConfiguration, Pull, Push, motion, MotionResult};
use pipeawesome2::launch::Launch;

// struct Source (ControlId, Port);
// 
// enum GracePeriod {
//     Infinity,
//     Seconds(usize),
// }

// struct ControllerInfo {
//     tracked_count: usize,
//     last_send: Instant,
//     grace_period: Duration,
// }
// 
// type ControlId = usize;
// type DataStorageId = usize;
// 
// enum Port {
//     TAP,
//     STDOUT,
//     STDERR,
// }


// impl DataStorage {
// 
//     fn clean(&mut self) {
//         while !self.data.is_empty() && self.data[0].to_send_count == 0 {
//             self.data.pop_front();
//             self.offset = self.offset + 1;
//         }
//     }
// 
//     fn push(&mut self, pre_data: PreData, to_send_count: usize) -> PostData {
//         self.clean();
//         let sd: StoredData = StoredData {
//             data: pre_data.data,
//             source: pre_data.source,
//             to_send_count
//         };
//         self.data.push_back(sd);
//         PostData { id: self.offset + self.data.len() - 1, data: &self.data[self.data.len() -1].data }
//     }
// 
//     fn decrement_instance_count(&mut self, data_storage_id: DataStorageId) {
//         self.data[data_storage_id - self.offset].to_send_count = self.data[data_storage_id - self.offset].to_send_count - 1;
//         self.clean();
//     }
// 
//     fn new() -> DataStorage {
//         DataStorage {
//             data: VecDeque::new(),
//             offset: 0,
//         }
//     }
// 
// }
// 
// #[test]
// fn test_datastorage() {
//     let mut data_storage = DataStorage::new();
//     let pre_data_0 = PreData { data: IOData::Data(1, [0; 255]), len: 0, source: Source (0, Port::TAP) };
//     assert_eq!(0, data_storage.push(pre_data_0, 0).id);
//     assert_eq!(1, data_storage.data.len());
//     let pre_data_1 = PreData { data: IOData::Data(1, [0; 255]), len: 0, source: Source (0, Port::TAP) };
//     assert_eq!(1, data_storage.push(pre_data_1, 1).id);
//     assert_eq!(1, data_storage.data.len());
//     let pre_data_2 = PreData { data: IOData::Data(1, [0; 255]), len: 0, source: Source (0, Port::TAP) };
//     assert_eq!(2, data_storage.push(pre_data_2, 0).id);
//     assert_eq!(2, data_storage.data.len());
//     let pre_data_3 = PreData { data: IOData::Data(1, [0; 255]), len: 0, source: Source (0, Port::TAP) };
//     assert_eq!(3, data_storage.push(pre_data_3, 1).id);
//     assert_eq!(3, data_storage.data.len());
// 
//     data_storage.decrement_instance_count(1);
//     assert_eq!(1, data_storage.data.len());
// 
//     let pre_data_4 = PreData { data: IOData::Data(1, [0; 255]), len: 0, source: Source (0, Port::TAP) };
//     assert_eq!(4, data_storage.push(pre_data_4, 0).id);
//     assert_eq!(2, data_storage.data.len());
// 
// }


// Pull::IoMock / Pull::CmdStderr / Pull::CmdStdout / Pull::Stdin -> Push::IoSender = 0
// Pull::IoReceiver -> Push::Sender * n = n
// Pull::Receiver -> Push::Stdout / Push::Stderr / Push::CmdStdin Push::IoMock -1
// Pull::Receiver -> Sender = 0

// #[derive(Debug)]
// pub struct NativeLaunchSpec<E, P, O, A, K, V, R>
//     where E: IntoIterator<Item = (K, V)>,
//           A: IntoIterator<Item = R>,
//           R: AsRef<OsStr>,
//           O: AsRef<OsStr>,
//           K: AsRef<OsStr>,
//           V: AsRef<OsStr>,
//           P: AsRef<Path>,
// {
//     command: O,
//     path: Option<P>,
//     env: Option<E>,
//     args: Option<A>,
// }
// 
// impl <E: IntoIterator<Item = (K, V)>,
//           A: IntoIterator<Item = R>,
//           R: AsRef<OsStr>,
//           O: AsRef<OsStr>,
//           K: AsRef<OsStr>,
//           V: AsRef<OsStr>,
//           P: AsRef<Path>> NativeLaunchSpec<E, P, O, A, K, V, R> {
//     pub fn new(env: E, path: P, command: O, args: A) -> NativeLaunchSpec<E, P, O, A, K, V, R> {
//         NativeLaunchSpec { command, path: Some(path), env: Some(env), args: Some(args) }
//     }
// }

// async fn generate_ids(s: InputIdSetter) -> usize {
//     let mut i: usize = 0;
//     while !s.is_closed() {
//         s.send(i);
//         i = i + 1;
//     }
//     i
// }

async fn do_stuff() -> MotionResult<usize> {

    let stdin = aio::stdin();
    let stdout = aio::stdout();

    // let cmd = aip::Command::new("sed")
    //     .arg("-u")
    //     .arg("s/^/OUT: /")
    //     .stdout(aip::Stdio::piped())
    //     .stdin(aip::Stdio::piped())
    //     .spawn()?;

    // let proc_stdin_pull = PullConfiguration {
    //     pull: Pull::Stdin(stdin),
    //     priority: 1,
    //     id: 0,
    // };

    // let mut cmd_stdin = Push::CmdStdin(cmd.stdin.unwrap());
    // let mut cmd_stdout = Pull::CmdStdout(cmd.stdout.unwrap());
    let proc_stdout_pushs = vec![Push::Stdout(stdout)];
    // let (input_id_setter, input_id_getter): (InputIdSetter, InputIdGetter) = bounded(8);

    // let have_read = motion_read(&mut proc_stdin);

    // // let t = task::spawn(async { true });

    // let (monitor_snd_1, _monitor_rcv): (Sender<MonitorMessage>, Receiver<MonitorMessage>) = bounded(8);
    // let monitor_snd_2 = monitor_snd_1.clone();

    let mut launch: Launch<HashMap<String, String>, String, &str, Vec<String>, String, String, String> = Launch::new(
        None,
        None,
        "sed",
        Some(vec!["-u".to_string(), "s/^/OUT: /".to_string()])
    );

    launch.add_stdin(Pull::Stdin(stdin), 1);

    let sed_stdout_pulls = vec![PullConfiguration {
        pull: launch.add_stdout(),
        priority: 1,
        id: 11
    }];

    let launch_motion = launch.start();

    let stdout_motion = motion(sed_stdout_pulls, MotionNotifications::empty(), proc_stdout_pushs);

    // motion_write(&mut cmd_stdin, have_read.await?).await?;
    // motion_write(&mut proc_stdout, motion_read(&mut cmd_stdout).await?).await
    let res = launch_motion.join(stdout_motion).await;
    match res {
        (MotionResult::Ok(a), MotionResult::Ok(b)) => MotionResult::Ok(a + b),
        (MotionResult::Ok(_a), b) => b,
        (a, _b) => a
    }
}


fn main() {

    // task::block_on(test_motion_impl());
    // println!("{:?}", task::block_on(do_stuff()));
    println!("{:?}", task::block_on(buffer::test_buffer_impl()));



    // let (mut data_storage) = &DataStorage::new();

    // let pre_data_send = PreData { data: [0; 255], len: 0, source: Source (0, Port::TAP) };
    // let post_data_send = data_storage.push(pre_data_send, 1);

    // sender.send(post_data_send);

    // std::thread::spawn(move || {
    //     println!("{:?}", reciever.recv().unwrap());
    // });
    println!("Hello, world!");
}
