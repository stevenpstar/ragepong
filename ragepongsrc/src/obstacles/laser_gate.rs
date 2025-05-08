use godot::{builtin::Color, classes::{Area2D, ColorRect, IArea2D}, global::godot_print, obj::{Base, Gd, OnReady, WithBaseField}, prelude::{godot_api, GodotClass}};

use crate::core::{colour_component::ColourComponent, colours::Colour};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct LaserGate {
    #[export]
    open: bool,
    #[export]
    colour_rect: Option<Gd<ColorRect>>,
    #[export]
    starting_colour: Colour,
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
            colour_rect: None,
            starting_colour: Colour::White,
            is_open: false,
            base,
        }
    }

    fn ready(&mut self) {
        self.is_open = self.open;
        let c = Colour::get_colour(&self.starting_colour);
        self.set_gate_colour(&c);
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

    fn set_gate_colour(&mut self, colour: &Colour) {

        self.colour.bind_mut().set_obj_colour(colour);

        let colour_rect = match &mut self.colour_rect {
            None => {
                godot_print!("gate should have a colour rect");
                panic!("No gate rect for player!");
            },
            Some(spr) => spr
        };

        match colour {
            Colour::White => colour_rect.set_modulate(Color::from_rgb(1.0, 1.0, 1.0)),
            Colour::Red => colour_rect.set_modulate(Color::from_rgb(1.0, 0.0, 0.0)),
            Colour::Blue => colour_rect.set_modulate(Color::from_rgb(0.0, 0.0, 1.0)),
            Colour::Green => colour_rect.set_modulate(Color::from_rgb(0.0, 1.0, 0.0)),
        };
    }


}
