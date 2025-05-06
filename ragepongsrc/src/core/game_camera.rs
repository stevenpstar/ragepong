use godot::{builtin::Vector2, classes::{Camera2D, ICamera2D}, obj::{Base, WithBaseField}, prelude::{godot_api, GodotClass}};

#[derive(GodotClass)]
#[class(base=Camera2D)]
pub struct GameCamera {
    target_pos: Vector2,
    base: Base<Camera2D>,
}

#[godot_api]
impl ICamera2D for GameCamera {
    fn init(base: Base<Camera2D>) -> Self {
        Self {
            target_pos: Vector2::new(0.0, 0.0),
            base,
        }
    }

    fn process(&mut self, _delta: f64) {

        let mut pos = self.base_mut().get_global_position();

        if pos.x != self.target_pos.x ||
            pos.y != self.target_pos.y {
                pos = pos.lerp(self.target_pos, 0.3);
                self.base_mut().set_global_position(pos);
        }
    }
}

#[godot_api]
impl GameCamera {
    #[func]
    pub fn set_target_pos(&mut self, target: Vector2) {
        self.target_pos = target;
    }

    #[func]
    pub fn set_target_zoom(&mut self, zoom: f32) {
        self.base_mut().set_zoom(Vector2::new(zoom, zoom));
    }
}

