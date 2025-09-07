use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const BALL_RADIUS: f32 = 8.0;
const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 15.0;
const PADDLE_SPEED: f32 = 400.0;
const BRICK_WIDTH: f32 = 75.0;
const BRICK_HEIGHT: f32 = 25.0;
const BRICK_ROWS: usize = 8;
const BRICK_COLS: usize = 10;

#[derive(Clone, Copy)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self { x: self.x / len, y: self.y / len }
        } else {
            *self
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

struct Ball {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    trail: Vec<Vec2>,
}

impl Ball {
    fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(200.0, -250.0),
            radius: BALL_RADIUS,
            trail: Vec::new(),
        }
    }

    fn update(&mut self, dt: f32) {
        self.position = self.position + self.velocity * dt;

        // Update trail
        self.trail.push(self.position);
        if self.trail.len() > 10 {
            self.trail.remove(0);
        }
    }

    fn draw(&self) {
        // Draw trail
        for (i, pos) in self.trail.iter().enumerate() {
            let alpha = (i as f32 / self.trail.len() as f32) * 0.5;
            let radius = self.radius * (0.3 + alpha * 0.7);
            draw_circle(pos.x, pos.y, radius, Color::new(1.0, 1.0, 1.0, alpha));
        }

        // Draw ball with glow effect
        draw_circle(self.position.x, self.position.y, self.radius + 2.0, Color::new(1.0, 1.0, 1.0, 0.3));
        draw_circle(self.position.x, self.position.y, self.radius, WHITE);
        draw_circle(self.position.x, self.position.y, self.radius * 0.6, Color::new(1.0, 1.0, 0.8, 0.8));
    }

    fn reset(&mut self, x: f32, y: f32) {
        self.position = Vec2::new(x, y);
        self.velocity = Vec2::new(200.0, -250.0);
        self.trail.clear();
    }
}

struct Paddle {
    position: Vec2,
    width: f32,
    height: f32,
    speed: f32,
}

impl Paddle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            speed: PADDLE_SPEED,
        }
    }

    fn update(&mut self, dt: f32) {
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.position.x -= self.speed * dt;
            if self.position.x < 0.0 {
                self.position.x = 0.0;
            }
        }

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.position.x += self.speed * dt;
            if self.position.x + self.width > SCREEN_WIDTH {
                self.position.x = SCREEN_WIDTH - self.width;
            }
        }
    }

    fn draw(&self) {
        // Draw paddle with gradient effect
        let gradient_height = 3.0;
        for i in 0..(self.height as i32) {
            let y = self.position.y + i as f32;
            let intensity = 1.0 - (i as f32 / self.height) * 0.3;
            let color = Color::new(0.2, 0.6, 1.0, intensity);
            draw_rectangle(self.position.x, y, self.width, 1.0, color);
        }

        // Draw highlight on top
        draw_rectangle(self.position.x, self.position.y, self.width, gradient_height,
                       Color::new(0.4, 0.8, 1.0, 0.8));
    }

    fn get_rect(&self) -> (f32, f32, f32, f32) {
        (self.position.x, self.position.y, self.width, self.height)
    }
}

#[derive(Clone, Copy)]
struct Brick {
    position: Vec2,
    width: f32,
    height: f32,
    destroyed: bool,
    color: Color,
    hit_animation: f32,
}

impl Brick {
    fn new(x: f32, y: f32, row: usize) -> Self {
        // Different colors for different rows
        let color = match row {
            0..=1 => RED,
            2..=3 => ORANGE,
            4..=5 => YELLOW,
            6..=7 => GREEN,
            _ => BLUE,
        };

        Self {
            position: Vec2::new(x, y),
            width: BRICK_WIDTH,
            height: BRICK_HEIGHT,
            destroyed: false,
            color,
            hit_animation: 0.0,
        }
    }

    fn update(&mut self, dt: f32) {
        if self.hit_animation > 0.0 {
            self.hit_animation -= dt * 3.0;
            if self.hit_animation <= 0.0 {
                self.destroyed = true;
            }
        }
    }

    fn hit(&mut self) {
        if !self.destroyed && self.hit_animation <= 0.0 {
            self.hit_animation = 1.0;
        }
    }

    fn draw(&self) {
        if self.destroyed && self.hit_animation <= 0.0 {
            return;
        }

        let mut color = self.color;
        let mut size_mult = 1.0;

        if self.hit_animation > 0.0 {
            // Flash white when hit
            let flash = (self.hit_animation * 10.0).sin().abs();
            color = Color::new(
                color.r + flash * (1.0 - color.r),
                color.g + flash * (1.0 - color.g),
                color.b + flash * (1.0 - color.b),
                color.a * self.hit_animation,
            );
            size_mult = 1.0 + (1.0 - self.hit_animation) * 0.2;
        }

        let draw_width = self.width * size_mult;
        let draw_height = self.height * size_mult;
        let offset_x = (self.width - draw_width) / 2.0;
        let offset_y = (self.height - draw_height) / 2.0;

        // Draw brick with border
        draw_rectangle(
            self.position.x + offset_x,
            self.position.y + offset_y,
            draw_width,
            draw_height,
            color,
        );

        // Draw border
        if self.hit_animation <= 0.0 {
            draw_rectangle_lines(
                self.position.x + offset_x,
                self.position.y + offset_y,
                draw_width,
                draw_height,
                2.0,
                Color::new(color.r * 0.7, color.g * 0.7, color.b * 0.7, 1.0),
            );
        }
    }

    fn get_rect(&self) -> (f32, f32, f32, f32) {
        (self.position.x, self.position.y, self.width, self.height)
    }
}

#[derive(PartialEq)]
enum GameState {
    Playing,
    GameOver,
    Victory,
    Paused,
}

struct Game {
    ball: Ball,
    paddle: Paddle,
    bricks: Vec<Brick>,
    state: GameState,
    score: u32,
    lives: u32,
    particles: Vec<Particle>,
}

struct Particle {
    position: Vec2,
    velocity: Vec2,
    life: f32,
    max_life: f32,
    color: Color,
}

impl Particle {
    fn new(x: f32, y: f32, color: Color) -> Self {
        let angle = rand::gen_range(0.0, 2.0 * std::f32::consts::PI);
        let speed = rand::gen_range(50.0, 150.0);

        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(angle.cos() * speed, angle.sin() * speed),
            life: rand::gen_range(0.5, 1.5),
            max_life: rand::gen_range(0.5, 1.5),
            color,
        }
    }

    fn update(&mut self, dt: f32) {
        self.position = self.position + self.velocity * dt;
        self.life -= dt;
        self.velocity.y += 200.0 * dt; // Gravity
    }

    fn draw(&self) {
        let alpha = self.life / self.max_life;
        let mut color = self.color;
        color.a = alpha;

        draw_circle(self.position.x, self.position.y, 2.0, color);
    }

    fn is_dead(&self) -> bool {
        self.life <= 0.0
    }
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            ball: Ball::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT - 100.0),
            paddle: Paddle::new(SCREEN_WIDTH / 2.0 - PADDLE_WIDTH / 2.0, SCREEN_HEIGHT - 60.0),
            bricks: Vec::new(),
            state: GameState::Playing,
            score: 0,
            lives: 3,
            particles: Vec::new(),
        };

        game.init_bricks();
        game
    }

    fn init_bricks(&mut self) {
        self.bricks.clear();
        let start_x = (SCREEN_WIDTH - (BRICK_COLS as f32 * (BRICK_WIDTH + 2.0))) / 2.0;
        let start_y = 60.0;

        for row in 0..BRICK_ROWS {
            for col in 0..BRICK_COLS {
                let x = start_x + col as f32 * (BRICK_WIDTH + 2.0);
                let y = start_y + row as f32 * (BRICK_HEIGHT + 2.0);
                self.bricks.push(Brick::new(x, y, row));
            }
        }
    }

    fn update(&mut self, dt: f32) {
        match self.state {
            GameState::Playing => {
                self.paddle.update(dt);
                self.ball.update(dt);

                // Update bricks
                for brick in &mut self.bricks {
                    brick.update(dt);
                }

                // Update particles
                for particle in &mut self.particles {
                    particle.update(dt);
                }
                self.particles.retain(|p| !p.is_dead());

                self.handle_collisions();

                // Check victory
                if self.bricks.iter().all(|b| b.destroyed || b.hit_animation > 0.0) {
                    self.state = GameState::Victory;
                }
            },
            _ => {
                // Handle restart
                if is_key_pressed(KeyCode::R) {
                    self.restart();
                }
            }
        }

        // Pause toggle
        if is_key_pressed(KeyCode::P) && self.state == GameState::Playing {
            self.state = GameState::Paused;
        } else if is_key_pressed(KeyCode::P) && self.state == GameState::Paused {
            self.state = GameState::Playing;
        }
    }

    fn handle_collisions(&mut self) {
        // Wall collisions
        if self.ball.position.x - self.ball.radius <= 0.0 {
            self.ball.position.x = self.ball.radius;
            self.ball.velocity.x = -self.ball.velocity.x;
        }
        if self.ball.position.x + self.ball.radius >= SCREEN_WIDTH {
            self.ball.position.x = SCREEN_WIDTH - self.ball.radius;
            self.ball.velocity.x = -self.ball.velocity.x;
        }
        if self.ball.position.y - self.ball.radius <= 0.0 {
            self.ball.position.y = self.ball.radius;
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        // Ball falls below screen
        if self.ball.position.y > SCREEN_HEIGHT + 50.0 {
            self.lives -= 1;
            if self.lives == 0 {
                self.state = GameState::GameOver;
            } else {
                self.ball.reset(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT - 100.0);
            }
        }

        // Paddle collision
        let (px, py, pw, ph) = self.paddle.get_rect();
        if self.ball.position.x + self.ball.radius > px &&
            self.ball.position.x - self.ball.radius < px + pw &&
            self.ball.position.y + self.ball.radius > py &&
            self.ball.position.y - self.ball.radius < py + ph &&
            self.ball.velocity.y > 0.0 {

            self.ball.velocity.y = -self.ball.velocity.y;

            // Add spin based on where ball hits paddle
            let hit_pos = (self.ball.position.x - (px + pw / 2.0)) / (pw / 2.0);
            self.ball.velocity.x += hit_pos * 100.0;

            // Limit velocity
            if self.ball.velocity.x.abs() > 300.0 {
                self.ball.velocity.x = self.ball.velocity.x.signum() * 300.0;
            }
        }

        // Brick collisions
        for brick in &mut self.bricks {
            if brick.destroyed || brick.hit_animation > 0.0 {
                continue;
            }

            let (bx, by, bw, bh) = brick.get_rect();
            if self.ball.position.x + self.ball.radius > bx &&
                self.ball.position.x - self.ball.radius < bx + bw &&
                self.ball.position.y + self.ball.radius > by &&
                self.ball.position.y - self.ball.radius < by + bh {

                brick.hit();
                self.score += 10;

                // Create particles
                for _ in 0..8 {
                    self.particles.push(Particle::new(
                        bx + bw / 2.0,
                        by + bh / 2.0,
                        brick.color,
                    ));
                }

                // Determine collision side and bounce accordingly
                let ball_center_x = self.ball.position.x;
                let ball_center_y = self.ball.position.y;
                let brick_center_x = bx + bw / 2.0;
                let brick_center_y = by + bh / 2.0;

                let dx = ball_center_x - brick_center_x;
                let dy = ball_center_y - brick_center_y;

                if dx.abs() / (bw / 2.0) > dy.abs() / (bh / 2.0) {
                    self.ball.velocity.x = -self.ball.velocity.x;
                } else {
                    self.ball.velocity.y = -self.ball.velocity.y;
                }

                break;
            }
        }
    }

    fn draw(&self) {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        // Draw background grid
        for x in (0..(SCREEN_WIDTH as i32)).step_by(40) {
            draw_line(x as f32, 0.0, x as f32, SCREEN_HEIGHT, 0.5,
                      Color::new(0.1, 0.1, 0.2, 0.3));
        }
        for y in (0..(SCREEN_HEIGHT as i32)).step_by(40) {
            draw_line(0.0, y as f32, SCREEN_WIDTH, y as f32, 0.5,
                      Color::new(0.1, 0.1, 0.2, 0.3));
        }

        // Draw game objects
        for brick in &self.bricks {
            brick.draw();
        }

        self.paddle.draw();
        self.ball.draw();

        // Draw particles
        for particle in &self.particles {
            particle.draw();
        }

        // Draw UI
        self.draw_ui();
    }

    fn draw_ui(&self) {
        let font_size = 30.0;

        // Score and lives
        draw_text(&format!("Score: {}", self.score), 20.0, 30.0, font_size, WHITE);
        draw_text(&format!("Lives: {}", self.lives), SCREEN_WIDTH - 150.0, 30.0, font_size, WHITE);

        // Game state messages
        match self.state {
            GameState::Paused => {
                let text = "PAUSED - Press P to resume";
                let text_width = measure_text(text, None, 40, 1.0).width;
                draw_text(text, (SCREEN_WIDTH - text_width) / 2.0, SCREEN_HEIGHT / 2.0, 40.0, YELLOW);
            },
            GameState::GameOver => {
                let text = "GAME OVER";
                let text_width = measure_text(text, None, 60, 1.0).width;
                draw_text(text, (SCREEN_WIDTH - text_width) / 2.0, SCREEN_HEIGHT / 2.0 - 40.0, 60.0, RED);

                let restart_text = "Press R to restart";
                let restart_width = measure_text(restart_text, None, 30, 1.0).width;
                draw_text(restart_text, (SCREEN_WIDTH - restart_width) / 2.0, SCREEN_HEIGHT / 2.0 + 20.0, 30.0, WHITE);
            },
            GameState::Victory => {
                let text = "VICTORY!";
                let text_width = measure_text(text, None, 60, 1.0).width;
                draw_text(text, (SCREEN_WIDTH - text_width) / 2.0, SCREEN_HEIGHT / 2.0 - 40.0, 60.0, GREEN);

                let restart_text = "Press R to restart";
                let restart_width = measure_text(restart_text, None, 30, 1.0).width;
                draw_text(restart_text, (SCREEN_WIDTH - restart_width) / 2.0, SCREEN_HEIGHT / 2.0 + 20.0, 30.0, WHITE);
            },
            _ => {}
        }

        // Controls
        if self.state == GameState::Playing {
            draw_text("A/D or Arrow Keys - Move  |  P - Pause", 20.0, SCREEN_HEIGHT - 20.0, 20.0,
                      Color::new(1.0, 1.0, 1.0, 0.6));
        }
    }

    fn restart(&mut self) {
        self.ball.reset(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT - 100.0);
        self.paddle.position = Vec2::new(SCREEN_WIDTH / 2.0 - PADDLE_WIDTH / 2.0, SCREEN_HEIGHT - 60.0);
        self.init_bricks();
        self.state = GameState::Playing;
        self.score = 0;
        self.lives = 3;
        self.particles.clear();
    }
}

#[macroquad::main("Brick Breaker")]
async fn main() {
    let mut game = Game::new();

    loop {
        let dt = get_frame_time();

        game.update(dt);
        game.draw();

        next_frame().await;
    }
}