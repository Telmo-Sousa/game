use ggez::error::GameError;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Text};
use ggez::{event::{self, EventHandler, KeyCode}, Context, GameResult};
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PLAYER_SIZE: f32 = 10.0;
const ENEMY_SIZE: f32 = 10.0;
const BULLET_SIZE: f32 = 5.0;
const BULLET_SPEED: f32 = 0.5;
const PLAYER_SPEED: f32 = 5.0;
const ENEMY_SPEED: f32 = 2.5;

struct MainState {
    player_x: f32,
    player_y: f32,
    enemies: Vec<(f32, f32, Instant)>,
    bullets: Vec<(f32, f32)>,
    score: i32,
    level: i32,
    player_lost: bool,
}

impl MainState {
    fn new() -> Self {
        MainState {
            player_x: WINDOW_WIDTH / 2.0,
            player_y: WINDOW_HEIGHT / 2.0,
            enemies: vec![],
            bullets: vec![],
            score: 0,
            level: 1,
            player_lost: false,
        }
    }

    fn start_level(&mut self) {
        self.enemies.clear();
        for _ in 0..(10 * self.level) {
            self.spawn_enemy();
        }
    }

    fn spawn_enemy(&mut self) {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..WINDOW_WIDTH - ENEMY_SIZE);
        let y = rng.gen_range(0.0..WINDOW_HEIGHT - ENEMY_SIZE);
        self.enemies.push((x, y, Instant::now()));
    }

    fn move_enemies(&mut self) {
        for enemy in &mut self.enemies {
            if enemy.2.elapsed().as_secs_f32() >= 0.1 {
                if enemy.0 < self.player_x {
                    enemy.0 += ENEMY_SPEED;
                } else if enemy.0 > self.player_x {
                    enemy.0 -= ENEMY_SPEED;
                }
                if enemy.1 < self.player_y {
                    enemy.1 += ENEMY_SPEED;
                } else if enemy.1 > self.player_y {
                    enemy.1 -= ENEMY_SPEED;
                }
                enemy.2 = Instant::now();
            }
        }
    }

    fn shoot(&mut self) {
        self.bullets.push((
            self.player_x + PLAYER_SIZE / 2.0 - BULLET_SIZE / 2.0,
            self.player_y + PLAYER_SIZE / 2.0 - BULLET_SIZE / 2.0,
        ));
    }

    fn update_bullets(&mut self) {
        self.bullets.retain(|(x, _)| *x <= WINDOW_WIDTH);
        for (x, y) in &mut self.bullets {
            *x += BULLET_SPEED;
        }
    }

    fn detect_collisions(&mut self) {
        let mut bullets_to_remove = Vec::new();
        let mut enemies_to_remove = Vec::new();
    
        for (bullet_idx, bullet) in self.bullets.iter().enumerate() {
            let bullet_rect = graphics::Rect::new(bullet.0, bullet.1, BULLET_SIZE, BULLET_SIZE);
            for (enemy_idx, enemy) in self.enemies.iter().enumerate() {
                let enemy_rect = graphics::Rect::new(enemy.0, enemy.1, ENEMY_SIZE, ENEMY_SIZE);
                if bullet_rect.overlaps(&enemy_rect) {
                    bullets_to_remove.push(bullet_idx);
                    enemies_to_remove.push(enemy_idx);
                }
            }
        }
    
        // Remove enemies first
        for idx in enemies_to_remove.into_iter().rev() {
            if idx < self.enemies.len() {
                self.enemies.remove(idx);
                self.score += 1;
            }
        }
    
        // Remove bullets
        for idx in bullets_to_remove.into_iter().rev() {
            if idx < self.bullets.len() {
                self.bullets.remove(idx);
            }
        }
    }
    

    fn detect_player_enemy_collision(&self) -> bool {
        for enemy in &self.enemies {
            let enemy_rect = graphics::Rect::new(enemy.0, enemy.1, ENEMY_SIZE, ENEMY_SIZE);
            let player_rect = graphics::Rect::new(self.player_x, self.player_y, PLAYER_SIZE, PLAYER_SIZE);
            if enemy_rect.overlaps(&player_rect) {
                return true;
            }
        }
        false
    }

    fn handle_loss(&mut self, ctx: &mut Context) {
        self.player_lost = true;
    }

}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.move_enemies();
        self.update_bullets();
        self.detect_collisions();

        if self.detect_player_enemy_collision() {
            self.handle_loss(ctx);
        }

        if self.enemies.is_empty() {
            self.level += 1;
            self.start_level();
        }


        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
    
        let player_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(self.player_x, self.player_y, PLAYER_SIZE, PLAYER_SIZE),
            Color::RED,
        )?;
        graphics::draw(ctx, &player_mesh, DrawParam::default())?;
    
        for enemy in &self.enemies {
            let enemy_mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                graphics::Rect::new(enemy.0, enemy.1, ENEMY_SIZE, ENEMY_SIZE),
                Color::BLUE,
            )?;
            graphics::draw(ctx, &enemy_mesh, DrawParam::default())?;
        }
    
        for (x, y) in &self.bullets {
            let bullet_mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                graphics::Rect::new(*x, *y, BULLET_SIZE, BULLET_SIZE),
                Color::WHITE,
            )?;
            graphics::draw(ctx, &bullet_mesh, DrawParam::default())?;
        }
    
        let score_text = Text::new(format!("Score: {}", self.score));
        let score_position = [
            WINDOW_WIDTH / 2.0 - score_text.width(ctx) as f32 / 2.0, 
            10.0
        ];
        graphics::draw(ctx, &score_text, (score_position, 0.0, Color::WHITE))?;
        
        let level_text = Text::new(format!("Level: {}", self.level));
        let level_position = [
            WINDOW_WIDTH / 2.0 - level_text.width(ctx) as f32 / 2.0, 
            26.0
        ];
        graphics::draw(ctx, &level_text, (level_position, 0.0, Color::WHITE))?;
        
        if self.player_lost {
            let lost_text = Text::new("You lost!");
            let lost_position = [
                WINDOW_WIDTH / 2.0 - lost_text.width(ctx) as f32 / 2.0, 
                WINDOW_HEIGHT / 2.0,
            ];
            graphics::draw(ctx, &lost_text, (lost_position, 0.0, Color::WHITE))?;

        }

        if self.enemies.is_empty() {
            let won_text = Text::new("You won!");
            let won_position = [
                WINDOW_WIDTH / 2.0 - won_text.width(ctx) as f32 / 2.0, 
                WINDOW_HEIGHT / 2.0,
            ];
            graphics::draw(ctx, &won_text, (won_position, 0.0, Color::WHITE))?;
        }
    
        graphics::present(ctx)?;
        Ok(())
    }
    
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        if self.player_lost {
            return;
        }

        match keycode {
            KeyCode::W => {
                if self.player_y > 0.0 {
                    self.player_y -= PLAYER_SPEED;
                }
            }
            KeyCode::A => {
                if self.player_x > 0.0 {
                    self.player_x -= PLAYER_SPEED;
                }
            }
            KeyCode::S => {
                if self.player_y < WINDOW_HEIGHT - PLAYER_SIZE {
                    self.player_y += PLAYER_SPEED;
                }
            }
            KeyCode::D => {
                if self.player_x < WINDOW_WIDTH - PLAYER_SIZE {
                    self.player_x += PLAYER_SPEED;
                }
            }
            KeyCode::Space => {
                self.shoot();
            }
            _ => {}
        }
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("my_game", "telmo-sousa")
        .window_setup(ggez::conf::WindowSetup::default().title("My Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    let mut state = MainState::new();
    for _ in 0..10 {
        state.spawn_enemy();
    }
    event::run(ctx, event_loop, state)
}
