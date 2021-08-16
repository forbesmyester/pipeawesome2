use crate::{buffer::{Buffer, BufferSizeMessage}, drain::Drain, faucet::{Faucet, FaucetControl}, junction::Junction, launch::Launch, motion::MotionError, startable_control::StartableControl};
use std::collections::HashMap;
use async_std::{channel::Receiver, prelude::*};

type LaunchString = Launch<HashMap<String, String>, String, String, Vec<String>, String, String, String>;

struct FaucetSettings {
    low_watermark: usize,
    high_watermark: usize,
    buffer_names: Vec<String>,
}

struct FaucetSettingsCapture {
    low_watermark: usize,
    high_watermark: usize,
    faucet_control: FaucetControl,
    buffer_size_monitors: Vec<Receiver<BufferSizeMessage>>,
}

pub struct Waiter {
    faucet: HashMap<String, Faucet>,
    launch: HashMap<String, LaunchString>,
    junction: HashMap<String, Junction>,
    buffer: HashMap<String, Buffer>,
    drain: HashMap<String, Drain>,
    faucet_settings: HashMap<String, FaucetSettings>,
}

#[derive(Debug)]
pub enum WaiterError {
    CausedByError(String, MotionError),
    SettingsForMissingFaucet(String),
    SettingsRefersMissingBuffer(String),
    CouldNotPause,
    CouldNotResume,
}


async fn manage_taps_from_buffers(mut faucet_settings: FaucetSettingsCapture) -> Result<usize, WaiterError> {

    let mut back_off = crate::back_off::BackOff::new();

    #[derive(Debug)]
    struct Bsm { pub buffer_size: usize, pub buffer_monitor: Receiver<BufferSizeMessage>, }

    let mut monitors: Vec<Bsm> = faucet_settings.buffer_size_monitors.into_iter().map(|bm| {
        Bsm {buffer_size: 0, buffer_monitor: bm }
    }).collect();

    #[derive(Debug, PartialEq)]
    enum FaucetStatus {
        Open,
        Closed,
    }

    let mut read_count = 0;
    let mut faucet_status = FaucetStatus::Open;
    loop {
        let mut to_remove = vec![];
        let mut nothing_read = true;
        let mut total = 0;
        for (index, buffer_size_monitor) in monitors.iter_mut().enumerate() {
            match buffer_size_monitor.buffer_monitor.try_recv() {
                Err(async_std::channel::TryRecvError::Closed) => {
                    to_remove.push(index);
                },
                Err(async_std::channel::TryRecvError::Empty) => {
                    total += buffer_size_monitor.buffer_size;
                },
                Ok(BufferSizeMessage(n)) => {
                    buffer_size_monitor.buffer_size = n;
                    total += n;
                    read_count += 1;
                    nothing_read = false;
                }
            }
        }
        if faucet_status == FaucetStatus::Open && total >= faucet_settings.high_watermark {
            faucet_settings.faucet_control.pause().await.map_err(|_| WaiterError::CouldNotPause)?;
            faucet_status = FaucetStatus::Closed;
        }
        if faucet_status == FaucetStatus::Closed && total <= faucet_settings.low_watermark {
            faucet_settings.faucet_control.resume().await.map_err(|_| WaiterError::CouldNotResume)?;
            faucet_status = FaucetStatus::Open;
        }
        //println!("TOTAL: {:?}: {:?}", faucet_status, total);
        for tr in to_remove.into_iter().rev() {
            monitors.remove(tr);
        }
        if monitors.is_empty() {
            return Ok(read_count);
        }
        match nothing_read {
            true => {
                back_off.wait().await
            },
            false => {
                back_off.reset()
            },
        };
    }
}

#[allow(clippy::new_without_default)]
impl Waiter {

    pub fn new() -> Waiter {
        Waiter {
            faucet: HashMap::new(),
            launch: HashMap::new(),
            junction: HashMap::new(),
            buffer: HashMap::new(),
            drain: HashMap::new(),
            faucet_settings: HashMap::new(),
        }
    }

    pub fn add_launch(&mut self, name: String, l: LaunchString) {
        self.launch.insert(name, l);
    }

    pub fn add_faucet(&mut self, name: String, f: Faucet) {
        self.faucet.insert(name, f);
    }

    pub fn add_junction(&mut self, name: String, j: Junction) {
        self.junction.insert(name, j);
    }

    pub fn add_buffer(&mut self, name: String, b: Buffer) {
        self.buffer.insert(name, b);
    }

    pub fn add_drain(&mut self, name: String, d: Drain) {
        self.drain.insert(name, d);
    }

    pub fn configure_faucet(&mut self, faucet_name: String, buffer_names: Vec<String>, low_watermark: usize, high_watermark: usize) -> bool {
        let has_all_buffers = buffer_names.iter().all(
            |f| self.buffer.get(f).is_some()
        );
        if !has_all_buffers || self.faucet.get(&faucet_name).is_none() {
            return false;
        }
        self.faucet_settings.insert(
            faucet_name,
            FaucetSettings { low_watermark, high_watermark, buffer_names }
        );
        true
    }

    fn faucet_settings(&mut self) -> Result<Vec<FaucetSettingsCapture>, WaiterError> {
        let mut faucet_settings_capture: Vec<FaucetSettingsCapture> = vec![];

        for (faucet_name, settings) in self.faucet_settings.iter() {
            let faucet = self.faucet.get_mut(faucet_name).ok_or_else(|| WaiterError::SettingsForMissingFaucet(faucet_name.to_string()))?;
            let faucet_control = faucet.get_control();
            let mut buffer_size_monitors = vec![];
            for buffer_name in settings.buffer_names.iter() {
                let buf = self.buffer.get_mut(buffer_name).ok_or_else(|| WaiterError::SettingsRefersMissingBuffer(buffer_name.to_string()))?;
                buffer_size_monitors.push(buf.add_buffer_size_monitor());
            }
            faucet_settings_capture.push(
                FaucetSettingsCapture {
                    low_watermark: settings.low_watermark,
                    high_watermark: settings.high_watermark,
                    faucet_control,
                    buffer_size_monitors
                }
            );
        }

        Ok(faucet_settings_capture)
    }

    #[allow(clippy::many_single_char_names)]
    pub async fn start(&mut self) -> Result<usize, WaiterError> {
        use futures::future::join_all;

        fn folder(acc: Result<usize, WaiterError>, cur: &mut Result<usize, WaiterError>) -> Result<usize, WaiterError> {
            match (acc, cur) {
                (Ok(i), Ok(j)) => Ok(i + *j),
                (Err(x), _) => Err(x),
                (_, x) => {
                    let mut y = Ok(0);
                    std::mem::swap(x, &mut y);
                    y
                }
            }
        }


        async fn start(name: &str, cntrl: &mut dyn StartableControl) -> Result<usize, WaiterError> {
            match cntrl.start().await {
                Err(e) => Err(WaiterError::CausedByError(name.to_string(), e)),
                Ok(x) => Ok(x),
            }
        }

        async fn start_launch(name: &str, cntrl: &mut LaunchString) -> Result<usize, WaiterError> {
            match cntrl.start().await {
                Err(e) => Err(WaiterError::CausedByError(name.to_string(), e)),
                Ok(x) => Ok(x),
            }
        }

        let managed_taps = join_all(self.faucet_settings()?.into_iter().map(manage_taps_from_buffers));
        let faucets = join_all(self.faucet.iter_mut().map(|(n, f)| start(n, f)));
        let launch = join_all(self.launch.iter_mut().map(
            |(n, l)| start_launch(n, l) )
        );
        let junction = join_all(self.junction.iter_mut().map(|(n, j)| start(n, j)));
        let buffers = join_all(self.buffer.iter_mut().map(|(n, b)| start(n, b)));
        let drain = join_all(self.drain.iter_mut().map(|(n, d)| start(n, d)));

        let (mut m, (mut f, (mut l, (mut j, (mut b, mut d))))) = managed_taps.join(faucets.join(launch.join(junction.join(buffers.join(drain))))).await;
        [
            m.iter_mut().fold(Ok(0), folder),
            f.iter_mut().fold(Ok(0), folder),
            l.iter_mut().fold(Ok(0), folder),
            j.iter_mut().fold(Ok(0), folder),
            b.iter_mut().fold(Ok(0), folder),
            d.iter_mut().fold(Ok(0), folder),
        ].iter_mut().fold(Ok(0), folder)
    }
}


