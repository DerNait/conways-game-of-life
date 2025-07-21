use crate::framebuffer;

use raylib::prelude::*;
use framebuffer::Framebuffer;


pub struct GameOfLife {
    width: i32,
    height: i32,
    board: Vec<bool>,       // fila mayor en X, fila*width + col
}

impl GameOfLife {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            board: vec![false; (width * height) as usize],
        }
    }

    fn idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn set_alive(&mut self, x: i32, y: i32, alive: bool) {
		let idx = self.idx(x, y);
		self.board[idx] = alive;
	}


    pub fn is_alive(&self, x: i32, y: i32) -> bool {
        self.board[self.idx(x, y)]
    }

    /// Cuenta los 8 vecinos en un mundo toroidal (los bordes se conectan)
	fn live_neighbors(&self, x: i32, y: i32) -> u8 {
		let mut count = 0;
		for dy in -1..=1 {
			for dx in -1..=1 {
				if dx == 0 && dy == 0 { continue; }

				// ¡envuelve!  ( (coord + desplazamiento + tamaño) % tamaño )
				let nx = (x + dx + self.width)  % self.width;
				let ny = (y + dy + self.height) % self.height;

				if self.is_alive(nx, ny) { count += 1; }
			}
		}
		count
	}

    pub fn step(&mut self) {
        let mut next = self.board.clone();   // copia actual
        for y in 0..self.height {
            for x in 0..self.width {
                let alive = self.is_alive(x, y);
                let n = self.live_neighbors(x, y);
                let idx = self.idx(x, y);
                next[idx] = match (alive, n) {
                    (true, 2) | (true, 3) => true,        // supervivencia
                    (false, 3)            => true,        // reproducción
                    _                     => false,       // muerte
                };
            }
        }
        self.board = next;
    }

    /// Dibuja el estado actual en el framebuffer
    pub fn draw(&self, fb: &mut Framebuffer) {
      for y in 0..self.height {
          for x in 0..self.width {
              if self.is_alive(x, y) {
                  fb.set_current_color(Color::WHITE);
              } else {
                  fb.set_current_color(Color::BLACK);
              }
              fb.set_pixel(x, y);
          }
      }
  }
}
