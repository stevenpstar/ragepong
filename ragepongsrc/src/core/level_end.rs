use godot::{builtin::GString, classes::{Area2D, INode2D, Node2D}, global::godot_print, obj::{Base, Gd, WithBaseField, WithUserSignals}, prelude::{godot_api, GodotClass}};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct LevelEnd {
    #[export]
    next_level: GString,
    #[export]
    area: Option<Gd<Area2D>>,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for LevelEnd {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            next_level: "level_1.tscn".into(),
            area: None,
            base,
        }
    }

    fn ready(&mut self) {
        let this = self.to_gd();
        // attach signal
        let end_area = match &mut self.area {
            None => {
                godot_print!("No Ending Area defined for level end");
                panic!("No ending area!");
            },
            Some(a) => a
        };

        end_area.signals()
            .body_entered()
            .connect_obj(&this, |s: &mut Self, body| {
                s.on_body_entered(body);
            });
            
    }
}

#[godot_api]
impl LevelEnd {
    #[func]
    pub fn get_level_path(&self) -> GString {
        let base_path = "res://Levels/";
        let level_str = String::from(self.next_level.clone());
        let level_path: GString = format!("{base_path}{level_str}").into();
        return level_path;
    }

    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        if body.get_class() == "Pong".into() {
            let level_path: GString;
            {
                level_path = self.get_level_path();
            }
            godot_print!("Level should end!");
            self.signals().level_ended().emit(level_path.to_string());
        }

    }

    #[signal]
    pub fn level_ended(path: String);
}
