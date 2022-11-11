use rand::{thread_rng, Rng};

#[test]
fn test() {
    let j = JosephRing::new(7, 20, vec![3, 1, 7, 2, 4, 8, 4]);
    println!("{:?}", j.run());
    let j = JosephRing::new_rand(7, 20);
    println!("{:?}", j.run());
}

pub struct JosephRing {
    limit: usize,
    ring: Vec<(usize, usize)>,
    //            id, password
}

impl JosephRing {
    pub fn new(n: usize, limit: usize, passwords: Vec<usize>) -> Self {
        let mut ring = Vec::with_capacity(n);
        ring.extend((1..=n).zip(passwords));
        JosephRing { limit, ring }
    }

    pub fn new_rand(n: usize, limit: usize) -> Self {
        let mut ring = Vec::with_capacity(n);
        let mut rng = thread_rng();
        ring.extend((1..=n).map(|x| (x, rng.gen_range(1..=n))));
        JosephRing { limit, ring }
    }

    /// this func consumes self
    pub fn run(mut self) -> Vec<usize> {
        let ring = &mut self.ring;
        let mut m = self.limit;
        let mut ret = Vec::with_capacity(ring.len());
        while !ring.is_empty() {
            let len = ring.len();
            ring.rotate_left(m % len);
            let p = ring.pop().unwrap();
            m = p.1;
            ret.push(p.0)
        }
        ret
    }
}
