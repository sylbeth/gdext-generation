# GDExtension loading file generation for Rust
This crate aims to provide a way to autogenerate a `.gdextension` file for using `Rust` to make a `Godot` `GDExtension`. It provides all the libraries pathfinding and a way to automatically link the default icons to the new defined classes based on the class they inherit from, and also a way to manually link a specific class with a custom icon. For more information, read the [documentation](https://docs.rs/gdext-gen), or the [source code](https://github.com/sylbeth/gdext-generation).

# Installation

To install this crate as a build dependency in your own crate, run: `cargo add --build gdext-gen`. If you instead want it added as a normal dependency run: `cargo add gdext-gen`.

# Acknowledgements

* This crate is based on the [`gdextension_file` documentation](https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/gdextension_file.html) from [`Godot`](https://godotengine.org/), and some snippets of the documentation are taken as are from their documentation, so they are as accurate as possible. The copyright notices for those files can be found directly in their [repository](https://github.com/godotengine/godot/blob/master/COPYRIGHT.txt), and are licensed under the [`CC BY 3.0`](https://creativecommons.org/licenses/by/3.0/) license. This applies to the doc comments on the serializable structs. The schema for the `.gdextension` file comes from the `Godot Engine` which is licensed under the [`MIT`](https://github.com/godotengine/godot/blob/master/LICENSE.txt) license.
* This crate is meant to work in tandem with [`godot-rust`](https://godot-rust.github.io/) to give the most painless use of [`Rust`](https://www.rust-lang.org/) for `Godot`'s `GDExtension`, automating a helpful bunch of the work. It could be use on its own, just to generate the `.gdextension` file, but it works best with it.
* The default GDExt Rust node's icon, `NodeRust.svg` is licensed under the [CC BY 4.0 license](https://creativecommons.org/licenses/by/4.0/), copyright by [burritobandit28](https://github.com/burritobandit28).
