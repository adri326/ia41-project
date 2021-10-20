use super::{WIDTH, HEIGHT};
use super::car::{Car, CarIter, CarMove};

#[derive(Clone)]
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
}
