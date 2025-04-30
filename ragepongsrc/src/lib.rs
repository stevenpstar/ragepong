use godot::prelude::*;
mod core;
mod player;

struct RagePongExtension;

#[gdextension]
unsafe impl ExtensionLibrary for  RagePongExtension {}
