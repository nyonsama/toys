use std::usize;

#[test]
fn test_maze() {
    let mut maze = Maze::new(
        8,
        9,
        (1, 1),
        (8, 9),
        vec![
            0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0,
            0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1,
            0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0,
        ],
    );
}

pub struct Maze {
    width: usize,
    height: usize,
    entrance: usize,
    exit: usize,
    maze: Vec<u8>,
}

impl Maze {
    /// 以左上角为(1,1)出入口为(行,列)
    pub fn new(
        width: usize,
        height: usize,
        entrance: (usize, usize),
        exit: (usize, usize),
        maze: Vec<u8>,
    ) -> Self {
        Maze {
            width,
            height,
            entrance: entrance.0 + entrance.1 * width,
            exit: exit.0 + exit.1 * width,
            maze,
        }
    }
    /// 返回Vec<(行，列，方向)>
    ///
    /// 方向1234依次表示右下左上
    pub fn start(&self) -> Vec<(usize, usize, usize)> {
        let mut pos = self.entrance;
        let mut stack=Vec::new();
        
    }
}
