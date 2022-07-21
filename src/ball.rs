use sdl2::{rect::{Rect, Point}, render::WindowCanvas};

use crate::paddle::Paddle;

const SIZE: u32 = 16;
pub const SPEED: f32 = 500.0;

pub struct Ball {
	origin: Point,
	out_of_bounds: bool,
	pos_offset: Point,
	pub pos_bounds: Rect,
}

impl Ball {
	pub fn new(x: i32, y: i32) -> Self {
		Self {
			origin: Point::new(x, y),
			out_of_bounds: false,
			pos_offset: Point::new(0, 0),
			pos_bounds: Rect::new(x, y, SIZE, SIZE),
		}
	}

	// Note: These numbers are not absolute positions, they're relative
	pub fn move_offset(&mut self, x: i32, y: i32) {
		self.pos_offset.x = x;
		self.pos_offset.y = y;
	}

	pub fn flip_x(&mut self) {
		self.pos_offset.x *= -1;
	}

	pub fn flip_y(&mut self) {
		self.pos_offset.y *= -1;
	}

	pub fn tick(&mut self, delta_time: f32, window_size: (i32, i32)) {
		if self.out_of_bounds {
			return;
		}

		// Note: current_pos is used as an offset rather than absolute position
		let mut new_x = (self.pos_offset.x as f32 * delta_time * SPEED) as i32;
		let mut new_y = (self.pos_offset.y as f32 * delta_time * SPEED) as i32;
		
		// let pos_x = self.pos_bounds.x;
		let pos_x = self.pos_bounds.x;
		let pos_y = self.pos_bounds.y;

		// Don't allow the ball offscreen, it needs to flip its position to "bounce"
		if pos_y + new_y <= 0 {
			new_y = -new_y;
			self.pos_offset.y *= -1;
		}

		// FIXME
		if pos_x + new_x <= 0 || pos_x + new_x >= window_size.0 - SIZE as i32 {
			new_x = -new_x;
			self.pos_offset.x *= -1;
		}

		self.pos_bounds.offset(new_x, new_y);

		// Out of bounds
		if pos_y + new_y >= window_size.1 - SIZE as i32 {
			self.out_of_bounds = true;
			self.move_offset(self.pos_offset.x * -1, -1);
		}
	}

	pub fn is_out_of_bounds(&self) -> bool {
		self.out_of_bounds
	}

	pub fn reset(&mut self) {
		self.out_of_bounds = false;
		self.pos_bounds.reposition(self.origin);
	}

	pub fn check_collision(&mut self, paddle: &Paddle) {
		if let Some(intersection) = self.pos_bounds.intersection(paddle.pos_bounds) {
			self.pos_offset.y *= -1;

			// Hitting sides of paddle
			if intersection.x >= paddle.pos_bounds.x && intersection.x <= paddle.pos_bounds.x as i32 + 8 {
				self.pos_offset.x *= -1;
			}
			else if intersection.x >= (paddle.pos_bounds.x + paddle.pos_bounds.width() as i32) - 8 &&
				intersection.x <= paddle.pos_bounds.x + paddle.pos_bounds.width() as i32 {
				self.pos_offset.x *= -1;
			}
		}
	}

	pub fn render(&self, canvas: &mut WindowCanvas) {
		canvas.fill_rect(self.pos_bounds).unwrap();
	}
}