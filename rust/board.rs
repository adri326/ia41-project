use super::{WIDTH, HEIGHT};
use super::car::{Car, CarIter, CarMove, Direction};
use std::collections::HashMap;
use fasthash::MetroHasher;
// use fasthash::xx::Hasher64;
use std::hash::Hasher;

#[cfg(feature = "bitboard")]
const_assert!(WIDTH * HEIGHT <= u64::BITS as usize);

#[derive(Clone, Debug)]
pub struct Board {
    pub cars: Vec<Car>,

    #[cfg(not(feature = "bitboard"))]
    bitboard: [bool; WIDTH * HEIGHT],

    #[cfg(feature = "bitboard")]
    bitboard: u64,
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
        let mut table = (0..HEIGHT).map(|_| vec![' '; WIDTH]).collect::<Vec<_>>();
        let mut x = 0;
        let mut y = 0;
        for c in string.chars() {
            if c == '\n' {
                y += 1;
                x = 0;
            } else {
                table[y][x] = c;
                x += 1;
            }
        }

        let mut cars: HashMap<usize, Car> = HashMap::new();

        // Find cars in string
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let n = table[y][x];
                if n >= '0' && n <= '9' || n >= 'a' && n <= 'z' || n >= 'A' && n <= 'Z' {
                    let n = n.to_digit(36).unwrap() as usize;
                    if let None = cars.get(&n) {
                        let mut length = 0;
                        if x < WIDTH - 1 && table[y][x + 1] == table[y][x] {
                            // Horizontal
                            for x2 in x..WIDTH {
                                if table[y][x2] == table[y][x] {
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
                                if table[y2][x] == table[y][x] {
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
        let cars: Vec<Car> = cars.into_iter().map(|(_i, c)| c).collect();

        // Compute bitmap
        let bitboard = compute_bitboard(&cars);

        Some(Board {
            cars,
            bitboard
        })
    }

    pub fn iter<'a>(&'a self) -> BoardIter<'a> {
        let mut cars: Vec<CarIter<'a>> = Vec::new();

        for (index, car) in self.cars.iter().enumerate() {
            cars.push(CarIter::from_parts((
                car,
                index,
                &self,
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
        let mut new_board = self.bitboard.clone();

        // replace car in bitmap with zeroes
        for l in 0..new_cars[mv.index].length {
            let (x, y) = new_cars[mv.index].get_coords(l);
            set_bitboard(&mut new_board, x, y, false);
        }

        // Update car position
        new_cars[mv.index].x = mv.to.0;
        new_cars[mv.index].y = mv.to.1;

        // replace car in bitmap with ones
        for l in 0..new_cars[mv.index].length {
            let (x, y) = new_cars[mv.index].get_coords(l);
            set_bitboard(&mut new_board, x, y, true);
        }

        Board {
            cars: new_cars,
            bitboard: new_board,
        }
    }

    pub fn is_square_empty(&self, x: usize, y: usize) -> bool {
        if x >= WIDTH || y >= HEIGHT {
            true
        } else {
            !get_bitboard(&self.bitboard, x, y)
        }
    }

    #[cfg(not(feature = "bitboard"))]
    pub fn get_board_hash(&self) -> u64 {
        let mut hasher = MetroHasher::default();
        for car in self.cars.iter() {
            hasher.write_u8(car.x as u8);
            hasher.write_u8(car.y as u8);
            hasher.write_u8(car.length as u8);
            hasher.write_u8(if car.direction == Direction::Horizontal { 1 } else { 0 });
        }
        hasher.finish()
    }

    #[cfg(feature = "bitboard")]
    pub fn get_board_hash(&self) -> u64 {
        let mut hasher = MetroHasher::default();
        if cfg!(feature = "precise_hash") {
            for car in self.cars.iter() {
                hasher.write_u8(car.x as u8);
                hasher.write_u8(car.y as u8);
            }
        }
        hasher.write_u64(self.bitboard);
        hasher.finish()
    }
}

#[cfg(not(feature = "bitboard"))]
fn compute_bitboard(cars: &[Car]) -> [bool; WIDTH * HEIGHT] {
    let mut res = [false; WIDTH * HEIGHT];
    for car in cars.iter() {
        for l in 0..car.length {
            let (x, y) = car.get_coords(l);
            res[x + y * WIDTH] = true;
        }
    }
    res
}

#[cfg(feature = "bitboard")]
fn compute_bitboard(cars: &[Car]) -> u64 {
    let mut res = 0;

    for car in cars.iter() {
        for l in 0..car.length {
            let (x, y) = car.get_coords(l);
            res |= 1 << (x + y * WIDTH);
        }
    }

    res
}

#[cfg(not(feature = "bitboard"))]
#[inline]
fn set_bitboard(bitboard: &mut [bool; WIDTH * HEIGHT], x: usize, y: usize, value: bool) {
    bitboard[x + y * WIDTH] = value;
}

#[cfg(feature = "bitboard")]
#[inline]
fn set_bitboard(bitboard: &mut u64, x: usize, y: usize, value: bool) {
    let position = 1 << (x + y * WIDTH);
    if value {
        *bitboard |= position;
    } else {
        *bitboard &= !position;
    }
}

#[cfg(not(feature = "bitboard"))]
#[inline]
fn get_bitboard(bitboard: &[bool; WIDTH * HEIGHT], x: usize, y: usize) -> bool {
    bitboard[x + y * WIDTH]
}


#[cfg(feature = "bitboard")]
#[inline]
fn get_bitboard(bitboard: &u64, x: usize, y: usize) -> bool {
    let position = 1 << (x + y * WIDTH);
    bitboard & position != 0
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if get_bitboard(&self.bitboard, x, y) {
                    for (index, car) in self.cars.iter().enumerate() {
                        if car.covers(x, y) {
                            if let Some(d) = std::char::from_digit(index as u32, 36) {
                                write!(f, "{}", d)?;
                            } else {
                                write!(f, "?")?;
                            }
                            break;
                        }
                    }
                } else {
                    write!(f, " ")?;
                }
            }
            if y != HEIGHT - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
