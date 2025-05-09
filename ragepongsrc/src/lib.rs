use engine::game::Game;
use godot::{classes::Engine, prelude::*};
mod core;
mod player;
mod effects;
mod obstacles;
mod engine;

struct RagePongExtension;

#[gdextension]
unsafe impl ExtensionLibrary for  RagePongExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            Engine::singleton().register_singleton(
                "Game",
                &Game::new_alloc()
                );
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let singleton_name = "Game";

            if let Some(game_singleton) = engine.get_singleton(singleton_name) {
                engine.unregister_singleton(singleton_name);
                game_singleton.free();
            } else {
                godot_error!("Failed to get singleton!");
            };
        }
    } 
}
