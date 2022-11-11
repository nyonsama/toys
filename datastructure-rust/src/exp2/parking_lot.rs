use std::usize;
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::VecDeque,
};

#[test]
fn test_parkinglot() {
    let mut p = ParkingLot::new(2);
    let input="('A',1,5),('A',2,10),('D',1,15),('A',3,20),('A',4,25),('A',5,30),('D',2,35),('D',4,40),('E',0,0)";
    let input = input.trim_matches(|c| c == '(' || c == ')').split("),(");
    for event in input {
        let mut event = event.split(',');
        let action = event.next().unwrap().trim_matches('\'');
        let car_id = event.next().unwrap().parse::<usize>().unwrap();
        let cur_time = event.next().unwrap().parse::<usize>().unwrap();
        match action {
            "A" => {
                let (status, _) = p.arrive(car_id, cur_time);
                println!("car {} arrived, status:{:?}", car_id, status)
            }
            "D" => {
                let (duration, fee) = p.leave(car_id, cur_time);
                println!(
                    "car {} leaved, duration:{:?}, fee:{}",
                    car_id, duration, fee
                );
            }
            "E" => {
                println!("exiting");
                break;
            }
            _ => panic!("invalid action"),
        }
    }
}
#[test]
fn test_crap() {
    ParkingLot::run(2, "('A',1,5),('A',2,10),('D',1,15),('A',3,20),('A',4,25),('A',5,30),('D',2,35),('D',4,40),('E',0,0)")
}

pub struct ParkingLot {
    cap: usize,                 //capacity
    space: Vec<(usize, usize)>, //car_id, arrive_time
    queue: VecDeque<usize>,     //car_id
}

#[derive(Debug)]
pub enum Status {
    Parking,
    Waiting,
}

impl ParkingLot {
    /// cap是停车场容量
    pub fn new(cap: usize) -> Self {
        ParkingLot {
            cap,
            space: Vec::new(),
            queue: VecDeque::new(),
        }
    }

    /// 返回车辆状态和车辆id
    pub fn arrive(&mut self, car_id: usize, cur_time: usize) -> (Status, usize) {
        match self.space.len().cmp(&self.cap) {
            Greater => panic!("parking lot stack overflow!"),
            Less => {
                self.space.push((car_id, cur_time));
                (Status::Parking, car_id)
            }
            Equal => {
                self.queue.push_back(car_id);
                (Status::Waiting, car_id)
            }
        }
    }

    /// 返回停留时间和缴纳费用
    pub fn leave(&mut self, car_id: usize, cur_time: usize) -> (usize, usize) {
        let index = self.space.iter().position(|x| x.0 == car_id).unwrap();
        let duration = cur_time - self.space[index].1;
        self.space.remove(index);
        match self.queue.pop_front() {
            Some(car_id) => self.space.push((car_id, cur_time)),
            None => {}
        }
        // self.space.push((self.queue.pop_front().unwrap(), cur_time));
        (duration, duration * 2)
    }

    pub fn run(cap: usize, input: &str) {
        let mut p = ParkingLot::new(cap);
        // let input="('A',1,5),('A',2,10),('D',1,15),('A',3,20),('A',4,25),('A',5,30),('D',2,35),('D',4,40),('E',0,0)";
        let input = input.trim_matches(|c| c == '(' || c == ')').split("),(");
        for event in input {
            let mut event = event.split(',');
            let action = event.next().unwrap().trim_matches('\'');
            let car_id = event.next().unwrap().parse::<usize>().unwrap();
            let cur_time = event.next().unwrap().parse::<usize>().unwrap();
            match action {
                "A" => {
                    let (status, _) = p.arrive(car_id, cur_time);
                    println!("car {} arrived, status:{:?}", car_id, status)
                }
                "D" => {
                    let (duration, fee) = p.leave(car_id, cur_time);
                    println!(
                        "car {} leaved, duration:{:?}, fee:{}",
                        car_id, duration, fee
                    );
                }
                "E" => {
                    println!("exiting");
                    break;
                }
                _ => panic!("invalid action"),
            }
        }
    }
}
