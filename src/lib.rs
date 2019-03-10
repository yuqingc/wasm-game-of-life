extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use std::fmt;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbor_count = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbor_count) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width*height)
            .map(|i| {
                if i%2 == 0 || i%7 ==0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_init_universe() -> Universe {

        let mut cells: Vec<Cell> = vec![Cell::Dead; 25];
        cells[6] = Cell::Alive;
        cells[11] = Cell::Alive;
        cells[16] = Cell::Alive;

        let universe = Universe {
            width: 5,
            height: 5,
            cells,
        };
        universe
    }

    #[test]
    fn it_should_work() {
        assert_eq!(0, 0);
    }

    #[test]
    fn it_should_get_index() {
        let uni = get_init_universe();
        assert_eq!(uni.get_index(1, 1), 6);
        assert_eq!(uni.get_index(2, 1), 11);
        assert_eq!(uni.get_index(3, 1), 16);
    }

    #[test]
    fn it_should_count_alive_neighbors() {
        let uni = get_init_universe();
        assert_eq!(uni.live_neighbor_count(0, 0), 1);
        assert_eq!(uni.live_neighbor_count(0, 1), 1);
        assert_eq!(uni.live_neighbor_count(1, 0), 2);
        assert_eq!(uni.live_neighbor_count(2, 1), 2);
        assert_eq!(uni.live_neighbor_count(2, 0), 3);
        assert_eq!(uni.live_neighbor_count(2, 2), 3);
        assert_eq!(uni.live_neighbor_count(3, 3), 0);
        assert_eq!(uni.live_neighbor_count(4, 1), 1);
    }

    #[test]
    fn it_should_change_after_tick() {
        let mut uni = get_init_universe();
        assert_eq!(uni.cells[uni.get_index(1, 1)], Cell::Alive);
        assert_eq!(uni.cells[uni.get_index(2, 1)], Cell::Alive);
        assert_eq!(uni.cells[uni.get_index(3, 1)], Cell::Alive);
        assert_eq!(uni.cells[uni.get_index(2, 0)], Cell::Dead);
        assert_eq!(uni.cells[uni.get_index(2, 2)], Cell::Dead);
        assert_eq!(uni.cells[uni.get_index(4, 2)], Cell::Dead);
        uni.tick();
        assert_eq!(uni.cells[uni.get_index(1, 1)], Cell::Dead);
        assert_eq!(uni.cells[uni.get_index(2, 1)], Cell::Alive);
        assert_eq!(uni.cells[uni.get_index(3, 1)], Cell::Dead);
        assert_eq!(uni.cells[uni.get_index(2, 0)], Cell::Alive);
        assert_eq!(uni.cells[uni.get_index(2, 2)], Cell::Alive);
        assert_eq!(uni.cells[uni.get_index(4, 2)], Cell::Dead);
    }

    #[test]
    fn it_should_render() {
        let uni = get_init_universe();
        let mut expected_str = String::from("");
        for row in 0..5 {
            for col in 0..5 {
                match (row, col) {
                    (1,1)|(2,1)|(3,1) => {
                        expected_str.push('◼');
                    },
                    _ => {
                        expected_str.push('◻');
                    },
                }
                if col==4{
                    expected_str.push('\n');
                }
            }
        }
        assert_eq!(uni.to_string(), expected_str);
    }
}
