# Flappy Bird WASM

A WebAssembly implementation of the classic Flappy Bird game, built with Rust and compiled to WebAssembly. This version features progressive difficulty that increases as you play, making it both accessible to beginners and challenging for experienced players.

## Play the Game

[Click here to play the game!](https://intertwine.github.io/flappy-bird-wasm/)

## 🎮 Gameplay

### Controls

- **SPACE / Click**: Make the bird flap
- **P**: Pause/Unpause the game
- **ESC**: Return to menu

### Game Mechanics

- The bird automatically falls due to gravity
- Press SPACE or click to make the bird flap upward
- Downward speed is capped so beginners can maintain control
- Navigate through pipes to score points
- Each point increases the difficulty by 5%
- Game ends if the bird hits a pipe or the ground

### Progressive Difficulty System

The game features a dynamic difficulty system that scales as you play:

- **Starting Difficulty**: Easy and approachable for beginners
  - Slower game speed
  - Lower gravity
  - Wider gaps between pipes
- **Progression**: Each point scored increases difficulty by 5%
  - Maximum difficulty caps at 2x the initial difficulty
  - Affects game speed, gravity, and pipe gap size
  - Flap strength automatically adjusts to maintain playability

## 🛠 Technical Implementation

### Architecture

The game is built using a hybrid of Rust (compiled to WebAssembly) and JavaScript:

#### Rust (Core Game Logic)

- `lib.rs`: Main game logic
  - Game state management
  - Physics calculations
  - Collision detection
  - Progressive difficulty scaling
  - Rendering logic

#### Web Technologies

- **WebAssembly**: Compiled Rust code
- **HTML5 Canvas**: Game rendering
- **JavaScript**: DOM manipulation and game loop

### Key Components

#### Game State Management

```rust
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}
```

- Handles different game states and transitions
- Controls game flow and user input processing

#### Physics Engine

- Gravity simulation with dynamic scaling
- Velocity-based bird movement
- Collision detection with pipes and boundaries

#### Asset Management

- Sprite-based rendering system
- Handles loading and drawing of:
  - Bird sprite
  - Pipe sprites
  - Background image

#### Progressive Difficulty System Parameters

- Dynamic scaling of game parameters:

  ```rust
  game_speed: Initial 1.5 → Max 3.0
  gravity: Initial 0.4 → Max 0.8
  pipe_gap: Initial 200px → Min 150px
  ```

## 🚀 Development Setup

### Prerequisites

- Rust (latest stable)
- wasm-pack
- Python 3 (for local development server)
- Web browser with WebAssembly support

### Building

1. Clone the repository
2. Install dependencies:

   ```bash
   cargo build
   ```

3. Build WebAssembly:

   ```bash
   wasm-pack build --target web
   ```

4. Start local server:

   ```bash
   python3 -m http.server 8000
   ```

5. Open `http://localhost:8000` in your browser

### Project Structure

```text
flappy-bird-wasm/
├── src/
│   └── lib.rs          # Main game logic
├── assets/
│   ├── bird.png        # Bird sprite
│   ├── pipe.png        # Pipe sprite
│   └── background.png  # Background image
├── Cargo.toml          # Rust dependencies
├── index.html          # Web entry point
└── pkg/                # Compiled WebAssembly
```

## 🔧 Dependencies

### Rust Crates

- `wasm-bindgen`: WebAssembly bindings
- `web-sys`: Web API interfaces
- `js-sys`: JavaScript API interfaces
- `rand`: Random number generation

### Web Dependencies

- No external JavaScript libraries required
- Pure WebAssembly and vanilla JavaScript implementation

## 🎯 Future Improvements

Planned features and enhancements:

1. Sound effects and background music
2. High score system with local storage
3. Additional visual effects and animations
4. Mobile-optimized controls
5. Multiple difficulty modes
6. Achievement system

## 📝 License

This project is open source and available under the MIT License.

## 🙋‍♂️ Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
