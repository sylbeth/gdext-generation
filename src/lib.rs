//! Library that aims to provide a way to autogenerate a `.gdextension` file for using `Rust` to make a `Godot` `GDExtension`. It provides all the libraries pathfinding and a way to automatically link the default icons to the new defined classes based on the class they inherit from, and also a way to manually link a specific class with a custom icon.

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
    #[cfg(any(feature = "find_icons", feature = "simple_find_icons"))]
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

#[cfg(all(feature = "find_icons", feature = "simple_find_icons"))]
compile_error!("The features that enable the finding of icons are mutually exclusive, you either use the regex or the language parser, but you can't use both. Deactivate \"find_icons\" or \"simple_find_icons\".");
#[cfg(all(feature = "checked_generation", feature = "forced_generation"))]
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
    "NodeRustFerris.svg"
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
