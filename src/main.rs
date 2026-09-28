mod game;

use game::*;
use macroquad::prelude::*;
use std::fs;

const CELL_SIZE: f32 = 30.0;
const BEST_SCORE_FILE: &str = "best_score.txt";

fn load_best_score() -> i32 {
    fs::read_to_string(BEST_SCORE_FILE)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

fn save_best_score(score: i32) {
    let _ = fs::write(BEST_SCORE_FILE, score.to_string());
}

fn handle_input(g: &mut Game) {
    if is_key_pressed(KeyCode::Up) {
        g.queue_direction(Direction::Up);
    }
    if is_key_pressed(KeyCode::Down) {
        g.queue_direction(Direction::Down);
    }
    if is_key_pressed(KeyCode::Left) {
        g.queue_direction(Direction::Left);
    }
    if is_key_pressed(KeyCode::Right) {
        g.queue_direction(Direction::Right);
    }
    if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
        g.toggle_pause();
    }
    if is_key_pressed(KeyCode::Space) && g.game_over {
        let best = g.best_score;
        *g = Game::new(best);
    }
}

fn draw_game(g: &Game) {
    clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

    // Grid lines
    let grid_color = Color::new(0.15, 0.15, 0.2, 1.0);
    for i in 0..=GRID_WIDTH {
        let x = i as f32 * CELL_SIZE;
        draw_line(x, 0.0, x, GRID_HEIGHT as f32 * CELL_SIZE, 1.0, grid_color);
    }
    for i in 0..=GRID_HEIGHT {
        let y = i as f32 * CELL_SIZE;
        draw_line(0.0, y, GRID_WIDTH as f32 * CELL_SIZE, y, 1.0, grid_color);
    }

    // Food
    let food_x = g.food.x as f32 * CELL_SIZE + 2.0;
    let food_y = g.food.y as f32 * CELL_SIZE + 2.0;
    let food_size = CELL_SIZE - 4.0;
    draw_rectangle(food_x, food_y, food_size, food_size, Color::new(0.9, 0.2, 0.2, 1.0));

    // Snake
    for (i, segment) in g.snake.iter().enumerate() {
        let sx = segment.x as f32 * CELL_SIZE + 1.0;
        let sy = segment.y as f32 * CELL_SIZE + 1.0;
        let s = CELL_SIZE - 2.0;
        let color = if i == 0 {
            Color::new(0.3, 0.9, 0.3, 1.0)
        } else {
            Color::new(0.2, 0.7, 0.2, 1.0)
        };
        draw_rectangle(sx, sy, s, s, color);
    }

    // Score
    let score_text = format!("Score: {}", g.score);
    draw_text(&score_text, 10.0, GRID_HEIGHT as f32 * CELL_SIZE + 30.0, 24.0, WHITE);

    let best_text = format!("Best: {}", g.best_score);
    draw_text(&best_text, 10.0, GRID_HEIGHT as f32 * CELL_SIZE + 55.0, 20.0, Color::new(0.7, 0.7, 0.7, 1.0));

    let controls = "Arrows: move | P/Esc: pause";
    draw_text(controls, 10.0, GRID_HEIGHT as f32 * CELL_SIZE + 80.0, 16.0, Color::new(0.5, 0.5, 0.5, 1.0));

    // Pause overlay
    if g.paused && !g.game_over {
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));
        let text = "PAUSED";
        let w = measure_text(text, None, 48, 1.0).width;
        draw_text(text, (screen_width() - w) / 2.0, screen_height() / 2.0, 48.0, WHITE);
        let sub = "Press P or Esc to resume";
        let sw = measure_text(sub, None, 20, 1.0).width;
        draw_text(sub, (screen_width() - sw) / 2.0, screen_height() / 2.0 + 40.0, 20.0, Color::new(0.7, 0.7, 0.7, 1.0));
    }

    // Game over overlay
    if g.game_over {
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.7));

        let go_text = "GAME OVER";
        let go_width = measure_text(go_text, None, 48, 1.0).width;
        draw_text(go_text, (screen_width() - go_width) / 2.0, screen_height() / 2.0 - 40.0, 48.0, Color::new(0.9, 0.2, 0.2, 1.0));

        let final_score = format!("Final Score: {}", g.score);
        let fs_width = measure_text(&final_score, None, 28, 1.0).width;
        draw_text(&final_score, (screen_width() - fs_width) / 2.0, screen_height() / 2.0 + 10.0, 28.0, WHITE);

        let restart = "Press SPACE to restart";
        let r_width = measure_text(restart, None, 20, 1.0).width;
        draw_text(restart, (screen_width() - r_width) / 2.0, screen_height() / 2.0 + 50.0, 20.0, Color::new(0.7, 0.7, 0.7, 1.0));
    }
}

#[macroquad::main("Snake")]
async fn main() {
    let mut game = Game::new(load_best_score());

    loop {
        let dt = get_frame_time();

        handle_input(&mut game);
        game.update(dt);

        if game.game_over && game.score > game.best_score {
            save_best_score(game.score);
            game.best_score = game.score;
        }

        draw_game(&game);

        next_frame().await;
    }
}
