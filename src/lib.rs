//! Library that aims to provide a way to autogenerate a `.gdextension` file for using `Rust` to make a `Godot` `GDExtension`. It provides all the libraries pathfinding and a way to automatically link the default icons to the new defined classes based on the class they inherit from, and also a way to manually link a specific class with a custom icon.

use std::{
    env::var,
    fs::File,
    io::{Error, ErrorKind, Result, Write},
    path::PathBuf,
};

use args::{EntrySymbol, WindowsABI};
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
    #[cfg(feature = "icons")]
    pub use super::args::{DefaultNodeIcon, IconsConfig, IconsCopyStrategy, IconsDirectories};
    pub use super::{
        args::{EntrySymbol, WindowsABI},
        features::{arch::Architecture, mode::Mode, sys::System, target::Target},
        gdext::config::Configuration,
        generate_gdextension_file,
    };
}

/// The representation of a path **relative** to the `Godot` project folder.
const PROJECT_FOLDER: &str = "res://";

/// SVG representation of the default GDExtension Rust node.
///
/// # Author
/// [burritobandit28](https://github.com/burritobandit28)
///
/// # License
/// [CC BY 4.0 license](https://creativecommons.org/licenses/by/4.0/)
#[cfg(feature = "icons")]
const NODE_RUST: &str = include_str!("assets/NodeRust.svg");

/// Generates the `.gdextension` file for the crate using all the necessary information.
///
/// # Parameters
///
/// * `target_dir` - Path to the target directory of the crate, **relative** to the *`Godot` project folder*. If [`None`] is provided, defaults to `"../rust/target"`, the path provided in the `godot-rust` book.
/// * `gdextension_path` - Path where the `.gdextension` file will be written in, **relative** to the *crate folder*. If [`None`] is provided, defaults to `"../godot/rust.gdextension"`, the path provided in the `godot-rust` book.
/// * `configuration` - [`Configuration`] section of the `.gdextension` file. If [`None`] is provided, defaults to the one found in the `godot-rust` book.
/// * `windows_abi` - `ABI` used when compiling the crate for `Windows`. If [`None`] is provided, defaults to [`MSVC`](WindowsABI::MSVC), the default for `Rust` in `Windows`.
/// * `icons_configuration` - Configuration for the generation of the icon section of the `.gdextension` file. If [`None`] is provided, it doesn't generate the icons section. Available with feature "icons".
/// * `dependencies` - Configuration for the generation of the dependencies section of the `.gdextension` file, comprised of the targets that have dependencies and the paths (**relative** to the *`Godot` project folder*) of all the dependencies. If [`None`] is provided, it doesn't generate the dependencies section. Available with feature "dependencies".
///
/// # Returns
/// * [`Ok`] - If the generation was successful and no IO errors or TOML errors happened.
/// * [`Err`] - If there has been a problem writing or serializing the TOML file, copying the necessary icons or reading the source to find the associations `ClassName: IconPath` for the icons.
pub fn generate_gdextension_file(
    target_dir: Option<PathBuf>,
    gdextension_path: Option<PathBuf>,
    configuration: Option<Configuration>,
    windows_abi: Option<WindowsABI>,
    #[cfg(feature = "icons")] icons_configuration: Option<IconsConfig>,
    #[cfg(feature = "dependencies")] dependencies: Option<HashMap<Target, Vec<PathBuf>>>,
) -> Result<()> {
    // Default values for the parameters.

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

    // Defaults to the provided path in the `godot-rust` book.
    let gdextension_path =
        gdextension_path.unwrap_or(PathBuf::from_iter(["..", "godot", "rust.gdextension"]));

    let mut gdextension = GDExtension::from_config(configuration);

    gdextension.generate_libs(lib_name.as_str(), windows_abi, target_dir);

    #[cfg(feature = "icons")]
    if let Some(icons_configuration) = icons_configuration {
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

        for (target, dependencies) in GDExtension::generate_deps(dependencies) {
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
