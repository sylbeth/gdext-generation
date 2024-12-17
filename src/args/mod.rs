//! Module with the structs and enums needed to call the main function of the library.

#[cfg(feature = "icons")]
pub mod icons;

use std::env::var;

#[allow(unused_imports)]
use super::gdext::GDExtension;

/// The representation of a path **relative** to the `Godot` project folder.
const PROJECT_FOLDER: &str = "res://";

/// The representation of a path **relative** to the folder where `.gdextension` lies.
const GDEXTENSION_FOLDER: &str = "";

/// The base directory to use for paths.
#[derive(Default, Debug, Clone, Copy)]
pub enum BaseDirectory {
    /// Uses the folder where `project.godot` lies as the base for relative paths. Makes all paths start with `"res://"`.
    #[default]
    ProjectFolder,
    /// Uses the folder where the `.gdextension` file lies as the base for relative paths. Makes all paths start with `""`.
    GDExtensionFolder,
}

impl BaseDirectory {
    /// Retrieves the base directory as the string to start the paths.
    ///
    /// # Returns
    /// "res://" if it is ProjectFolder or "" if it is GDExtensionFolder.
    pub fn as_str(&self) -> &'static str {
        match self {
            BaseDirectory::ProjectFolder => PROJECT_FOLDER,
            BaseDirectory::GDExtensionFolder => GDEXTENSION_FOLDER,
        }
    }
}

/// Name of the default entry function `godot-rust` uses for initializing the [`GDExtension`].
pub const DEFAULT_ENTRY_SYMBOL: &str = "gdext_rust_init";

/// Entry symbol for the [`GDExtension`].
#[derive(Default, Debug, Clone)]
pub enum EntrySymbol {
    /// The default entry symbol to the [`GDExtension`]: [`DEFAULT_ENTRY_SYMBOL`].
    #[default]
    GodotRustDefault,
    /// A generic entry symbol to the [`GDExtension`] based on the crate name in `snake_case`: "lib{crate_name}_init".
    CrateNameBased,
    /// A custom entry symbol to the [`GDExtension`], specified through the String.
    Custom(String),
}

impl ToString for EntrySymbol {
    fn to_string(&self) -> String {
        match self {
            EntrySymbol::GodotRustDefault => DEFAULT_ENTRY_SYMBOL.into(),
            EntrySymbol::CrateNameBased => format!(
                "lib{}_init",
                var("CARGO_PKG_NAME")
                    .map_or("rust".into(), |entry_symbol| entry_symbol.replace('-', "_"))
            ),
            EntrySymbol::Custom(entry_symbol) => entry_symbol.clone(),
        }
    }
}

/// Env and ABI used to build the `Rust GDExtension` for `Windows`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowsABI {
    /// Microsoft Visual C++ compiler.
    #[default]
    MSVC,
    /// The `MinGW` compiler (`MSYS2` port of `GCC`).
    MinGW,
    /// Similar to `MinGW` but using `UCRT` as the runtime and various `LLVM` tools/libraries instead of `GCC/Binutils`. More information: <https://doc.rust-lang.org/rustc/platform-support/pc-windows-gnullvm.html>
    LLVM,
}

impl WindowsABI {
    /// Gets the name of the [`WindowsABI`] used in `Rust` target triples.
    ///
    /// # Returns
    ///
    /// The name of the [`WindowsABI`] for the `Rust` target triple.
    pub fn get_rust_name(&self) -> &'static str {
        match self {
            Self::MSVC => "msvc",
            Self::MinGW => "gnu",
            Self::LLVM => "gnullvm",
        }
    }
}