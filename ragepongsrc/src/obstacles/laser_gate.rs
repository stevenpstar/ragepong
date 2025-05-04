use godot::{classes::{Area2D, IArea2D}, obj::{Base, WithBaseField}, prelude::{godot_api, GodotClass}};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct LaserGate {
    #[export]
    open: bool,
    is_open: bool,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for LaserGate {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            open: false,
            is_open: false,
            base,
        }
    }

    fn ready(&mut self) {
        self.is_open = self.open;
    }
}

#[godot_api]
impl LaserGate {
    #[func]
    pub fn toggle_gate(&mut self) {
        self.is_open = !self.is_open;
        let open = self.is_open;
        self.base_mut().set_visible(!open);
    }

    #[func]
    pub fn reset(&mut self) {
        self.is_open = self.open;
        let open = self.is_open;
        self.base_mut().set_visible(!open);
    }

    #[func]
    pub fn get_is_open(&self) -> bool {
        return self.is_open;
    }
}
