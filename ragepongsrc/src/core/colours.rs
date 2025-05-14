use core::fmt;

use godot::{builtin::GString, prelude::{Export, GodotConvert, Var}};

#[derive(GodotConvert, Var, Export, Debug, PartialEq, Eq)]
#[godot(via = GString)]
pub enum Colour {
    White,
    Red,
    Blue,
    Green
}

impl Colour {
    pub fn get_colour(col: &Colour) -> Colour {
        let c =  match &col {
            Colour::White => Colour::White,
            Colour::Red => Colour::Red,
            Colour::Blue => Colour::Blue,
            Colour::Green => Colour::Green,
        };

        return c;
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Colour::White => write!(f, "White"),
            Colour::Blue => write!(f, "Blue"),
            Colour::Red => write!(f, "Red"),
            Colour::Green => write!(f, "Green"),
        }
    }
}

