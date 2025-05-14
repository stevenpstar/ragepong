use godot::{classes:: Object, obj::Base, prelude::{godot_api, GodotClass}};

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct Game {
    resetting: bool,
    base: Base<Object>,
}

#[godot_api]
impl Game {
    pub fn set_resetting(&mut self, r: bool) {
        self.resetting = r;
    }
    pub fn is_resetting(&self) -> bool {
        return self.resetting;
    }
}
