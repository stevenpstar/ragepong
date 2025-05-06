use godot::{builtin::Vector2, classes::{Area2D, IArea2D, Node2D}, obj::{Base, Gd, OnReady, WithUserSignals}, prelude::{godot_api, GodotClass}};

use super::game_camera::GameCamera;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct CameraArea {
    camera: OnReady<Gd<GameCamera>>,
    #[export]
    camera_pos: Option<Gd<Node2D>>,
    #[export]
    camera_zoom: f32,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for CameraArea {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            camera: OnReady::from_node("/root/Game/Camera"),
            camera_pos: None,
            camera_zoom: 1.0,
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
impl CameraArea {
    #[signal]
    fn test_signal();

    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        if body.get_class() == "Player".into() {
            let cam_pos = match &self.camera_pos {
                None => Vector2::new(0.0, 0.0),
                Some(pos) => pos.get_global_position()
            };
            let zoom = self.camera_zoom;
            self.camera.bind_mut().set_target_pos(cam_pos);
            self.camera.bind_mut().set_target_zoom(zoom);
        }
    }
}
