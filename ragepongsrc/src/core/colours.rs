use godot::{builtin::GString, prelude::{Export, GodotConvert, Var}};

#[derive(GodotConvert, Var, Export, PartialEq, Eq)]
#[godot(via = GString)]
pub enum Colour {
    White,
    Red,
    Blue,
    Green
}

