use godot::{builtin::Array, classes::{Area2D, IArea2D, Node2D}, global::godot_print, obj::{Base, Gd}, prelude::{godot_api, GodotClass}};

use crate::obstacles::laser_gate::LaserGate;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Switch {
    #[export]
    activatables: Array<Gd<Node2D>>,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Switch {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            activatables: Default::default(),
            base,
        }
    }
}

#[godot_api]
impl Switch {
    #[func]
    pub fn toggle(&mut self) {
        for act in self.activatables.iter_shared() {
            if act.get_class() == "LaserGate".into() {
                godot_print!("Switch toggle activated for LaserGate");
                let mut laser_gate = act.cast::<LaserGate>();
                laser_gate.bind_mut().toggle_gate();
            }
        }
    }
}
