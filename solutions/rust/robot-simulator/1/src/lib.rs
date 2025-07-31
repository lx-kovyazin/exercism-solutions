#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        Self::new(
            self.x,
            self.y,
            match self.d {
                Direction::North => Direction::East,
                Direction::East  => Direction::South,
                Direction::South => Direction::West,
                Direction::West  => Direction::North,
            }
        )
    }

    pub fn turn_left(self) -> Self {
        Self::new(
            self.x,
            self.y,
            match self.d {
                Direction::North => Direction::West,
                Direction::East  => Direction::North,
                Direction::South => Direction::East,
                Direction::West  => Direction::South,
            }
        )
    }

    pub fn advance(self) -> Self {
        let (ox, oy) = match self.d {
            Direction::North => (0 ,  1),
            Direction::East  => (1 ,  0),
            Direction::South => (0 , -1),
            Direction::West  => (-1,  0),
        };
        Self::new(self.x + ox, self.y + oy, self.d)
    }

    pub fn instructions(self, instructions: &str) -> Self {
        const MAP: [(char, fn(Robot) -> Robot); 3] = [
            ('L', Robot::turn_left),
            ('R', Robot::turn_right),
            ('A', Robot::advance),
        ];
        instructions.chars().fold(self, |r, i| MAP.iter().find(|(c, _)| i.eq(c)).unwrap().1(r))
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
