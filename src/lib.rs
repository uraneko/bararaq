#![allow(warnings)]
pub mod cache;
pub mod events;
pub mod kbd_decode;
pub mod mouse_input;
pub mod object_tree;
pub mod presets;
pub mod raw_mode;
pub mod render_pipeline;
pub mod space;
pub mod termbuf;
pub mod themes;

pub use kbd_decode::*;
pub use raw_mode::*;
pub use termbuf::*;

// TODO: text position, vertical/horizontal center, start or end
// TODO: ctrl + l  = clear and render whole term buffer event
// TODO: term switch event
// TODO: menu selection events
// TODO: input objects mevement events
// TODO: emoji selection event
