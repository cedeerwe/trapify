use comfy::*;
use trapify::*;

simple_game!("Trapify", GameState, config, setup, update);

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        // This overrides the default ResolutionConfig::Logical(1106, 526) for WASM targets
        resolution: ResolutionConfig::Physical(1920, 1080),
        ..config
    }
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    #[cfg(target_arch = "wasm32")]
    let start_time = web_sys::window().unwrap().performance().unwrap().now();

    state.passage_of_time();

    state.draw();
    state.check_inputs();

    state.deal_damage_over_time_to_enemies();
    state.activate_traps();
    state.cleanup_dead_enemies();

    state.spawn_enemies();
    state.move_enemies_and_deal_damage_to_player();

    state.check_dead_player();
    state.check_game_over();

    state.apply_gold_interest();

    state.ui();

    // Hack to have lower FPS in wasm -- wait until 16 ms have passed since the beginning of update
    #[cfg(target_arch = "wasm32")]
    loop {
        let current_time = web_sys::window().unwrap().performance().unwrap().now();
        if current_time - start_time >= 16. {
            break;
        }
    }
}
