use sdl2::{rect::Rect, render::WindowCanvas, pixels::Color};

use crate::ball::Ball;

pub const WIDTH: u32 = 128;
pub const HEIGHT: u32 = 32;
const INTERSECT: i32 = 8;
const ONE_COL: Color = Color::WHITE;
const TWO_COL: Color = Color::RGB(120, 120, 120);
const THREE_COL: Color = Color::RGB(50, 50, 50);

#[derive(Clone, Copy)]
pub struct Brick {
	hitpoints: i8,
	pub pos_bounds: Rect,
}

impl Default for Brick {
    fn default() -> Self {
        Self { hitpoints: Default::default(), pos_bounds: Rect::new(0, 0, 0, 0) }
    }
}

impl Brick {
	pub fn new(hitpoints: i8, x: i32, y: i32) -> Self {
		Self {
			hitpoints,
			pos_bounds: Rect::new(x, y, WIDTH, HEIGHT),
		}
	}

	pub fn check_collision(&mut self, ball: &mut Ball) -> i32 {
		if !self.is_alive() {
			return 0;
		}

		if let Some(intersection) = self.pos_bounds.intersection(ball.pos_bounds) {
			// TODO: Not janky collisions
			// Hitting sides of ball
			if intersection.x >= self.pos_bounds.x && intersection.x <= self.pos_bounds.x as i32 + INTERSECT {
				ball.flip_x();
			}
			else if intersection.x >= (self.pos_bounds.x + WIDTH as i32) - INTERSECT &&
				intersection.x <= self.pos_bounds.x + WIDTH as i32 {
				ball.flip_x();
			}
			
			if intersection.y >= self.pos_bounds.y && intersection.y <= self.pos_bounds.y as i32 + INTERSECT {
				ball.flip_y();
			}
			else if intersection.y >= (self.pos_bounds.y + HEIGHT as i32) - INTERSECT &&
				intersection.x <= self.pos_bounds.x + HEIGHT as i32 {
				ball.flip_y();
			}

			// Damage the brick
			self.hitpoints -= 1;
			return 1;
		}

		0
	}

	pub fn is_alive(&self) -> bool {
		self.hitpoints > 0
	}

	pub fn render(&self, canvas: &mut WindowCanvas) {
		match self.hitpoints {
			1 => canvas.set_draw_color(ONE_COL),
			2 => canvas.set_draw_color(TWO_COL),
			3 => canvas.set_draw_color(THREE_COL),
			_ => {},
		}
		 
		canvas.fill_rect(self.pos_bounds).unwrap();
	}
}