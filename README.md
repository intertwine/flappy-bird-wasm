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
