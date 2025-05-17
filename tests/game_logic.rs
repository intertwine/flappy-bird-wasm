use wasm_bindgen_test::*;
use flappy_bird_wasm::Game;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn difficulty_multiplier_never_exceeds_two() {
    let mut game = Game::new_headless().unwrap();

    for score in 0..100 {
        game.set_score(score);
        assert!(game.get_difficulty_multiplier() <= 2.0);
    }
}
