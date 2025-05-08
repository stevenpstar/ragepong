use godot::{classes::{Area2D, IArea2D}, obj::{Base, Gd, OnReady, WithBaseField}, prelude::{godot_api, GodotClass}};

use crate::core::{colour_component::ColourComponent, colours::Colour};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct LaserGate {
    #[export]
    open: bool,
    colour: OnReady<Gd<ColourComponent>>,
    is_open: bool,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for LaserGate {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            open: false,
            colour: OnReady::from_node("ColourComponent"),
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

    #[func]
    pub fn get_colour(&mut self) -> Colour {
        return self.colour.bind().get_obj_colour();
    }

}
