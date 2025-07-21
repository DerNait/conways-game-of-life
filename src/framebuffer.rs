use raylib::prelude::*;
use raylib::ffi::GetImageColor;
use raylib::prelude::Color;

use std::slice;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub color_buffer: Image,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width, height, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x < self.width && y < self.height {
            Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.current_color);
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str) {
        Image::export_image(&self.color_buffer, file_path);
    }

    // framebuffer.rs (nuevo get_color)
	pub fn get_color(&self, x: i32, y: i32) -> Color {
		if x >= 0 && y >= 0 && x < self.width && y < self.height {
			let raw_image = self.color_buffer.as_ref(); // &ffi::Image
			let raw_color = unsafe {
				GetImageColor(*raw_image, x, y) // desreferencia el puntero
			};
			Color::from(raw_color) // convierte ffi::Color → raylib::Color
		} else {
			self.background_color
		}
	}

	/// Devuelve una *vista* de todos los bytes RGBA que hay dentro del Image.
	/// (4 bytes por píxel, orden RGBA‑8888)
	pub fn as_bytes(&self) -> &[u8] {
		unsafe {
			let raw = self.color_buffer.as_ref();          // &ffi::Image
			slice::from_raw_parts(
				raw.data as *const u8,                     // puntero a los pixeles
				(self.width * self.height * 4) as usize,   // len en bytes
			)
		}
	}
}
