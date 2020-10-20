use crate::Point;

pub enum LinearFunction {
    Standard { slope: f32, y_int: f32 },
    Vertical { x: i32 }
}

impl LinearFunction {
    pub fn from_points(p1: &Point<i32>, p2: &Point<i32>) -> Self {
        if p1.x == p2.x {
            return LinearFunction::Vertical { x: p1.x };
        }

        let p1f = p1.convert_to_f32();
        let p2f = p2.convert_to_f32();
        let slope = (p2f.y - p1f.y) / (p2f.x - p1f.x);
        let y_int = p1f.y - (slope * p1f.x);

        LinearFunction::Standard { slope, y_int }
    }

    pub fn solve_x(&self, y: i32) -> i32 {
        match self {
            Self::Standard { slope, y_int } => ((y as f32 - y_int) / slope) as i32,
            Self::Vertical { x } => *x,
        }
    }

    pub fn solve_y(&self, x: i32) -> i32 {
        match self {
            Self::Standard { slope, y_int } => ((slope * x as f32) + y_int) as i32,
            Self::Vertical { x: _ } => panic!("Cannot solve for y given a vertical LinearFunction!"),
        }
    }
}
