use godot::prelude::*;
mod core;
mod player;
mod effects;
mod obstacles;

struct RagePongExtension;

#[gdextension]
unsafe impl ExtensionLibrary for  RagePongExtension {}
