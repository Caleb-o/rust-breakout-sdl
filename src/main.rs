mod paddle;
mod ball;
mod brick;

extern crate sdl2;

use ball::Ball;
use brick::Brick;
use paddle::Paddle;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

const FPS_CAP: u32 = 60;
const FRAME_DURATION: u32 = 1_000_000_000;
const FONT_SIZE: i32 = 64;

const WIDTH: u32 = 1080;
const HEIGHT: u32 = 720;
const BRICK_COUNT: usize = 30;
const BRICK_SPACING: i32 = 16;

// SDL Util because I don't want unsafe other places
fn sdl_get_ticks() -> u32 {
    unsafe {
        sdl2::sys::SDL_GetTicks()
    }
}

fn create_bricks(bricks: &mut [Brick]) -> i32 {
    let mut y = 32;
    let mut r = rand::thread_rng();
    let mut total_score = 0i32;

    // This is the big dumb
    for i in 0..bricks.len() {
        if i % 6 == 0 {
            y += BRICK_SPACING + brick::HEIGHT as i32;
        }

        let score = r.gen_range(1..4);
        total_score += score;

        bricks[i] = Brick::new(
            score as i8, // FIXME: Add more hitpoints to some bricks, probably needs a colour
            BRICK_SPACING * 7 + (i as i32 % 6 * (brick::WIDTH as i32 + BRICK_SPACING)),
            y
        ); 
    }

    total_score
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().expect("TTF could not initialise");
    
    let window = video_subsystem.window("Rust Breakout", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    // NOTE: To switch between VSync and a fixed/target FPS, I need to recreate the canvas
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player_score = 0i32;
    let mut player_lives = 3i32;

    let font = ttf_context.load_font("assets/Hack-Regular.ttf", FONT_SIZE as u16).unwrap();
    let mut player_score_tex = font
        .render("0")
        .blended(Color::WHITE)
        .unwrap()
        .as_texture(&texture_creator)
        .unwrap();

    let mut player_lives_tex = font
        .render(format!("lives: {}", player_lives).as_str())
        .blended(Color::WHITE)
        .unwrap()
        .as_texture(&texture_creator)
        .unwrap();

    // Delta time
    let mut last_update = sdl_get_ticks();

    let mut player_paddle = Paddle::new(((WIDTH / 2) - paddle::WIDTH / 2) as i32, (HEIGHT - 32) as i32 - (paddle::HEIGHT / 2) as i32);

    let mut ball = Ball::new((WIDTH / 2) as i32, (HEIGHT / 2) as i32 + 128);
    ball.move_offset(-1, -1);

    let mut bricks: [Brick; BRICK_COUNT] = [Brick::default(); BRICK_COUNT];

    let total_score = create_bricks(&mut bricks);

    'main_loop: while player_score < total_score {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // To draw the paddles and ball, and it's easier to do it here rather than in each draw
        canvas.set_draw_color(Color::WHITE);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_loop
                },

                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
                    player_paddle.move_offset(-1);
                }
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
                    player_paddle.move_offset(1);
                }

                // FIXME: Stop sticky movement, when quickly swapping direction
                Event::KeyUp { keycode: Some(Keycode::A), .. } |
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                    player_paddle.move_offset(0);
                }
                _ => {}
            }
        }

        let current_time = sdl_get_ticks();
        // Delta time in seconds
        let delta_time = (current_time - last_update) as f32 / 1000.0f32;
        last_update = current_time;

        if ball.is_out_of_bounds() {
            // TODO: Remove life
            ball.reset();

            player_lives -= 1;

            if player_lives > 0 {
                player_lives_tex = font
                    .render(format!("lives: {}", player_lives).as_str())
                    .blended(Color::WHITE)
                    .unwrap()
                    .as_texture(&texture_creator)
                    .unwrap();
            } else {
                player_lives_tex = font
                    .render("Game Over!")
                    .blended(Color::WHITE)
                    .unwrap()
                    .as_texture(&texture_creator)
                    .unwrap();
                
                // Game Over end screen hack
                canvas.copy(&player_lives_tex, None, Some(Rect::new(
                    (WIDTH / 2) as i32 - ((FONT_SIZE / 2) * 6),
                    (HEIGHT / 2) as i32 - ((FONT_SIZE / 2) * 3),
                    320,
                    128)
                )).unwrap();
                canvas.present();
            }
        }

        // HACK
        if player_lives == 0 {
            continue;
        }

        // Tick here
        player_paddle.tick(delta_time, WIDTH as i32);

        // Dumb structuring but it's fine :^)
        ball.tick(delta_time, (WIDTH as i32, HEIGHT as i32));
        // TODO: Check collisions with bricks
        ball.check_collision(&player_paddle);

        let score_value = bricks.iter_mut()
            .filter(|b| b.is_alive())
            .fold(0, |mut acc, b| { acc += b.check_collision(&mut ball); acc });

        if score_value > 0 {
            player_score += score_value;

            player_score_tex = font
                .render(format!("{}", player_score).as_str())
                .blended(Color::WHITE)
                .unwrap()
                .as_texture(&texture_creator)
                .unwrap();
        }

        // Render here
        player_paddle.render(&mut canvas);
        ball.render(&mut canvas);

        bricks.iter()
            .filter(|b| b.is_alive())
            .for_each(|b| b.render(&mut canvas));

        // -- Score
        canvas.copy(&player_score_tex, None, Some(Rect::new((WIDTH / 2) as i32 - (FONT_SIZE / 2), 8, 64, 64))).unwrap();
        canvas.copy(&player_lives_tex, None, Some(Rect::new(32, 8, 150, 64))).unwrap();
        
        canvas.present();
        std::thread::sleep(Duration::new(0, FRAME_DURATION / FPS_CAP));
    }
}