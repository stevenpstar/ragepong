use godot::{builtin::GString, prelude::{Export, GodotConvert, Var}};

#[derive(GodotConvert, Var, Export, PartialEq, Eq)]
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

