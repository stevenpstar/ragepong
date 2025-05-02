use godot::prelude::*;
mod core;
mod player;
mod effects;

struct RagePongExtension;

#[gdextension]
unsafe impl ExtensionLibrary for  RagePongExtension {}
