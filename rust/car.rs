use super::{WIDTH, HEIGHT};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Car {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    pub length: usize,
}

pub struct CarMove {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub index: usize,
}

#[derive(Clone)]
pub struct CarIter<'a> {
    car: &'a Car,
    index: usize,
    board: &'a [bool; WIDTH * HEIGHT],

    distance: usize,
    orientation: bool,
}

impl<'a> CarIter<'a> {
    pub fn from_parts((car, index, board, distance, orientation): (&'a Car, usize, &'a [bool; WIDTH * HEIGHT], usize, bool)) -> Self {
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
                (x + offset + len, y)
            } else {
                (x, y + offset + len)
            }
        } else {
            let (x, y) = (self.car.x as isize, self.car.y as isize);
            if let Direction::Horizontal = self.car.direction {
                (x + offset, y)
            } else {
                (x, y + offset)
            }
        };

        if detx < 0 || detx >= WIDTH as isize || dety < 0 || dety >= HEIGHT as isize || self.board[detx as usize + dety as usize * WIDTH] {
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
