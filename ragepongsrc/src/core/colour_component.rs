use godot::{classes::{INode, Node}, obj::Base, prelude::{godot_api, GodotClass}};

use super::colours::Colour;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ColourComponent {
    #[export]
    colour: Colour,
    base: Base<Node>,
}

#[godot_api]
impl INode for ColourComponent {
    fn init(base: Base<Node>) -> Self {
        Self {
            colour: Colour::White,
            base,
        }
    }
}

#[godot_api]
impl ColourComponent {
    pub fn set_obj_colour(&mut self, col: &Colour) {

        let c =  match &col {
            Colour::White => Colour::White,
            Colour::Red => Colour::Red,
            Colour::Blue => Colour::Blue,
            Colour::Green => Colour::Green,
        };

        self.colour = c;
    }

    pub fn get_obj_colour(&self) -> Colour {
        let col =  match &self.colour {
            Colour::White => Colour::White,
            Colour::Red => Colour::Red,
            Colour::Blue => Colour::Blue,
            Colour::Green => Colour::Green,
        };

        return col;
    }
}


