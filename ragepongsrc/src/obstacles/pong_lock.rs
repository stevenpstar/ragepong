use godot::{builtin::{Array, StringName}, classes::{Area2D, Engine, IArea2D, Node2D}, global::godot_print, obj::{Base, Gd, WithBaseField, WithUserSignals}, prelude::{godot_api, GodotClass}};

use crate::{engine::game::Game, player::pong::Pong};

use super::laser_gate::LaserGate;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct PongLock {
    #[export]
    activatables: Array<Gd<Node2D>>,
    game: Option<Gd<Game>>,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for PongLock {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            activatables: Default::default(),
            game: None,
            base,
        }
    }

    fn ready(&mut self) {
        let game = match Engine::singleton().get_singleton(&StringName::from("Game")) {
            None => panic!("No game singleton"),
            Some(game) => game.cast::<Game>()
        };

        self.game = Some(game);

        self.signals()
            .body_entered()
            .connect_self(Self::on_body_entered);

        self.signals()
            .body_exited()
            .connect_self(Self::on_body_exited);
    }
}

#[godot_api]
impl PongLock {
    #[signal]
    pub fn lock_ball();

    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        let this = self.to_gd();
        if body.get_class() == "Pong".into() {
            let g = match &self.game {
                None => panic!("Should never panic here"),
                Some(g) => g
            };
            let mut pong = body.cast::<Pong>();
            if g.bind().is_resetting() {
                return;
            }
            pong.bind_mut().lock(this);
            self.toggle();
        }
    }

    #[func]
    fn on_body_exited(&mut self, body: Gd<Node2D>) {
        if body.get_class() == "Pong".into() {

            let g = match &self.game {
                None => panic!("Should never panic here"),
                Some(g) => g
            };

            if g.bind().is_resetting() {
                return;
            }
            self.toggle();
        }
    }

    #[func]
    fn toggle(&mut self) {
        for act in self.activatables.iter_shared() {
            if act.get_class() == "LaserGate".into() {
                let mut laser_gate = act.cast::<LaserGate>();
                laser_gate.bind_mut().toggle_gate();
            }
        }
    }

}
