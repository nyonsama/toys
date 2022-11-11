mod myallocator;
mod myprocess;
mod scheduler;
use myallocator::*;
use myprocess::*;
use scheduler::*;
use std::{
    collections::VecDeque,
    io::{self, Write},
};
fn main() {
    let menu = concat!(
        "choose a experiment to run.\n",
        "1. exp1   scheduler\n",
        "2. exp2.1 banker allocator\n",
        "3. exp2.2 ordered allocator\n",
        "4. exit\n"
    );
    loop {
        println!();
        print!("{}", menu);
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        match buf.trim().parse() {
            Ok(choice) => match choice {
                1 => exp1(),
                2 => exp2_1(),
                3 => exp2_2(),
                4 => break,
                _ => {}
            },
            Err(_) => {}
        }
    }
}

fn exp1() {
    // let processes = vec![
    //     MyProcess::new(1, 11, 5),
    //     MyProcess::new(2, 10, 3),
    //     MyProcess::new(3, 13, 2),
    //     MyProcess::new(4, 12, 1),
    //     MyProcess::new(5, 11, 4),
    // ];

    println!("please input priority and max time for 5 processes.");
    println!("1 line for 1 process,seperate by space.");
    let mut processes = Vec::new();
    for i in 0..5 {
        print!("process{}:", i);
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let mut proc_data = buf
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("your input is not valid"));

        processes.push(MyProcess::new(
            i,
            proc_data
                .next()
                .expect(format!("missing priority for pid:{}", i).as_str()),
            proc_data
                .next()
                .expect(format!("missing max time for pid:{}", i).as_str()),
        ))
    }

    let mut hpf = HpfScheduler::new();
    for p in processes.iter() {
        hpf.add_process(p.clone());
    }
    println!("{:-^60}", "start hpf scheduler");
    hpf.start();

    let mut rr = RrScheduler::new();
    for p in processes.iter() {
        rr.add_process(p.clone());
    }
    println!("{:-^60}", "start rr scheduler");
    rr.start();
}

fn exp2_1() {
    let mut processes = VecDeque::new();
    for i in 0..5 {
        processes.push_back(MyProcess::new(i, 0, 0));
    }
    processes[0].set_resource(Resource::new_exp2_1([7, 5, 3], [0, 1, 0], [7, 4, 3]));
    processes[1].set_resource(Resource::new_exp2_1([3, 2, 2], [2, 0, 0], [1, 2, 2]));
    processes[2].set_resource(Resource::new_exp2_1([9, 0, 2], [3, 0, 2], [6, 0, 0]));
    processes[3].set_resource(Resource::new_exp2_1([2, 2, 2], [2, 1, 1], [0, 1, 1]));
    processes[4].set_resource(Resource::new_exp2_1([4, 3, 3], [0, 0, 2], [4, 3, 1]));

    // 资源 请求 序列
    let resource_request_sequence = [
        vec![[2usize, 1, 3], [5, 3, 0]],
        vec![[1, 2, 2]],
        vec![[2, 0, 0], [4, 0, 0]],
        vec![[0, 1, 1]],
        vec![[1, 0, 0], [3, 3, 1]],
    ];

    println!("{:-^60}", "starting banker allocator");
    exp2_1_allocator_test(
        &mut processes.clone(),
        &mut BankerAllocator::new((3, 3, 2)),
        &mut resource_request_sequence.clone(),
    );

    println!("{:-^60}", "starting random allocator");
    exp2_1_allocator_test(
        &mut processes.clone(),
        &mut RandomAllocator::new((3, 3, 2)),
        &mut resource_request_sequence.clone(),
    );
}

fn exp2_1_allocator_test<T: MyAllocator1>(
    processes: &mut VecDeque<MyProcess>,
    allocator: &mut T,
    res_req_seq: &mut [Vec<[usize; 3]>],
) {
    let mut timer = 0usize;
    // 遍历进程队列，分配资源，将已经获得全部资源的进程移出队列
    loop {
        timer += 1;
        match processes.pop_front() {
            Some(mut proc) => {
                let pid = proc.get_pid();
                // println!("allocating for pid:{}", proc.get_pid());
                if proc.own_all_resource() {
                    println!("pid:{} has owned all resource.free it.", pid);
                    allocator.free(&mut proc);
                } else {
                    match allocator.allocate_for(&mut proc, &res_req_seq[pid][0]) {
                        Some(r) => {
                            println!(
                                "allocated resource A:{}, B:{}, C:{} for pid:{}",
                                r[0], r[1], r[2], pid
                            );
                            res_req_seq[pid].remove(0);
                        }
                        None => {
                            println!("allocate for pid:{} failed.resource is not enough.", pid);
                        }
                    };
                    processes.push_back(proc);
                }
            }
            None => {
                println!("all processes are satisfied.");
                break;
            }
        };
        if timer > 60 {
            println!("dead lock occurred!");
            break;
        }
    }
}

fn exp2_2() {
    let mut processes = VecDeque::new();
    for i in 0..3 {
        processes.push_back(MyProcess::new(i, 0, 0));
    }

    let resource_request_sequence = [
        vec![1, 5, 3, 0, 6, 8, 7],       //pid0
        vec![9, 2, 5, 4, 3, 1],          //pid1
        vec![1, 4, 6, 9, 3, 5, 8, 2, 0], //pid2
    ];

    println!("{:-^60}", "starting sequence allocator");
    exp2_2_allocator_test(
        processes.clone(),
        SeqAllocator::new(),
        resource_request_sequence.clone(),
    );
    println!("{:-^60}", "starting blind allocator");
    exp2_2_allocator_test(
        processes.clone(),
        BlindAllocator::new(),
        resource_request_sequence.clone(),
    );
}

fn exp2_2_allocator_test<T: MyAllocator2>(
    mut processes: VecDeque<MyProcess>,
    mut allocator: T,
    mut all_request_list: [Vec<usize>; 3],
) {
    let mut timer = 0usize;
    loop {
        timer += 1;
        match processes.pop_front() {
            Some(mut proc) => {
                let pid = proc.get_pid();
                let requests = &mut all_request_list[pid];

                // println!("allocating for pid:{}", pid);
                if !requests.is_empty() {
                    proc.resource.cur = requests[0];
                    //为proc分配资源
                    match allocator.allocate_for(&mut proc, requests) {
                        Some(allocated) => {
                            println!("allocated {:?} for pid:{}", allocated, pid);
                        }
                        None => {
                            println!("no enough resource({}) for pid:{}", proc.resource.cur, pid);
                        }
                    }
                    processes.push_back(proc);
                } else {
                    println!("pid:{} has owned all resource.free it.", pid);
                    allocator.free(&mut proc);
                }
            }
            None => {
                println!("all processes are satisfied.");
                break;
            }
        }
        if timer > 40 {
            println!("dead lock occured!");
            break;
        }
    }
}
