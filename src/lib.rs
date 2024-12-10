//! Library that aims to provide a way to autogenerate a `.gdextension` file for using `Rust` to make a `Godot` `GDExtension`. It provides all the libraries pathfinding and a way to automatically link the default icons to the new defined classes based on the class they inherit from, and also a way to manually link a specific class with a custom icon.

pub mod args;
pub mod features;
pub mod gdext;
pub mod prelude {
    pub use super::args::WindowsCompiler;
}
/// SVG representation of the default GDExtension Rust node.
///
/// # Author
/// [burritobandit28](https://github.com/burritobandit28)
///
/// # License
/// [CC BY 4.0 license](https://creativecommons.org/licenses/by/4.0/)
const NODE_RUST: &str = include_str!("assets/NodeRust.svg");
