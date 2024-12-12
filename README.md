# GDExtension loading file generation for Rust
This crate aims to provide a way to autogenerate a `.gdextension` file for using `Rust` to make a `Godot` `GDExtension`. It provides all the libraries pathfinding and a way to automatically link the default icons to the new defined classes based on the class they inherit from, and also a way to manually link a specific class with a custom icon. For more information, read the [documentation](https://docs.rs/gdext-gen), or the [source code](https://github.com/sylbeth/gdext-generation).

# Installation

To install this crate as a build dependency in your own crate, run: `cargo add --build gdext-gen`. If you instead want it added as a normal dependency run: `cargo add gdext-gen`.

# Usage

## build.rs call

To get all the functionality of this crate, in your `build.rs` file on the root of your crate (not your `src/`), write the following (parameters may vary depending on the features you've opt in or out of):

```rust
use gdext_gen::generate_gdextension_file;

fn main() {
    // All your variable initialization and setup goes here.
    generate_gdextension_file(base_dir, target_dir, gdextension_path, configuration, windows_abi, icons_configuration, dependencies);
}
```

The parameters of this function and how it works are documented in the docs. It should be kept in mind that this function returns an `std::io::Result`, so the following code could be used instead:

```rust
use std::io::Result;
use gdext_gen::generate_gdextension_file;

fn main() -> Result<()> {
    // All your variable initialization and setup goes here.
    generate_gdextension_file(base_dir, target_dir, gdextension_path, configuration, windows_abi, icons_configuration, dependencies)?;
}
```

To compile for `Android`, `Web`, `MacOS` or `iOS` please refer to the [`godot-rust` book](https://godot-rust.github.io/book/toolchain/index.html).

It's worth noting that one can configure when the build script will be run, so it's sensible to change it were one not to need it running at every source file change.

## Variable initialization

An example of variable initialization to have parity with the `godot-rust` example is the following (with all the primaty features enabled):

```rust
fn main() -> Result<()> {
    generate_gdextension_file(
        BaseDirectory::ProjectFolder.into(),
        Some("../rust/target".into()),
        Some("../godot/rust.gdextension".into()),
        Some(Configuration::new(
            EntrySymbol::GodotRustDefault,
            Some((4, 1)),
            None,
            true,
            false,
        )),
        Some(WindowsABI::MSVC),
        Some(IconsConfig::new(
            DefaultNodeIcon::NodeRust(NodeRust::Ferris, "rust".into()),
            IconsCopyStrategy::new(true, true, "../godot/addons/rust".into(), false),
            None,
            IconsDirectories::new("addons".into(), "editor".into(), "rust".into(), BaseDirectory::ProjectFolder.into()),
        )),
        None,
    )?;

    Ok(())
}
```

This results in a "rust.gdextension" file in "Project/godot", which contains the following:

```toml
[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1
reloadable = true

[libraries]
"target.mode" = "res://../rust/target/mode/library.file"
"target.mode.architecture" = "res://../rust/target/target-triple/mode/library.file"
...

[icons]
YourStructName = "res://addons/rust/NodeRust.svg"
...
```

Few lines of code for a customized automated `.gdextension` file, in conclusion.

## Variables short explanation

Based on the last example, the GDExtension is configured as follows:
- `BaseDirectory::ProjectFolder` uses `"res://"` based paths.
- `target_dir = "../rust/target"`: The target folder for the GDExtension crate is found at `"res://../rust/target"`.
- `gdextension_path = "../godot/rust.gdextension`: Makes the file at `"Project/godot/rust.gdextension"` (if `"rust"` and `"godot"` are in a `"Project"` folder).
- `EntrySymbol::GodotRustDefault` defaults to `"gdext_rust_init"`.
- `minimum_compatibility` -> 4.1 and `reloatable =  true`
- `WindowsABI::MSVC` uses `MSVC` as linker and environment when compiling for `Windows`.
- `DefaultNodeIcon::NodeRust(NodeRust::Ferris, "rust")` uses the `NodeRustFerris.svg` icon and finds it in the folder `"res://{base_dir}/rust"`.
- IconsCopyStrategy: true, copy the `NodeRust` (and true) file**s** in path `"../godot/addons/rust"` relative to your crate and if it's there, don't copy it again.
- No custom nodes.
- The directories will be laid out as following:
  - All icons will be found relative to `"res://addons"`.
  - The editor icons will be located in `"res://addons/editor"`.
  - The custom nodes will be located in `"res://addons/rust"`
- None: No dependencies.

# Limitations

The feature "simple_find_icons" is not a perfect way of finding the icons for each GDExtension custom node, since it doesn't account for comments. If you experience problems due to this fact, due let us know, there may be a fix for it, but "find_icons" is in development to have a parser that will not fail, so consider changing features if you think it's worth it for you.

# Acknowledgements

* This crate is based on the [`gdextension_file` documentation](https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/gdextension_file.html) from [`Godot`](https://godotengine.org/), and some snippets of the documentation (all written by [paddy-exe](https://github.com/paddy-exe)) are taken as are from their documentation, so they are as accurate as possible. The copyright notices for those files can be found directly in their [repository](https://github.com/godotengine/godot/blob/master/COPYRIGHT.txt), and are licensed under the [`CC BY 3.0`](https://creativecommons.org/licenses/by/3.0/) license. This applies to the doc comments on the serializable structs, so these are not relicensed under the licenses of this repository. The schema for the `.gdextension` file comes from the `Godot Engine` which is licensed under the [`MIT`](https://github.com/godotengine/godot/blob/master/LICENSE.txt) license.
* This crate is meant to work in tandem with [`godot-rust`](https://godot-rust.github.io/) to give the most painless use of [`Rust`](https://www.rust-lang.org/) for `Godot`'s `GDExtension`, automating a helpful bunch of the work. It could be use on its own, just to generate the `.gdextension` file, but it works best with it.
* The explanation on the `WindowsABI::LLVM` documentation, taken from the [`rustc` documentation](https://doc.rust-lang.org/rustc/platform-support/pc-windows-gnullvm.html), which is licensed under the [`MIT`](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) license.

## Asset licenses
* The default GDExt Rust node's icons, `NodeRustSmall.svg`, `NodeRustLarge.svg` and `NodeRustFerris.svg` are licensed under the [CC BY 4.0 license](https://creativecommons.org/licenses/by/4.0/), copyright by [burritobandit28](https://github.com/burritobandit28), so they is not relicensed under the licenses of this repository. They are derived from the following works:

* `Rust` `Ferris`, made by [Karen Rustad Tölva](rustacean.net) and licensed under the [`CC0 1.0 Universal`](https://creativecommons.org/publicdomain/zero/1.0/) license.
* `Ferris` emoji, made by [Dzuk](https://weirder.earth/@dzuk) and licensed under the [`CC BY-NC-SA 4.0`](https://creativecommons.org/licenses/by-nc-sa/4.0/) license.
* `Godot` logo, made by [Andrea Calabró](https://godotengine.org) and licensed under the [`CC BY 4.0`](https://creativecommons.org/licenses/by/4.0/) license.
* `godot-rust` `Ferris`, licensed under the [`CC BY-NC-SA 4.0`](https://creativecommons.org/licenses/by-nc-sa/4.0) license, from [`godot-rust`](godot-rust.github.io).
