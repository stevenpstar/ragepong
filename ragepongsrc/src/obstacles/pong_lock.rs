use godot::{classes::{Area2D, IArea2D, Node2D}, obj::{Base, Gd, WithBaseField, WithUserSignals}, prelude::{godot_api, GodotClass}};

use crate::player::pong::Pong;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct PongLock {
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for PongLock {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        self.signals()
            .body_entered()
            .connect_self(Self::on_body_entered);
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
        }
    }
}
