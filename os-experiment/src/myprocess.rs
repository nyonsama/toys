use std::fmt;
use std::{cmp::Ordering, usize};
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MyProcessStatus {
    Ready,
    // Running,
    Waiting,
    Stopped,
}

#[derive(Clone)]
pub struct MyProcess {
    pid: usize,
    priority: usize,
    total_time: usize,
    used_time: usize,
    status: MyProcessStatus,
    pub resource: Resource,
}

impl PartialEq for MyProcess {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl PartialOrd for MyProcess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl MyProcess {
    pub fn new(pid: usize, priority: usize, total_time: usize) -> Self {
        MyProcess {
            pid,
            priority,
            total_time,
            used_time: 0,
            status: MyProcessStatus::Ready,
            resource: Resource {
                max: [0, 0, 0],
                own: [0, 0, 0],
                need: [0, 0, 0],
                cur: 0,
                last: 0,
            },
        }
    }

    pub fn run(&mut self) -> &MyProcessStatus {
        match self.status {
            MyProcessStatus::Ready => {
                println!("starting process(pid:{})", self.pid);
                self.priority -= 1;
                self.used_time += 1;
                if self.total_time == self.used_time {
                    println!("stopping process(pid:{})", self.pid);
                    self.status = MyProcessStatus::Stopped;
                } else {
                    println!("halting process(pid:{})", self.pid);
                }
                &self.status
            }
            _ => {
                panic!("this process (pid:{}) can not run!", self.pid)
            }
        }
    }

    pub fn set_status(&mut self, status: MyProcessStatus) {
        self.status = status;
    }

    pub fn set_resource(&mut self, res: Resource) {
        self.resource = res;
    }

    pub fn get_pid(&self) -> usize {
        self.pid
    }

    pub fn own_all_resource(&self) -> bool {
        let mut ret = true;
        for &i in self.resource.need.iter() {
            if i != 0 {
                ret = false;
            }
        }
        ret
    }
}

impl fmt::Display for MyProcess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pid:{} priority:{} used_time:{} total_time:{} status:{:?}",
            self.pid, self.priority, self.used_time, self.total_time, self.status
        )
    }
}

impl Eq for MyProcess {}

impl Ord for MyProcess {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

#[derive(Clone)]
pub struct Resource {
    pub max: [usize; 3],  //exp2_1
    pub own: [usize; 3],  //exp2_1
    pub need: [usize; 3], //exp2_1
    pub cur: usize,       //exp2_2
    pub last: usize,      //exp2_2
}
impl Resource {
    pub fn new_exp2_1(max: [usize; 3], own: [usize; 3], need: [usize; 3]) -> Resource {
        Resource {
            max,
            own,
            need,
            cur: 0,
            last: 0,
        }
    }
    // pub fn new_exp2_2(cur: usize, last: usize) -> Resource {
    //     Resource {
    //         max: [0, 0, 0],
    //         own: [0, 0, 0],
    //         need: [0, 0, 0],
    //         cur,
    //         last,
    //     }
    // }
}
