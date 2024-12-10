use rand::Rng;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

// Asset loading
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct Sprites {
    images: HashMap<String, HtmlImageElement>,
}

impl Sprites {
    fn new() -> Self {
        Sprites {
            images: HashMap::new(),
        }
    }

    async fn load_image(_name: &str, path: &str) -> Result<HtmlImageElement, JsValue> {
        let image = HtmlImageElement::new()?;
        let promise = js_sys::Promise::new(&mut |resolve, reject| {
            image.set_src(path);
            image.set_onload(Some(&resolve));
            image.set_onerror(Some(&reject));
        });
        JsFuture::from(promise).await?;
        Ok(image)
    }

    async fn load_all() -> Result<Self, JsValue> {
        let mut sprites = Sprites::new();

        // Load all sprite images
        let images = [
            ("bird", "assets/bird.png"),
            ("pipe", "assets/pipe.png"),
            ("background", "assets/background.png"),
        ];

        for (name, path) in images.iter() {
            match Self::load_image(name, path).await {
                Ok(img) => {
                    sprites.images.insert(name.to_string(), img);
                }
                Err(e) => {
                    log(&format!("Failed to load {}: {:?}", name, e));
                    return Err(e);
                }
            }
        }

        Ok(sprites)
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

#[wasm_bindgen]
pub struct Game {
    context: CanvasRenderingContext2d,
    bird_y: f64,
    bird_velocity: f64,
    pipes: Vec<Pipe>,
    score: u32,
    state: GameState,
    game_speed: f64,
    gravity: f64,
    sprites: Option<Sprites>,
    bird_frame: u32,  // For bird animation
    frame_count: u32, // For general animation timing
}

struct Pipe {
    x: f64,
    gap_y: f64,
    width: f64,
    gap_height: f64,
    passed: bool,
}

impl Game {
    // Helper function to calculate difficulty multiplier based on score
    fn get_difficulty_multiplier(&self) -> f64 {
        // Gradually increase difficulty up to a maximum
        let base_multiplier = 1.0 + (self.score as f64 * 0.05);
        base_multiplier.min(2.0) // Cap at 2x difficulty
    }

    // Helper function to get current gap height based on score
    fn get_current_gap_height(&self) -> f64 {
        let base_gap = 200.0; // Starting gap height
        let min_gap = 150.0; // Minimum gap height
        let gap = base_gap / self.get_difficulty_multiplier();
        gap.max(min_gap)
    }

    // Helper function to get current game speed
    fn get_current_speed(&self) -> f64 {
        self.game_speed * self.get_difficulty_multiplier()
    }

    // Helper function to get current gravity
    fn get_current_gravity(&self) -> f64 {
        self.gravity * self.get_difficulty_multiplier()
    }
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<Game, JsValue> {
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Game {
            context,
            bird_y: 200.0,
            bird_velocity: 0.0,
            pipes: Vec::new(),
            score: 0,
            state: GameState::Menu,
            game_speed: 1.5,
            gravity: 0.4,
            sprites: None,
            bird_frame: 0,
            frame_count: 0,
        })
    }

    pub async fn load_assets(&mut self) -> Result<(), JsValue> {
        self.sprites = Some(Sprites::load_all().await?);
        Ok(())
    }

    pub fn update(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);

        if self.state != GameState::Playing {
            return;
        }

        // Animate bird (change frame every 10 frames)
        if self.frame_count % 10 == 0 {
            self.bird_frame = (self.bird_frame + 1) % 3; // Assuming 3 animation frames
        }

        // Update bird physics with dynamic gravity
        self.bird_velocity += self.get_current_gravity();
        self.bird_y += self.bird_velocity;

        // Update pipes with dynamic speed
        let current_speed = self.get_current_speed();
        for pipe in self.pipes.iter_mut() {
            pipe.x -= current_speed;

            // Check if bird has passed this pipe
            if !pipe.passed && (100.0 > pipe.x + pipe.width) {
                pipe.passed = true;
                self.score += 1;
            }
        }

        // Remove off-screen pipes
        self.pipes.retain(|pipe| pipe.x > -pipe.width);

        // Add new pipes with dynamic gap height
        if self.pipes.is_empty() || self.pipes.last().unwrap().x < 300.0 {
            let mut rng = rand::thread_rng();
            let gap_height = self.get_current_gap_height();

            // Adjust the vertical position range based on gap height
            let min_gap_y = gap_height * 0.5; // Ensure some space at the top
            let max_gap_y = 500.0 - gap_height * 1.5; // Ensure some space at the bottom

            self.pipes.push(Pipe {
                x: 500.0,
                gap_y: rng.gen_range(min_gap_y..max_gap_y),
                width: 50.0,
                gap_height,
                passed: false,
            });
        }

        // Check collisions
        self.check_collisions();
    }

    pub fn render(&self) {
        // Clear canvas
        self.context.clear_rect(0.0, 0.0, 500.0, 500.0);

        if let Some(sprites) = &self.sprites {
            // Draw background
            if let Some(bg) = sprites.images.get("background") {
                // Calculate scaling factors to fit the canvas height while maintaining aspect ratio
                let bg_height = bg.height() as f64;
                let scale = 500.0 / bg_height;  // Scale to fit canvas height
                let scaled_width = (bg.width() as f64) * scale;
                
                // Calculate how many times we need to repeat the background
                let repeats = (500.0 / scaled_width).ceil() as i32;
                
                for i in 0..repeats {
                    self.context
                        .draw_image_with_html_image_element_and_dw_and_dh(
                            bg,
                            i as f64 * scaled_width,
                            0.0,
                            scaled_width,
                            500.0,  // Canvas height
                        )
                        .unwrap();
                }
            }

            // Draw bird with animation
            if let Some(bird) = sprites.images.get("bird") {
                self.context
                    .draw_image_with_html_image_element_and_dw_and_dh(
                        bird,
                        100.0,
                        self.bird_y,
                        34.0, // Width of bird sprite
                        24.0, // Height of bird sprite
                    )
                    .unwrap();
            }

            // Draw pipes
            if self.state != GameState::Menu {
                if let Some(pipe) = sprites.images.get("pipe") {
                    for pipe_obj in &self.pipes {
                        // Draw top pipe (flipped)
                        self.context.save();
                        self.context.translate(pipe_obj.x, pipe_obj.gap_y).unwrap();
                        self.context.scale(1.0, -1.0).unwrap();
                        self.context
                            .draw_image_with_html_image_element_and_dw_and_dh(
                                pipe,
                                0.0,
                                0.0,
                                pipe_obj.width,
                                pipe_obj.gap_y,
                            )
                            .unwrap();
                        self.context.restore();

                        // Draw bottom pipe
                        self.context
                            .draw_image_with_html_image_element_and_dw_and_dh(
                                pipe,
                                pipe_obj.x,
                                pipe_obj.gap_y + pipe_obj.gap_height,
                                pipe_obj.width,
                                500.0 - (pipe_obj.gap_y + pipe_obj.gap_height),
                            )
                            .unwrap();
                    }
                }
            }
        }

        // Draw score and difficulty level if playing or paused
        if self.state == GameState::Playing || self.state == GameState::Paused {
            #[allow(deprecated)]
            self.context.set_fill_style(&JsValue::from("black"));
            self.context.set_font("24px Arial");
            self.context
                .fill_text(&format!("Score: {}", self.score), 10.0, 30.0)
                .unwrap();

            // Show current difficulty level
            let difficulty_percent = ((self.get_difficulty_multiplier() - 1.0) * 100.0) as u32;
            self.context.set_font("16px Arial");
            self.context
                .fill_text(&format!("Difficulty: {}%", difficulty_percent), 10.0, 60.0)
                .unwrap();
        }

        // Draw state-specific messages
        #[allow(deprecated)]
        self.context.set_fill_style(&JsValue::from("black"));
        self.context.set_font("30px Arial");
        match self.state {
            GameState::Menu => {
                self.context.fill_text("Flappy Bird", 180.0, 200.0).unwrap();
                self.context.set_font("20px Arial");
                self.context
                    .fill_text("Press SPACE to Start", 160.0, 250.0)
                    .unwrap();
                self.context
                    .fill_text("Now with Progressive Difficulty!", 120.0, 300.0)
                    .unwrap();
            }
            GameState::Paused => {
                self.context.fill_text("PAUSED", 200.0, 250.0).unwrap();
            }
            GameState::GameOver => {
                self.context.fill_text("Game Over!", 180.0, 200.0).unwrap();
                self.context.set_font("20px Arial");
                self.context
                    .fill_text(&format!("Final Score: {}", self.score), 180.0, 250.0)
                    .unwrap();
                self.context
                    .fill_text("Press SPACE to Restart", 160.0, 300.0)
                    .unwrap();
            }
            _ => {}
        }
    }

    pub fn flap(&mut self) {
        if self.state == GameState::Playing {
            // Adjust flap strength based on difficulty
            let flap_strength = -8.0 - (self.get_difficulty_multiplier() - 1.0);
            self.bird_velocity = flap_strength;
        }
    }

    pub fn toggle_pause(&mut self) {
        if self.state == GameState::Playing {
            self.state = GameState::Paused;
        } else if self.state == GameState::Paused {
            self.state = GameState::Playing;
        }
    }

    pub fn start_game(&mut self) {
        match self.state {
            GameState::Menu | GameState::GameOver => {
                self.reset_game();
                self.state = GameState::Playing;
            }
            GameState::Paused => {
                self.state = GameState::Playing;
            }
            _ => {}
        }
    }

    pub fn quit_game(&mut self) {
        self.reset_game();
        self.state = GameState::Menu;
    }

    fn reset_game(&mut self) {
        self.bird_y = 200.0;
        self.bird_velocity = 0.0;
        self.pipes.clear();
        self.score = 0;
        // Reset to initial game speed and gravity
        self.game_speed = 1.5;
        self.gravity = 0.4;
    }

    fn check_collisions(&mut self) {
        // Check if bird hits the ground or ceiling
        if self.bird_y <= 0.0 || self.bird_y >= 500.0 {
            self.state = GameState::GameOver;
            return;
        }

        // Check collision with pipes
        for pipe in &self.pipes {
            // Bird hitbox dimensions (more precise collision detection)
            let bird_x = 100.0;
            let bird_width = 25.0;  // Reduced from 30.0
            let bird_height = 15.0; // Reduced from 20.0
            let bird_hitbox_x_offset = 5.0;  // Offset from the left/right edges
            let bird_hitbox_y_offset = 4.0;  // Offset from the top/bottom edges

            // Check if bird is within pipe's x-range (with offset)
            if (bird_x + bird_hitbox_x_offset + bird_width) > pipe.x && 
               (bird_x + bird_hitbox_x_offset) < (pipe.x + pipe.width) {
                // Check if bird hits the pipes (with offset)
                if (self.bird_y - bird_height + bird_hitbox_y_offset) < pipe.gap_y || 
                   (self.bird_y + bird_height - bird_hitbox_y_offset) > (pipe.gap_y + pipe.gap_height) {
                    self.state = GameState::GameOver;
                    return;
                }
            }
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }
}
