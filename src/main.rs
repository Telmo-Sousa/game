use ggez::error::GameError;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Text};
use ggez::{
    event::{self, EventHandler, KeyCode},
    Context, GameResult,
};
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PLAYER_SIZE: f32 = 10.0;
const ENEMY_SIZE: f32 = 10.0;
const BULLET_SIZE: f32 = 5.0;
const COIN_SIZE: f32 = 10.0;
const BULLET_SPEED: f32 = 0.5;
const PLAYER_SPEED: f32 = 5.0;
const ENEMY_SPEED: f32 = 2.5;
const COIN_SPAWN_INTERVAL: f32 = 5.0;
const SHOP_ITEM_COST: i32 = 100;

struct MainState {
    player_x: f32,
    player_y: f32,
    enemies: Vec<(f32, f32, Instant)>,
    bullets: Vec<(f32, f32, f32, f32)>,
    coins: Vec<Coin>,
    score: i32,
    level: i32,
    player_lost: bool,
    menu_active: bool,
    bullets_limit: i32,
    bullets_on_screen: i32,
    last_coin_spawn_time: Instant,
}

impl MainState {
    fn new() -> Self {
        MainState {
            player_x: WINDOW_WIDTH / 2.0,
            player_y: WINDOW_HEIGHT / 2.0,
            enemies: vec![],
            bullets: vec![],
            coins: vec![],
            score: 0,
            level: 0,
            player_lost: false,
            menu_active: true,
            bullets_limit: 0,
            bullets_on_screen: 0,
            last_coin_spawn_time: Instant::now(),
        }
    }

    fn start_level(&mut self) {
        self.enemies.clear();
        let enemies_count = 10 * self.level;
        for _ in 0..enemies_count {
            self.spawn_enemy();
        }
        self.bullets_limit = enemies_count + 10;
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

    fn shoot(&mut self, dx: f32, dy: f32) {
        if self.bullets_limit > 0 && self.bullets_on_screen < 5 {
            let bullet_x = self.player_x + PLAYER_SIZE / 2.0 - BULLET_SIZE / 2.0;
            let bullet_y = self.player_y + PLAYER_SIZE / 2.0 - BULLET_SIZE / 2.0;
            self.bullets.push((bullet_x, bullet_y, dx, dy));
            self.bullets_limit -= 1;
            self.bullets_on_screen += 1;
        }
    }

    fn shoot_left(&mut self) {
        self.shoot(-BULLET_SPEED, 0.0);
    }

    fn shoot_down(&mut self) {
        self.shoot(0.0, BULLET_SPEED);
    }

    fn shoot_up(&mut self) {
        self.shoot(0.0, -BULLET_SPEED);
    }

    fn shoot_right(&mut self) {
        self.shoot(BULLET_SPEED, 0.0);
    }

    fn update_bullets(&mut self) {
        let mut bullets_to_remove = Vec::new();
        for (i, (x, y, dx, dy)) in self.bullets.iter_mut().enumerate() {
            if *x > WINDOW_WIDTH || *x < 0.0 || *y > WINDOW_HEIGHT || *y < 0.0 {
                bullets_to_remove.push(i);
            }
            *x += *dx;
            *y += *dy;
        }
        for idx in bullets_to_remove.into_iter().rev() {
            if idx < self.bullets.len() {
                self.bullets.remove(idx);
                self.bullets_on_screen -= 1;
            }
        }
    }

    fn detect_collisions(&mut self) {
        let mut bullets_to_remove = Vec::new();
        let mut enemies_to_remove = Vec::new();

        for (bullet_idx, (bullet_x, bullet_y, _, _)) in self.bullets.iter().enumerate() {
            let bullet_rect = graphics::Rect::new(*bullet_x, *bullet_y, BULLET_SIZE, BULLET_SIZE);
            for (enemy_idx, (enemy_x, enemy_y, _)) in self.enemies.iter().enumerate() {
                let enemy_rect =
                    graphics::Rect::new(*enemy_x, *enemy_y, ENEMY_SIZE, ENEMY_SIZE);
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
                self.bullets_on_screen -= 1;
            }
        }
    }

    fn detect_player_enemy_collision(&self) -> bool {
        for (enemy_x, enemy_y, _) in &self.enemies {
            let enemy_rect = graphics::Rect::new(*enemy_x, *enemy_y, ENEMY_SIZE, ENEMY_SIZE);
            let player_rect =
                graphics::Rect::new(self.player_x, self.player_y, PLAYER_SIZE, PLAYER_SIZE);
            if enemy_rect.overlaps(&player_rect) {
                return true;
            }
        }
        false
    }

    fn handle_loss(&mut self, ctx: &mut Context) {
        self.player_lost = true;
    }

    fn handle_menu(&mut self, ctx: &mut Context) {
        self.menu_active = !self.menu_active;
    }

    fn spawn_coin(&mut self) {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..WINDOW_WIDTH - COIN_SIZE);
        let y = rng.gen_range(0.0..WINDOW_HEIGHT - COIN_SIZE);
        self.coins.push(Coin::new(x, y));
        self.last_coin_spawn_time = Instant::now();
    }

    fn update_coins(&mut self) {
        let elapsed = self.last_coin_spawn_time.elapsed().as_secs_f32();
        if elapsed >= COIN_SPAWN_INTERVAL {
            self.spawn_coin();
        }

        let mut coins_to_remove = Vec::new();
        for (i, coin) in self.coins.iter().enumerate() {
            if coin.should_despawn() {
                coins_to_remove.push(i);
            }
        }
        for idx in coins_to_remove.into_iter().rev() {
            self.coins.remove(idx);
        }
    }

    fn handle_coin_collisions(&mut self) {
        let mut coins_to_remove = Vec::new();
        for (i, coin) in self.coins.iter().enumerate() {
            let coin_rect = graphics::Rect::new(coin.x, coin.y, COIN_SIZE, COIN_SIZE);
            let player_rect =
                graphics::Rect::new(self.player_x, self.player_y, PLAYER_SIZE, PLAYER_SIZE);
            if coin_rect.overlaps(&player_rect) {
                self.score += 500;
                coins_to_remove.push(i);
            }
        }
        for idx in coins_to_remove.into_iter().rev() {
            self.coins.remove(idx);
        }
    }

    fn buy_shop_item(&mut self, item: ShopItem) {
        match item {
            ShopItem::Bullets => {
                if self.score >= SHOP_ITEM_COST {
                    self.score -= SHOP_ITEM_COST;
                    self.bullets_limit += 20;
                }
            }
            ShopItem::RemoveEnemies => {
                if self.score >= SHOP_ITEM_COST {
                    self.score -= SHOP_ITEM_COST;
                    let enemies_to_remove = self.enemies.len().min(5);
                    for _ in 0..enemies_to_remove {
                        self.enemies.pop();
                    }
                }
            }
            ShopItem::ScoreBoost => {
                if self.score >= SHOP_ITEM_COST {
                    self.score -= SHOP_ITEM_COST;
                    let random_chance: f32 = thread_rng().gen_range(0.0..1.0);
                    if random_chance < 0.5 {
                        self.score *= 2;
                    } else {
                        self.score = 0;
                    }
                }
            }
        }
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.menu_active {
            return Ok(());
        }

        self.move_enemies();
        self.update_bullets();
        self.detect_collisions();
        self.update_coins();
        self.handle_coin_collisions();

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
        if self.menu_active {
            graphics::clear(ctx, Color::BLACK);

            let welcome_text = Text::new("game!");
            let welcome_position = [
                WINDOW_WIDTH / 2.0 - welcome_text.width(ctx) as f32 / 2.0,
                WINDOW_HEIGHT / 2.0 - 20.0,
            ];
            graphics::draw(ctx, &welcome_text, (welcome_position, 0.0, Color::WHITE))?;

            let start_text = Text::new("press space");
            let start_position = [
                WINDOW_WIDTH / 2.0 - start_text.width(ctx) as f32 / 2.0,
                WINDOW_HEIGHT / 2.0 + 20.0,
            ];
            graphics::draw(ctx, &start_text, (start_position, 0.0, Color::WHITE))?;

            let shop_text_key1 = Text::new("Key 1 - 100 points for 20 more bullets");
            let shop_text_key2 = Text::new("Key 2 - 100 points to remove 5 enemies");
            let shop_text_key3 = Text::new("Key 3 - 100 points for a random score");
            let shop_position_x = 10.0;
            let shop_position_y = 40.0;

            let key1_position = [shop_position_x, shop_position_y];
            let key2_position = [shop_position_x, shop_position_y + 20.0];
            let key3_position = [shop_position_x, shop_position_y + 40.0];

            graphics::draw(ctx, &shop_text_key1, (key1_position, 0.0, Color::WHITE))?;
            graphics::draw(ctx, &shop_text_key2, (key2_position, 0.0, Color::WHITE))?;
            graphics::draw(ctx, &shop_text_key3, (key3_position, 0.0, Color::WHITE))?;

            graphics::present(ctx)?;
            return Ok(());
        }

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

        for (x, y, _, _) in &self.bullets {
            let bullet_mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                graphics::Rect::new(*x, *y, BULLET_SIZE, BULLET_SIZE),
                Color::WHITE,
            )?;
            graphics::draw(ctx, &bullet_mesh, DrawParam::default())?;
        }

        for coin in &self.coins {
            let coin_mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                graphics::Rect::new(coin.x, coin.y, COIN_SIZE, COIN_SIZE),
                Color::YELLOW,
            )?;
            graphics::draw(ctx, &coin_mesh, DrawParam::default())?;
        }

        let score_text = Text::new(format!("Score: {}", self.score));
        let score_position = [
            WINDOW_WIDTH / 2.0 - score_text.width(ctx) as f32 / 2.0,
            10.0,
        ];
        graphics::draw(ctx, &score_text, (score_position, 0.0, Color::WHITE))?;

        let level_text = Text::new(format!("Level: {}", self.level));
        let level_position = [
            WINDOW_WIDTH / 2.0 - level_text.width(ctx) as f32 / 2.0,
            26.0,
        ];
        graphics::draw(ctx, &level_text, (level_position, 0.0, Color::WHITE))?;

        let bullets_text = Text::new(format!("Bullets: {}", self.bullets_limit));
        let bullets_position = [
            WINDOW_WIDTH - bullets_text.width(ctx) as f32 - 10.0,
            WINDOW_HEIGHT - bullets_text.height(ctx) as f32 - 10.0,
        ];
        graphics::draw(ctx, &bullets_text, (bullets_position, 0.0, Color::WHITE))?;

        if self.player_lost {
            let lost_text = Text::new("you lost, press space");
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
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        if self.menu_active {
            if keycode == KeyCode::Space {
                self.handle_menu(ctx);
            }
            return;
        }

        if self.player_lost || self.enemies.is_empty() {
            if keycode == KeyCode::Space {
                self.player_x = WINDOW_WIDTH / 2.0;
                self.player_y = WINDOW_HEIGHT / 2.0;
                self.score = 0;
                self.level = 1;
                self.player_lost = false;
                self.start_level();
            }
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
            KeyCode::H => {
                self.shoot_left();
            }
            KeyCode::J => {
                self.shoot_down();
            }
            KeyCode::K => {
                self.shoot_up();
            }
            KeyCode::L => {
                self.shoot_right();
            }
            KeyCode::Escape => {
                self.handle_menu(ctx);
            }
            KeyCode::Key1 => {
                self.buy_shop_item(ShopItem::Bullets);
            }
            KeyCode::Key2 => {
                self.buy_shop_item(ShopItem::RemoveEnemies);
            }
            KeyCode::Key3 => {
                self.buy_shop_item(ShopItem::ScoreBoost);
            }
            _ => {}
        }
    }

}

struct Coin {
    x: f32,
    y: f32,
    spawn_time: Instant,
}

impl Coin {
    fn new(x: f32, y: f32) -> Self {
        Coin {
            x,
            y,
            spawn_time: Instant::now(),
        }
    }

    fn should_despawn(&self) -> bool {
        self.spawn_time.elapsed().as_secs_f32() >= COIN_SPAWN_INTERVAL
    }
}

enum ShopItem {
    Bullets,
    RemoveEnemies,
    ScoreBoost,
}

fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("my_game", "telmo-sousa")
        .window_setup(ggez::conf::WindowSetup::default().title("My Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    let state = MainState::new();
    event::run(ctx, event_loop, state)
}
