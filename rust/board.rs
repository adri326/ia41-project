use super::{WIDTH, HEIGHT};
use super::car::{Car, CarIter, CarMove, Direction};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Board {
    pub cars: Vec<Car>,
    pub board: [bool; WIDTH * HEIGHT],
}

impl std::cmp::PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        self.cars == other.cars
    }
}

impl std::hash::Hash for Board {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cars.hash(state);
    }
}

#[derive(Clone)]
pub struct BoardIter<'a> {
    cars: Vec<CarIter<'a>>,
}

impl<'a> Iterator for BoardIter<'a> {
    type Item = CarMove;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.cars.len();

        if len == 0 {
            return None;
        }

        let car_iter = &mut self.cars[len - 1];
        match car_iter.next() {
            Some(x) => Some(x),
            None => {
                std::mem::drop(car_iter);
                self.cars.pop();
                self.next() // Hopefully TCO can happen here?
            }
        }
    }
}

impl Board {
    pub fn from_string(string: &str) -> Option<Board> {
        let mut string = string.chars().filter(|c| *c != '\n').collect::<Vec<_>>();

        // println!("{} / {}: {:?}", string.len(), WIDTH * HEIGHT, string);
        if string.len() > WIDTH * HEIGHT {
            return None
        }
        while string.len() < WIDTH * HEIGHT {
            string.push(' ');
        }

        let mut cars: HashMap<usize, Car> = HashMap::new();

        // Find cars in string
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let n = string[x + y * WIDTH];
                if n >= '0' && n <= '9' || n >= 'a' && n <= 'z' || n >= 'A' && n <= 'Z' {
                    let n = n.to_digit(36).unwrap() as usize;
                    if let None = cars.get(&n) {
                        let mut length = 0;
                        if x < WIDTH - 1 && string[x + 1 + y * WIDTH] == string[x + y * WIDTH] {
                            // Horizontal
                            for x2 in x..WIDTH {
                                if string[x2 + y * WIDTH] == string[x + y * WIDTH] {
                                    length += 1;
                                }
                            }
                            cars.insert(n, Car {
                                x,
                                y,
                                length,
                                direction: Direction::Horizontal,
                            });
                        } else if y < HEIGHT - 1 {
                            // Vertical
                            for y2 in y..HEIGHT {
                                if string[x + y2 * WIDTH] == string[x + y * WIDTH] {
                                    length += 1;
                                }
                            }
                            cars.insert(n, Car {
                                x,
                                y,
                                length,
                                direction: Direction::Vertical,
                            });
                        }
                    }
                }
            }
        }

        // Compile car hashmap to vec
        let mut cars: Vec<(usize, Car)> = cars.into_iter().collect();
        cars.sort_unstable_by_key(|(i, _c)| *i);
        let mut cars: Vec<Car> = cars.into_iter().map(|(_i, c)| c).collect();

        // Compute bitmap
        let mut board = [false; WIDTH * HEIGHT];
        for car in cars.iter() {
            for l in 0..car.length {
                let (x, y) = car.get_coords(l);
                board[x + y * WIDTH] = true;
            }
        }

        Some(Board {
            cars,
            board
        })
    }

    pub fn iter<'a>(&'a self) -> BoardIter<'a> {
        let mut cars: Vec<CarIter<'a>> = Vec::new();

        for (index, car) in self.cars.iter().enumerate() {
            cars.push(CarIter::from_parts((
                car,
                index,
                &self.board,
                0,
                false,
            )));
        }

        BoardIter {
            cars
        }
    }

    pub fn apply(&self, mv: &CarMove) -> Board {
        let mut new_cars = self.cars.clone();
        let mut new_board = self.board.clone();

        // replace car in bitmap with zeroes
        for l in 0..new_cars[mv.index].length {
            let (x, y) = new_cars[mv.index].get_coords(l);
            new_board[x + y * WIDTH] = false;
        }

        // Update car position
        new_cars[mv.index].x = mv.to.0;
        new_cars[mv.index].y = mv.to.1;

        // replace car in bitmap with ones
        for l in 0..new_cars[mv.index].length {
            let (x, y) = new_cars[mv.index].get_coords(l);
            new_board[x + y * WIDTH] = true;
        }

        Board {
            cars: new_cars,
            board: new_board,
        }
    }
}
