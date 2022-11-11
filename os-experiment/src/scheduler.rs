use crate::myprocess::{MyProcess, MyProcessStatus};
use std::collections::{BinaryHeap, VecDeque};
pub trait Scheduler {
    fn add_process(&mut self, p: MyProcess);
    fn start(&mut self);
}
pub struct HpfScheduler {
    queue: BinaryHeap<MyProcess>,
}

impl HpfScheduler {
    pub fn new() -> HpfScheduler {
        HpfScheduler {
            queue: BinaryHeap::new(),
        }
    }
}

impl Scheduler for HpfScheduler {
    fn add_process(&mut self, p: MyProcess) {
        self.queue.push(p)
    }

    fn start(&mut self) {
        loop {
            match self.queue.pop() {
                Some(mut p) => match p.run() {
                    MyProcessStatus::Stopped => {}
                    _ => self.queue.push(p),
                },
                None => break,
            }
            println!("current queue:");
            for p in self.queue.iter() {
                println!("  {}", p);
            }
        }
    }
}
pub struct RrScheduler {
    queue: VecDeque<MyProcess>,
}

impl RrScheduler {
    pub fn new() -> RrScheduler {
        RrScheduler {
            queue: VecDeque::new(),
        }
    }
}

impl Scheduler for RrScheduler {
    fn add_process(&mut self, p: MyProcess) {
        self.queue.push_back(p)
    }
    fn start(&mut self) {
        loop {
            match self.queue.pop_front() {
                Some(mut p) => match p.run() {
                    MyProcessStatus::Stopped => {}
                    _ => self.queue.push_back(p),
                },
                None => break,
            }
            println!("current queue:");
            for p in self.queue.iter() {
                println!("  {}", p);
            }
        }
    }
}
