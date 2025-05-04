use godot::{builtin::Array, classes::{Area2D, IArea2D, Node2D}, obj::{Base, Gd, WithBaseField, WithUserSignals}, prelude::{godot_api, GodotClass}};

use crate::player::pong::Pong;

use super::laser_gate::LaserGate;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct PongLock {
    #[export]
    activatables: Array<Gd<Node2D>>,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for PongLock {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            activatables: Default::default(),
            base,
        }
    }

    fn ready(&mut self) {
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
            let mut pong = body.cast::<Pong>();
            pong.bind_mut().lock(this);
            self.toggle();
        }
    }

    #[func]
    fn on_body_exited(&mut self, body: Gd<Node2D>) {
        if body.get_class() == "Pong".into() {
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
