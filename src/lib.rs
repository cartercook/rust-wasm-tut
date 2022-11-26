mod utils;

extern crate js_sys;
extern crate fixedbitset;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
  Dead = 0,
  Alive = 1
}

#[wasm_bindgen]
pub struct Universe {
  width: u16,
  height: u16,
  cells: FixedBitSet
}

/// Public methods, exported to Javascript
#[wasm_bindgen]
impl Universe {
  // PUBLIC
  #[allow(clippy::new_without_default)]
  pub fn new() -> Universe {
    utils::set_panic_hook();

    const WIDTH: u16 = 64;
    const HEIGHT: u16 = 64;
    const SIZE: u16 = WIDTH * HEIGHT;

    let mut cells: FixedBitSet = FixedBitSet::with_capacity(SIZE as usize);
    for i in 0..SIZE {
      cells.set(i as usize, js_sys::Math::random() > 0.5);
    }

    Universe { width: WIDTH, height: HEIGHT, cells }
  }

  /// Exposed for JS
  pub fn width(&self) -> u16 {
    self.width
  }

  /// Exposed for JS
  pub fn height(&self) -> u16 {
    self.height
  }

  /// Exposed for JS
  pub fn cells(&self) -> *const u32 {
    self.cells.as_slice().as_ptr()
  }

  pub fn tick(&mut self) {
    let mut next = self.cells.clone();

    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_index(row, col);
        let cell = self.cells[idx];
        let live_neighbors = self.live_neighbor_count(row, col);

        next.set(idx, match (cell, live_neighbors) {
          (true, x) if x < 2 => false,
          (true, 2) | (true, 3) => true,
          (true, x) if x > 3 => false,
          (false, 3) => true,
          (otherwise, _) => otherwise
        });
      }
    }

    self.cells = next;
  }




  // PRIVATE

  fn get_index(&self, row: u16, column: u16) -> usize {
    (row * self.width + column) as usize
  }

  fn live_neighbor_count(&self, row: u16, column: u16) -> u8 {
    let mut count = 0;
    
    for &delta_row in [self.height - 1, 0, 1].iter() {
      for &delta_col in [self.width - 1, 0, 1].iter() {
        if delta_row == 0 && delta_col == 0 {
          continue;
        }

        let neighbor_row = (row + delta_row) % self.height;
        let neightbor_col = (column + delta_col) % self.width;
        let idx = self.get_index(neighbor_row, neightbor_col);
        count += self.cells[idx] as u8;
      }
    }

    count
  }
}