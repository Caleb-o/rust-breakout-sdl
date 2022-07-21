use sdl2::{rect::Rect, render::WindowCanvas};

pub const WIDTH: u32 = 160;
pub const HEIGHT: u32 = 24;
pub const SPEED: f32 = 800.0;

pub struct Paddle {
	x_offset: i32,
	pub pos_bounds: Rect,
}

impl Paddle {
	pub fn new(x: i32, y: i32) -> Self {
		Self {
			x_offset: 0,
			pos_bounds: Rect::new(x, y, WIDTH, HEIGHT),
		}
	}

	// Note: These numbers are not absolute positions, they're relative
	pub fn move_offset(&mut self, offset: i32) {
		self.x_offset = offset;
	}

	pub fn tick(&mut self, delta_time: f32, window_x: i32) {
		// Note: current_pos is used as an offset rather than absolute position
		let new_x = (self.x_offset as f32 * delta_time * SPEED) as i32;
		let pos_x = self.pos_bounds.x;

		// Don't allow the paddle offscreen
		if pos_x + new_x > 0 && new_x + pos_x < window_x - WIDTH as i32 {
			self.pos_bounds.offset(new_x, 0);
		}
	}

	pub fn render(&self, canvas: &mut WindowCanvas) {
		canvas.fill_rect(self.pos_bounds).unwrap();
	}
}