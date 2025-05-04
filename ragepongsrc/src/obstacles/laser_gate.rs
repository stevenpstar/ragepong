use godot::{classes::{Area2D, IArea2D}, global::godot_print, obj::{Base, WithBaseField}, prelude::{godot_api, GodotClass}};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct LaserGate {
    #[export]
    open: bool,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for LaserGate {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            open: false,
            base,
        }
    }
}

#[godot_api]
impl LaserGate {
    #[func]
    pub fn toggle_gate(&mut self) {
        godot_print!("Toggling gate!");
        self.open = !self.open;
        let open = self.open;
//        self.base_mut().set_monitoring(!open);
//        self.base_mut().set_monitorable(!open);
        self.base_mut().set_visible(!open);
    }
}
