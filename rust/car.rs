use super::{WIDTH, HEIGHT};
use super::board::Board;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Car {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    pub length: usize,
}

impl Car {
    pub fn get_coords(&self, index: usize) -> (usize, usize) {
        if let Direction::Horizontal = self.direction {
            (self.x + index, self.y)
        } else {
            (self.x, self.y + index)
        }
    }

    pub fn covers(&self, x: usize, y: usize) -> bool {
        if let Direction::Horizontal = self.direction {
            self.y == y && self.x <= x && self.x + self.length > x
        } else {
            self.x == x && self.y <= y && self.y + self.length > y
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CarMove {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub index: usize,
}

#[derive(Clone)]
pub struct CarIter<'a> {
    car: &'a Car,
    index: usize,
    board: &'a Board,

    distance: usize,
    orientation: bool,
}

impl<'a> CarIter<'a> {
    pub fn from_parts((car, index, board, distance, orientation): (&'a Car, usize, &'a Board, usize, bool)) -> Self {
        Self {
            car,
            index,
            board,
            distance,
            orientation
        }
    }
}

impl<'a> Iterator for CarIter<'a> {
    type Item = CarMove;

    fn next(&mut self) -> Option<Self::Item> {
        self.distance += 1;
        let offset = if self.orientation {self.distance as isize} else {-(self.distance as isize)};

        let (dx, dy) = if let Direction::Horizontal = self.car.direction {
            (offset, 0)
        } else {
            (0, offset)
        };


        let (detx, dety) = if self.orientation {
            let (x, y, len) = (self.car.x as isize, self.car.y as isize, self.car.length as isize);
            if let Direction::Horizontal = self.car.direction {
                (x + offset + len - 1, y)
            } else {
                (x, y + offset + len - 1)
            }
        } else {
            let (x, y) = (self.car.x as isize, self.car.y as isize);
            if let Direction::Horizontal = self.car.direction {
                (x + offset, y)
            } else {
                (x, y + offset)
            }
        };

        if detx < 0 || detx >= WIDTH as isize || dety < 0 || dety >= HEIGHT as isize || !self.board.is_square_empty(detx as usize, dety as usize) {
            if self.orientation {
                None
            } else {
                self.orientation = true;
                self.distance = 0;
                self.next()
            }
        } else {
            Some(CarMove {
                from: (self.car.x, self.car.y),
                to: ((self.car.x as isize + dx) as usize, (self.car.y as isize + dy) as usize),
                index: self.index,
            })
        }
    }
}

impl std::fmt::Display for CarMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {},{} -> {},{}", self.index, self.from.0, self.from.1, self.to.0, self.to.1)
    }
}
