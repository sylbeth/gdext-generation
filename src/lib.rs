//! # GDExtension loading file generation for Rust
//!
//! This crate aims to provide a way to autogenerate a `.gdextension` file for using `Rust` to make a `Godot` `GDExtension`. It provides all the libraries pathfinding and a way to automatically link the default icons to the new defined classes based on the class they inherit from, and also a way to manually link a specific class with a custom icon. For more information, read the [documentation](https://docs.rs/gdext-gen), or the [source code](https://github.com/sylbeth/gdext-generation).
//!
//! # Installation
//!
//! To install this crate as a build dependency in your own crate, run: `cargo add --build gdext-gen`. If you instead want it added as a normal dependency run: `cargo add gdext-gen`.
//!
//! # Usage
//!
//! ## build.rs call
//!
//! To get all the functionality of this crate, in your `build.rs` file on the root of your crate (not your `src/`), write the following (parameters may vary depending on the features you've opt in or out of):
//!
//! ```rust
//! use gdext_gen::prelude::*;
//! fn main() {
//!     // All your variable initialization and setup goes here.
//!     generate_gdextension_file(base_dir, target_dir, gdextension_path, force_generation, configuration, windows_abi, icons_configuration, dependencies);
//! }
//! ```
//!
//! The parameters of this function and how it works are documented in the docs. It should be kept in mind that this function returns an `std::io::Result`, so the following code could be used instead:
//!
//! ```rust
//! use std::io::Result;
//! use gdext_gen::prelude::*;
//!
//! fn main() -> Result<()> {
//!     // All your variable initialization and setup goes here.
//!     generate_gdextension_file(base_dir, target_dir, gdextension_path, force_generation, configuration, windows_abi, icons_configuration, dependencies)?;
//! }
//! ```
//!
//! To compile for `Android`, `Web`, `MacOS` or `iOS` please refer to the [`godot-rust` book](https://godot-rust.github.io/book/toolchain/index.html).
//!
//! It's worth noting that one can configure when the build script will be run, so it's sensible to change it were one not to need it running at every source file change.
//!
//! ## Variable initialization
//!
//! An example of variable initialization to have parity with the `godot-rust` example is the following (with all the primaty features enabled and `checked_generation` chosen):
//!
//! ```rust
//! use std::io::Result;
//! use gdext_gen::prelude::*;
//! fn main() -> Result<()> {
//!     generate_gdextension_file(
//!         BaseDirectory::ProjectFolder.into(),
//!         Some("../rust/target".into()),
//!         Some("../godot/rust.gdextension".into()),
//!         true,
//!         Some(Configuration::new(
//!             EntrySymbol::GodotRustDefault,
//!             Some((4, 1)),
//!             None,
//!             true,
//!             false,
//!         )),
//!         Some(WindowsABI::MSVC),
//!         Some(IconsConfig::new(
//!             DefaultNodeIcon::NodeRust(NodeRust::Ferris, "rust".into()),
//!             IconsCopyStrategy::new(true, true, "../godot/addons/rust".into(), false),
//!             None,
//!             IconsDirectories::new("addons".into(), "editor".into(), "rust".into(), BaseDirectory::ProjectFolder.into()),
//!         )),
//!         None,
//!     )?;
//!
//!     Ok(())
//! }
//! ```
//! This results in a "rust.gdextension" file in "Project/godot", which contains the following:
//! ```toml
//! [configuration]
//! entry_symbol = "gdext_rust_init"
//! compatibility_minimum = 4.1
//! reloadable = true
//!
//! [libraries]
//! "target.mode" = "res://../rust/target/mode/library.file"
//! "target.mode.architecture" = "res://../rust/target/target-triple/mode/library.file"
//! ...
//!
//! [icons]
//! YourStructName = "res://addons/rust/NodeRust.svg"
//! ...
//! ```
//!
//! Few lines of code for a customized automated `.gdextension` file, in conclusion.
//!
//! ## Variables short explanation
//!
//! Based on the last example, the GDExtension is configured as follows:
//! - `BaseDirectory::ProjectFolder` uses `"res://"` based paths.
//! - `target_dir = "../rust/target"`: The target folder for the GDExtension crate is found at `"res://../rust/target"`.
//! - `gdextension_path = "../godot/rust.gdextension`: Makes the file at `"Project/godot/rust.gdextension"` (if `"rust"` and `"godot"` are in a `"Project"` folder).
//! - `true` here means the `.gdextension` will be rewritten even if the file already exists.
//! - `EntrySymbol::GodotRustDefault` defaults to `"gdext_rust_init"`.
//! - `minimum_compatibility` -> 4.1 and `reloatable =  true`
//! - `WindowsABI::MSVC` uses `MSVC` as linker and environment when compiling for `Windows`.
//! - `DefaultNodeIcon::NodeRust(NodeRust::Ferris, "rust")` uses the `NodeRustFerris.svg` icon and finds it in the folder `"res://{base_dir}/rust"`.
//! - IconsCopyStrategy: true, copy the `NodeRust` (and true) file**s** in path `"../godot/addons/rust"` relative to your crate and if it's there, don't copy it again.
//! - No custom nodes.
//! - The directories will be laid out as following:
//!     - All icons will be found relative to `"res://addons"`.
//!     - The editor icons will be located in `"res://addons/editor"`.
//!     - The custom nodes will be located in `"res://addons/rust"`
//! - None: No dependencies.
//!
//! # Features
//!
//! - `icons` - Allows the use of custom icons and the copying of `Rust`'s default icons for the generation of the `icons` section of the `.gdextension` file.
//! - `find_icons` - Allows for the finding of the names of the custom implemented nodes and their subclasses using regex to automate the `icons` section generation process.
//! - `dependencies` - Allows for the generation of the `dependencies` section of the `.gdextension` file.
//! - `checked_generation` - Adds a parameter to the function call to allow for specifying whether the `.gdextension` file should always be copied or only when it doesn't exist. This option is mutually exclusive with `forced_generation`. If none is chosen, it defaults to writing it only when it doesn't exist.
//! - `forced_generation` - Ensures the `.gdextension` file is always written regardless of whether it exists or not. This option is mutually exclusive with `checked_generation`. If none is chosen, it defaults to writing it only when it doesn't exist.
//!
//! # Limitations
//!
//! The feature "find_icons" uses regex to do its work. It's not a perfect way of finding the icons for each GDExtension custom node, but it always resets after each file, so one file's contents failing can only affect itself. It does so by searching for lines that contain both `"base"` and `"="`, then trying to find the name of the base. Same with `"struct"`. The only ways it could fail is if that exact appearance is in a comment or string, has comments in between or extends over more than a line. I believe these to be reasonable compromises, as searching for more than these would only make the code slower, and any reasonably formatted code would have `"base ="` in the same line and for `"base = NameBase"`, or struct `"NameStruct {"` to appear on their own in a comment is hard enough, and the auto found icons can ALWAYS be overriden by custom icons that just happen to be the editor's. In any case, if one thinks otherwise, here are other ways to implement this. 1: A pretty barebones Rust parser, 2: Preprocessing strings and comments in a file before doing the search, 3: Searching for the `impl INameOfBase for StructName`. If you experience problems due to this fact, due let us know, there may be a fix for it.
//!
//! There is also an issue with structs that use generics, or structs that don't follow the standard. These, may not be found at all, so it's best to just add them as custom.
//!
//! # Acknowledgements
//!
//! - This crate is based on the [`gdextension_file` documentation](https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/gdextension_file.html) from [`Godot`](https://godotengine.org/), and some snippets of the documentation (all written by [paddy-exe](https://github.com/paddy-exe)) are taken as are from their documentation, so they are as accurate as possible. The copyright notices for those files can be found directly in their [repository](https://github.com/godotengine/godot/blob/master/COPYRIGHT.txt), and are licensed under the [`CC BY 3.0`](https://creativecommons.org/licenses/by/3.0/) license. This applies to the doc comments on the serializable structs, so these are not relicensed under the licenses of this repository. The schema for the `.gdextension` file comes from the `Godot Engine` which is licensed under the [`MIT`](https://github.com/godotengine/godot/blob/master/LICENSE.txt) license.
//! - This crate is meant to work in tandem with [`godot-rust`](https://godot-rust.github.io/) to give the most painless use of [`Rust`](https://www.rust-lang.org/) for `Godot`'s `GDExtension`, automating a helpful bunch of the work. It could be use on its own, just to generate the `.gdextension` file, but it works best with it.
//! - The explanation on the `WindowsABI::LLVM` documentation, taken from the [`rustc` documentation](https://doc.rust-lang.org/rustc/platform-support/pc-windows-gnullvm.html), which is licensed under the [`MIT`](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) license.
//! ## Asset licenses
//! - The default GDExt Rust node's icons, `NodeRustSmall.svg`, `NodeRustLarge.svg` and `NodeRustFerris.svg` are licensed under the [CC BY 4.0 license](https://creativecommons.org/licenses/by/4.0/), copyright by [burritobandit28](https://github.com/burritobandit28), so they is not relicensed under the licenses of this repository. They are derived from the following works:
//! - `Rust` `Ferris`, made by [Karen Rustad Tölva](rustacean.net) and licensed under the [`CC0 1.0 Universal`](https://creativecommons.org/publicdomain/zero/1.0/) license.
//! - `Ferris` emoji, made by [Dzuk](https://weirder.earth/@dzuk) and licensed under the [`CC BY-NC-SA 4.0`](https://creativecommons.org/licenses/by-nc-sa/4.0/) license.
//! - `Godot` logo, made by [Andrea Calabró](https://godotengine.org) and licensed under the [`CC BY 4.0`](https://creativecommons.org/licenses/by/4.0/) license.
//! - `godot-rust` `Ferris`, licensed under the [`CC BY-NC-SA 4.0`](https://creativecommons.org/licenses/by-nc-sa/4.0) license, from [`godot-rust`](godot-rust.github.io).

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use std::{
    env::var,
    ffi::OsString,
    fs::File,
    io::{Error, ErrorKind, Result, Write},
    path::PathBuf,
};

use args::{BaseDirectory, EntrySymbol, WindowsABI};
use gdext::{config::Configuration, GDExtension};

#[cfg(feature = "dependencies")]
use features::target::Target;
#[cfg(feature = "dependencies")]
use std::collections::HashMap;
#[cfg(feature = "dependencies")]
use toml_edit::{table as toml_table, value as toml_value, DocumentMut};

#[cfg(feature = "icons")]
use args::IconsConfig;

pub mod args;
pub mod features;
pub mod gdext;
pub mod prelude {
    #[cfg(feature = "find_icons")]
    pub use super::args::{DefaultNodeIcon, NodeRust};
    #[cfg(feature = "icons")]
    pub use super::args::{IconsConfig, IconsCopyStrategy, IconsDirectories};
    pub use super::{
        args::{BaseDirectory, EntrySymbol, WindowsABI},
        features::{arch::Architecture, mode::Mode, sys::System, target::Target},
        gdext::config::Configuration,
        generate_gdextension_file,
    };
}

#[cfg(all(
    feature = "checked_generation",
    feature = "forced_generation",
    not(docsrs)
))]
compile_error!("The features that select the kind of generation are mutually exclusive, you either use the checked or the forced one, but you can't use both. Deactivate \"checked_generation\" or \"forced_generation\".");

/// SVG representations of the default GDExtension Rust nodes.
///
/// # Author
/// [burritobandit28](https://github.com/burritobandit28)
///
/// # License
/// [CC BY 4.0 license](https://creativecommons.org/licenses/by/4.0/)
#[cfg(feature = "icons")]
const NODES_RUST: [&str; 3] = [
    include_str!("assets/NodeRustSmall.svg"),
    include_str!("assets/NodeRustLarge.svg"),
    include_str!("assets/NodeRustFerris.svg"),
];

/// Name of the NodeRust files.
#[cfg(feature = "icons")]
pub const NODES_RUST_FILENAMES: [&str; 3] = [
    "NodeRustSmall.svg",
    "NodeRustLarge.svg",
    "NodeRustFerris.svg",
];

/// Generates the `.gdextension` file for the crate using all the necessary information.
///
/// # Parameters
///
/// * `base_dir` - The base directory to use for the paths in the `.gdextension` file.
/// * `target_dir` - Path to the target directory of the crate, **relative** to the *`base_dir`*. If [`None`] is provided, defaults to `"../rust/target"`, the path provided in the `godot-rust` book.
/// * `gdextension_path` - Path where the `.gdextension` file will be written in, **relative** to the *crate folder*. If [`None`] is provided, defaults to `"../godot/rust.gdextension"`, the path provided in the `godot-rust` book.
/// * `force_generation` - Whether or not to generate the file even if it already exists. Available with feature "checked_generation".
/// * `configuration` - [`Configuration`] section of the `.gdextension` file. If [`None`] is provided, defaults to the one found in the `godot-rust` book.
/// * `windows_abi` - `ABI` used when compiling the crate for `Windows`. If [`None`] is provided, defaults to [`MSVC`](WindowsABI::MSVC), the default for `Rust` in `Windows`.
/// * `icons_configuration` - Configuration for the generation of the icon section of the `.gdextension` file. If [`None`] is provided, it doesn't generate the icons section. Available with feature "icons".
/// * `dependencies` - Configuration for the generation of the dependencies section of the `.gdextension` file, comprised of the targets that have dependencies and the paths (**relative** to the *`base_dir`*) of all the dependencies. If [`None`] is provided, it doesn't generate the dependencies section. Available with feature "dependencies".
///
/// # Returns
/// * [`Ok`] - If the generation was successful and no IO errors or TOML errors happened.
/// * [`Err`] - If there has been a problem writing or serializing the TOML file, copying the necessary icons or reading the source to find the associations `ClassName: IconPath` for the icons.
pub fn generate_gdextension_file(
    base_dir: BaseDirectory,
    target_dir: Option<PathBuf>,
    gdextension_path: Option<PathBuf>,
    #[cfg(feature = "checked_generation")] force_generation: bool,
    configuration: Option<Configuration>,
    windows_abi: Option<WindowsABI>,
    #[cfg(feature = "icons")] icons_configuration: Option<IconsConfig>,
    #[cfg(feature = "dependencies")] dependencies: Option<HashMap<Target, Vec<PathBuf>>>,
) -> Result<()> {
    // Default values for the parameters.

    // If the generation is neither forced nor checked, it's assumed to only be written when no file exists.
    #[cfg(not(any(feature = "forced_generation", feature = "checked_generation")))]
    let force_generation = true;

    // Defaults to the provided path in the `godot-rust` book.
    let gdextension_path = if let Some(gdextension_path) = gdextension_path {
        if let Some(extension) = gdextension_path.extension() {
            if extension != "gdextension" {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "The extension of the file must be gdextension.",
                ));
            }
        } else if gdextension_path
            .file_name()
            .unwrap_or(OsString::from("").as_os_str())
            != ".gdextension"
        {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "The path to the gdextension file must lead to .gdextension a file.",
            ));
        }
        gdextension_path
    } else {
        PathBuf::from_iter(["..", "godot", "rust.gdextension"])
    };

    // If the generation is not forced and the file exists.
    #[cfg(not(feature = "forced_generation"))]
    if !force_generation & gdextension_path.exists() {
        return Ok(());
    }

    // Name of the library in snake_case.
    let lib_name =
        var("CARGO_PKG_NAME").map_or("rust".into(), |entry_symbol| entry_symbol.replace('-', "_"));

    // Defaults to the provided path in the `godot-rust` book.
    let target_dir = target_dir.unwrap_or(PathBuf::from_iter(["..", "rust", "target"]));

    // Defaults to the provided configuration in the `godot-rust`.
    let configuration = configuration.unwrap_or(Configuration::new(
        EntrySymbol::GodotRustDefault,
        Some((4, 1)),
        None,
        true,
        false,
    ));

    // Defaults to `MSVC` since it's `Rust`'s default too.
    let windows_abi = windows_abi.unwrap_or(WindowsABI::MSVC);

    let mut gdextension = GDExtension::from_config(configuration);

    gdextension.generate_libs(base_dir, lib_name.as_str(), windows_abi, target_dir);

    #[cfg(feature = "icons")]
    if let Some(mut icons_configuration) = icons_configuration {
        if icons_configuration.directories.relative_directory.is_none() {
            icons_configuration.directories.relative_directory = Some(base_dir)
        }
        gdextension.generate_icons(icons_configuration)?;
    }

    // A TOML Error gets associated with the InvalidData IO ErrorKind.
    #[allow(unused_mut)]
    let mut toml_string = match toml::to_string_pretty(&gdextension) {
        Ok(toml) => toml,
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
    };

    #[cfg(feature = "dependencies")]
    if let Some(dependencies) = dependencies {
        let mut toml_document = toml_string
            .parse::<DocumentMut>()
            .expect("Invalid toml that was just parsed.");

        toml_document["dependencies"] = toml_table();

        for (target, dependencies) in GDExtension::generate_deps(base_dir, dependencies) {
            toml_document["dependencies"][target] = toml_value(dependencies);
        }

        toml_document["dependencies"]
            .as_table_like_mut()
            .expect("The dependencies are a table, it should be tablelike.")
            .sort_values();

        // Newline after sections.
        /*for (_, table) in toml_document.iter_mut() {
            table.as_table_mut().unwrap().decor_mut().set_suffix("\n");
        }*/

        toml_string = toml_document.to_string();
    }

    File::create(gdextension_path)?.write(toml_string.as_bytes())?;

    Ok(())
}
