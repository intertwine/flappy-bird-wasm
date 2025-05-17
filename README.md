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
- Node.js and npm
- Web browser with WebAssembly support

### Building

1. Clone the repository
2. Install dependencies:

   ```bash
   # Install Rust dependencies
   cargo build

   # Install npm dependencies and download game assets
   npm install
   ```

   The `npm install` command will automatically download required game assets (sprites and images) to the `assets/` directory.

3. Build WebAssembly:

   ```bash
   # Using npm
   npm run build

   # Or using wasm-pack directly
   wasm-pack build --target web
   ```

4. Start local server:

   ```bash
   # Using npm
   npm start
   ```

   This will start a local development server using the `serve` package. The server will automatically open your default browser to the game.

5. If the browser doesn't open automatically, navigate to the URL shown in the terminal (typically `http://localhost:3000`)

### Git Configuration

The project uses `.gitignore` to exclude build artifacts and dependencies. If you've cloned the repository and find that some ignored files are still being tracked, you can remove them from Git's index (while keeping them locally) with:

```bash
git rm --cached <file>
```

Common files that might need this treatment:

- `Cargo.lock` (for library projects)
- `target/` directory
- `pkg/` directory
- `node_modules/` directory

### Project Structure

```text
flappy-bird-wasm/
├── src/
│   └── lib.rs          # Main game logic
├── assets/
│   ├── bird.png        # Bird sprite
│   ├── pipe.png        # Pipe sprite
│   └── background.png  # Background image
├── scripts/
│   └── download_assets.sh  # Asset download script
├── Cargo.toml          # Rust dependencies
├── package.json        # npm dependencies and scripts
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

## 🧪 Running Tests

Integration tests are executed with `wasm-bindgen-test`. Ensure the `wasm32-unknown-unknown` target is installed and run:

```bash
# Using npm
npm test              # Run tests in Chrome
npm run test:firefox  # Run tests in Firefox
npm run test:chrome   # Run tests in Chrome
npm run test:node     # Run tests in Node.js

# Or using cargo directly
cargo test --target wasm32-unknown-unknown -- --nocapture
```

The included `tests/game_logic.rs` verifies that the game's difficulty multiplier never exceeds `2.0` as the score increases.

### Testing Approach

- **Integration Tests**: Located in `tests/game_logic.rs`, these tests verify core game logic (e.g., difficulty scaling).
- **Browser-Based Tests**: Tests are configured to run in a browser environment using `wasm_bindgen_test_configure!(run_in_browser);`.
- **Running Tests**:
  - **Firefox**: Use `npm run test:firefox` or `wasm-pack test --headless --firefox`.
  - **Chrome**: Use `npm run test:chrome` or `wasm-pack test --headless --chrome`.
  - **Node.js**: Use `npm run test:node` or `wasm-pack test --node`.

### Test Configuration

- The crate is configured with both `cdylib` and `rlib` crate types to support integration tests.
- Test-specific methods (e.g., `new_headless` and `set_score`) are made available for testing but hidden from public documentation.

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
