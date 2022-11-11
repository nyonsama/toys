use crate::myprocess::MyProcess;
use crate::myprocess::MyProcessStatus;

pub struct BankerAllocator {
    res: [usize; 3],
}

impl BankerAllocator {
    pub fn new(res: (usize, usize, usize)) -> BankerAllocator {
        BankerAllocator {
            res: [res.0, res.1, res.2],
        }
    }
}

impl MyAllocator1 for BankerAllocator {
    fn allocate_for<'a>(
        &mut self,
        proc: &mut MyProcess, // 给哪个进程分配
        request: &'a [usize], // 分配多少
    ) -> Option<&'a [usize]> {
        assert!(request.len() == 3);
        let mut ret = Some(request);

        //检查是否有足够的资源
        for i in 0..3 {
            if proc.resource.need[i] > self.res[i] {
                ret = None;
            }
        }

        //分配资源
        match ret {
            Some(_) => {
                for i in 0..3 {
                    self.res[i] -= request[i];
                    proc.resource.own[i] += request[i];
                    proc.resource.need[i] -= request[i];
                }
                println!(
                    "system resource: A:{} B:{} C:{}",
                    self.res[0], self.res[1], self.res[2]
                );
            }
            None => {}
        }
        ret
    }

    fn free<'a>(&mut self, proc: &'a mut MyProcess) -> &'a [usize] {
        for i in 0..3 {
            self.res[i] += proc.resource.max[i];
        }
        &proc.resource.max
    }
}

pub struct RandomAllocator {
    res: [usize; 3],
}

impl RandomAllocator {
    pub fn new(res: (usize, usize, usize)) -> Self {
        Self {
            res: [res.0, res.1, res.2],
        }
    }
}

impl MyAllocator1 for RandomAllocator {
    fn allocate_for<'a>(
        &mut self,
        proc: &mut MyProcess, // 给哪个进程分配
        request: &'a [usize], // 分配多少
    ) -> Option<&'a [usize]> {
        let mut ret = Some(request);

        //检查是否有足够的资源
        for i in 0..3 {
            if request[i] > self.res[i] {
                ret = None;
            }
        }

        //分配资源
        match ret {
            Some(_) => {
                for i in 0..3 {
                    self.res[i] -= request[i];
                    proc.resource.own[i] += request[i];
                    proc.resource.need[i] -= request[i];
                }
                println!(
                    "system resource: A:{} B:{} C:{}",
                    self.res[0], self.res[1], self.res[2]
                );
                proc.set_status(MyProcessStatus::Ready)
            }
            None => proc.set_status(MyProcessStatus::Waiting),
        }
        ret
    }

    fn free<'a>(&mut self, proc: &'a mut MyProcess) -> &'a [usize] {
        for i in 0..3 {
            self.res[i] += proc.resource.max[i];
        }
        &proc.resource.max
    }
}

pub struct SeqAllocator {
    res: [usize; 10], //数组索引是资源号，内容是拥有这个资源的pid
}

impl SeqAllocator {
    pub fn new() -> SeqAllocator {
        SeqAllocator {
            res: [usize::MAX; 10],
        }
    }
}

impl MyAllocator2 for SeqAllocator {
    fn allocate_for(&mut self, p: &mut MyProcess, seq: &mut Vec<usize>) -> Option<Vec<usize>> {
        let mut temp_seq = seq.clone();
        temp_seq.sort();
        temp_seq.reverse();

        p.resource.last = p.resource.cur;
        let mut occupied = false;
        //检查资源是否已被占用
        for &i in temp_seq.iter() {
            if i < p.resource.cur {
                //只检查最大编号到请求编号的资源
                break;
            }

            let owner = self.res[i];
            if owner != usize::MAX && owner != p.get_pid() {
                //这个资源已被占用
                occupied = true;
            }
        }

        let mut allocated = Vec::new();
        if occupied {
            p.set_status(MyProcessStatus::Waiting);
            None
        } else {
            //给当前进程分配资源
            for &res_id in temp_seq.iter() {
                if res_id < p.resource.cur {
                    break;
                }
                self.res[res_id] = p.get_pid();
                allocated.push(res_id);

                //把已分配的资源号从请求队列中去掉
                for j in 0..seq.len() {
                    if seq[j] == res_id {
                        seq.remove(j);
                        break;
                    }
                }
            }
            p.set_status(MyProcessStatus::Ready);
            Some(allocated)
        }
    }

    fn free(&mut self, p: &mut MyProcess) -> Option<Vec<usize>> {
        p.set_status(MyProcessStatus::Stopped);
        let mut ret = Vec::new();
        for i in 0..self.res.len() {
            if self.res[i] == p.get_pid() {
                self.res[i] = usize::MAX;
                ret.push(i);
            }
        }
        if ret.is_empty() {
            None
        } else {
            Some(ret)
        }
    }
}

pub struct BlindAllocator {
    res: [usize; 10], //数组索引是资源号，内容是拥有这个资源的pid
}

impl BlindAllocator {
    pub fn new() -> BlindAllocator {
        BlindAllocator {
            res: [usize::MAX; 10],
        }
    }
}

impl MyAllocator2 for BlindAllocator {
    fn allocate_for(&mut self, p: &mut MyProcess, seq: &mut Vec<usize>) -> Option<Vec<usize>> {
        let pid = p.get_pid();
        let request = p.resource.cur;
        p.resource.last = request;
        //检查资源是否已被占用
        if self.res[request] != usize::MAX && self.res[request] != pid {
            p.set_status(MyProcessStatus::Waiting);
            None
        } else {
            //找到这个请求在请求列表中的位置，将其去掉
            for i in 0..seq.len() {
                if seq[i] == request {
                    seq.remove(i);
                    break;
                }
            }

            self.res[request] = pid;
            p.set_status(MyProcessStatus::Ready);
            Some(vec![request])
        }
    }

    fn free(&mut self, p: &mut MyProcess) -> Option<Vec<usize>> {
        p.set_status(MyProcessStatus::Stopped);
        let mut ret = Vec::new();
        for i in 0..self.res.len() {
            if self.res[i] == p.get_pid() {
                self.res[i] = usize::MAX;
                ret.push(i);
            }
        }
        if ret.is_empty() {
            None
        } else {
            Some(ret)
        }
    }
}

pub trait MyAllocator2 {
    fn allocate_for(&mut self, p: &mut MyProcess, seq: &mut Vec<usize>) -> Option<Vec<usize>>;
    fn free(&mut self, p: &mut MyProcess) -> Option<Vec<usize>>;
}

pub trait MyAllocator1 {
    /// 分配资源
    /// 成功分配则返回Some(request)
    fn allocate_for<'a>(
        &mut self,
        proc: &mut MyProcess,
        request: &'a [usize],
    ) -> Option<&'a [usize]>;

    fn free<'a>(&mut self, proc: &'a mut MyProcess) -> &'a [usize];
}
