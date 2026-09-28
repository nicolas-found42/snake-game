// Pure game logic — no graphics, no input. This is what we test.

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub const GRID_WIDTH: i32 = 20;
pub const GRID_HEIGHT: i32 = 20;
pub const INITIAL_SPEED: f32 = 0.15;
pub const SPEED_INCREMENT: f32 = 0.005;
pub const MIN_SPEED: f32 = 0.05;

pub struct Game {
    pub snake: Vec<Point>,
    pub direction: Direction,
    pub pending_direction: Direction,
    pub food: Point,
    pub score: i32,
    pub best_score: i32,
    pub speed: f32,
    pub move_timer: f32,
    pub game_over: bool,
    pub paused: bool,
}

impl Game {
    pub fn new(best_score: i32) -> Self {
        let mut game = Game {
            snake: vec![
                Point { x: GRID_WIDTH / 2, y: GRID_HEIGHT / 2 },
                Point { x: GRID_WIDTH / 2 - 1, y: GRID_HEIGHT / 2 },
                Point { x: GRID_WIDTH / 2 - 2, y: GRID_HEIGHT / 2 },
            ],
            direction: Direction::Right,
            pending_direction: Direction::Right,
            food: Point { x: 0, y: 0 },
            score: 0,
            best_score,
            speed: INITIAL_SPEED,
            move_timer: 0.0,
            game_over: false,
            paused: false,
        };
        game.spawn_food();
        game
    }

    pub fn spawn_food(&mut self) {
        loop {
            let x = rand::random::<i32>().rem_euclid(GRID_WIDTH);
            let y = rand::random::<i32>().rem_euclid(GRID_HEIGHT);
            let candidate = Point { x, y };
            if !self.snake.contains(&candidate) {
                self.food = candidate;
                break;
            }
        }
    }

    pub fn queue_direction(&mut self, dir: Direction) {
        // Prevent 180-degree turns
        if dir != self.direction.opposite() {
            self.pending_direction = dir;
        }
    }

    pub fn toggle_pause(&mut self) {
        if !self.game_over {
            self.paused = !self.paused;
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.game_over || self.paused {
            return;
        }

        self.move_timer += dt;

        if self.move_timer >= self.speed {
            self.move_timer = 0.0;
            self.direction = self.pending_direction;

            let head = self.snake[0];
            let new_head = match self.direction {
                Direction::Up => Point { x: head.x, y: head.y - 1 },
                Direction::Down => Point { x: head.x, y: head.y + 1 },
                Direction::Left => Point { x: head.x - 1, y: head.y },
                Direction::Right => Point { x: head.x + 1, y: head.y },
            };

            // Wall collision = death
            if new_head.x < 0 || new_head.x >= GRID_WIDTH || new_head.y < 0 || new_head.y >= GRID_HEIGHT {
                self.game_over = true;
                return;
            }

            // Self collision = death
            // Check against body (excluding tail if it will move)
            let ate_food = new_head == self.food;
            let body_to_check = if ate_food {
                &self.snake[..]
            } else {
                &self.snake[..self.snake.len() - 1]
            };
            if body_to_check.contains(&new_head) {
                self.game_over = true;
                return;
            }

            self.snake.insert(0, new_head);

            if ate_food {
                self.score += 1;
                self.speed = (self.speed - SPEED_INCREMENT).max(MIN_SPEED);
                self.spawn_food();
            } else {
                self.snake.pop();
            }
        }
    }

    pub fn save_best_score(&self) -> Option<i32> {
        if self.score > self.best_score {
            Some(self.score)
        } else {
            None
        }
    }
}

// ---------- TESTS ----------
// These run with `cargo test` — no window, no graphics, just logic.

#[cfg(test)]
mod tests {
    use super::*;

    fn game_at_position(px: i32, py: i32) -> Game {
        let mut g = Game::new(0);
        g.snake = vec![
            Point { x: px, y: py },
            Point { x: px - 1, y: py },
            Point { x: px - 2, y: py },
        ];
        g.direction = Direction::Right;
        g.pending_direction = Direction::Right;
        g.food = Point { x: px + 5, y: py }; // food far away
        g
    }

    #[test]
    fn snake_starts_with_three_segments() {
        let g = Game::new(0);
        assert_eq!(g.snake.len(), 3);
    }

    #[test]
    fn snake_moves_right() {
        let mut g = game_at_position(5, 10);
        g.speed = 0.0; // make it move immediately
        g.update(0.01);
        assert_eq!(g.snake[0], Point { x: 6, y: 10 });
    }

    #[test]
    fn snake_moves_up() {
        let mut g = game_at_position(5, 10);
        g.direction = Direction::Up;
        g.pending_direction = Direction::Up;
        g.speed = 0.0;
        g.update(0.01);
        assert_eq!(g.snake[0], Point { x: 5, y: 9 });
    }

    #[test]
    fn snake_cannot_reverse_into_itself() {
        let mut g = game_at_position(5, 10);
        g.direction = Direction::Right;
        g.queue_direction(Direction::Left); // try to reverse
        g.speed = 0.0;
        g.update(0.01);
        // Should still be moving right, not left
        assert_eq!(g.snake[0], Point { x: 6, y: 10 });
    }

    #[test]
    fn eating_food_grows_snake() {
        let mut g = game_at_position(5, 10);
        g.food = Point { x: 6, y: 10 }; // food right in front
        g.speed = 0.0;
        g.update(0.01);
        assert_eq!(g.snake.len(), 4); // grew by 1
        assert_eq!(g.score, 1);
    }

    #[test]
    fn eating_food_increases_speed() {
        let mut g = game_at_position(5, 10);
        g.food = Point { x: 6, y: 10 };
        let old_speed = g.speed;
        g.speed = 0.0;
        g.update(0.01);
        assert!(g.speed < old_speed, "speed should decrease (faster) after eating");
    }

    #[test]
    fn hitting_wall_kills_snake() {
        let mut g = game_at_position(GRID_WIDTH - 1, 10); // at right edge
        g.speed = 0.0;
        g.update(0.01);
        assert!(g.game_over, "snake should die hitting the wall");
    }

    #[test]
    fn hitting_top_wall_kills_snake() {
        let mut g = game_at_position(5, 0); // at top edge
        g.direction = Direction::Up;
        g.pending_direction = Direction::Up;
        g.speed = 0.0;
        g.update(0.01);
        assert!(g.game_over);
    }

    #[test]
    fn hitting_self_kills_snake() {
        // Create a snake that will run into itself
        let mut g = Game::new(0);
        g.snake = vec![
            Point { x: 5, y: 5 },
            Point { x: 5, y: 6 },
            Point { x: 6, y: 6 },
            Point { x: 6, y: 5 },
            Point { x: 6, y: 4 },
            Point { x: 5, y: 4 },
            Point { x: 4, y: 4 },
            Point { x: 4, y: 5 },
        ];
        g.direction = Direction::Right;
        g.pending_direction = Direction::Right;
        g.food = Point { x: 10, y: 10 }; // far away
        g.speed = 0.0;
        g.update(0.01);
        // Moving right from (5,5) into (6,5) which is in the body
        assert!(g.game_over, "snake should die hitting itself");
    }

    #[test]
    fn score_increments_on_food() {
        let mut g = game_at_position(5, 10);
        g.food = Point { x: 6, y: 10 };
        g.speed = 0.0;
        g.update(0.01);
        assert_eq!(g.score, 1);
    }

    #[test]
    fn best_score_saved_when_beaten() {
        let mut g = Game::new(5);
        g.score = 10;
        assert_eq!(g.save_best_score(), Some(10));
    }

    #[test]
    fn best_score_not_saved_when_not_beaten() {
        let mut g = Game::new(10);
        g.score = 5;
        assert_eq!(g.save_best_score(), None);
    }

    #[test]
    fn pause_stops_movement() {
        let mut g = game_at_position(5, 10);
        g.paused = true;
        g.speed = 0.0;
        g.update(0.01);
        assert_eq!(g.snake[0], Point { x: 5, y: 10 }); // didn't move
    }

    #[test]
    fn toggle_pause_works() {
        let mut g = game_at_position(5, 10);
        assert!(!g.paused);
        g.toggle_pause();
        assert!(g.paused);
        g.toggle_pause();
        assert!(!g.paused);
    }

    #[test]
    fn food_spawns_not_on_snake() {
        let mut g = Game::new(0);
        for _ in 0..100 {
            g.spawn_food();
            assert!(!g.snake.contains(&g.food), "food spawned on snake body");
        }
    }

    #[test]
    fn speed_never_goes_below_minimum() {
        let mut g = game_at_position(5, 10);
        g.speed = MIN_SPEED - 0.001;
        g.food = Point { x: 6, y: 10 };
        g.speed = 0.0;
        g.update(0.01);
        assert!(g.speed >= MIN_SPEED, "speed should not go below minimum");
    }
}
