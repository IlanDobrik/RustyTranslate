use std::{thread::sleep, time::Duration};

use anyhow::Result;

pub struct State {
    pub image: Vec<u8>,
    pub texts: Vec<String>,
    pub translated: String,
}
impl State {
    pub fn new() -> Self {
        Self {
            image: vec![],
            texts: vec![],
            translated: String::new(),
        }
    }
}

type Job = Box<dyn FnMut(&mut State) -> Result<&mut State>>;
pub trait Pipeline {
    fn register_func(&mut self, func: Job);
    fn run(&mut self) -> Result<()>;
}

pub struct SimplePipe {
    interval_sec: u64,
    functions: Vec<Job>,
}

impl Pipeline for SimplePipe {
    fn register_func(&mut self, func: Job) {
        self.functions.push(func)
    }

    fn run(&mut self) -> Result<()> {
        loop {
            let mut state = State::new();
            let _ = self
                .functions
                .iter_mut()
                .try_fold(&mut state, |acc, f| f(acc));
            sleep(Duration::from_secs(self.interval_sec));
        }
    }
}

impl SimplePipe {
    pub fn new(interval_sec: u64) -> Self {
        Self {
            interval_sec,
            functions: vec![],
        }
    }
}
