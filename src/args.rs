//! Module with the structs and enums needed to call the main function of the library.

use std::{collections::HashMap, env::var, path::PathBuf};

#[allow(unused_imports)]
use super::{gdext::GDExtension, NODE_RUST};

/// ABI used to build the `Rust GDExtension` for `Windows`.
#[derive(Debug, Clone, Copy)]
pub enum WindowsABI {
    /// Microsoft Visual C++ compiler.
    MSVC,
    /// The `MinGW` compiler (`MSYS2` port of `GCC`).
    MinGW,
    /// Similar to `MinGW` but using `UCRT` as the runtime and various `LLVM` tools/libraries instead of `GCC/Binutils`. More information: https://doc.rust-lang.org/rustc/platform-support/pc-windows-gnullvm.html
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

/// Name of the default entry function `godot-rust` uses for initializing the [`GDExtension`].
pub const DEFAULT_ENTRY_SYMBOL: &str = "gdext_rust_init";

/// Entry symbol for the [`GDExtension`].
#[derive(Debug, Clone)]
pub enum EntrySymbol {
    /// The default entry symbol to the [`GDExtension`]: [`DEFAULT_ENTRY_SYMBOL`].
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

/// Node icon to use as the default node when none are specified.
#[derive(Default, Debug, Clone, Copy)]
pub enum DefaultNodeIcon {
    /// When using the icon of the base class of the node.
    BaseClass,
    /// When using the [`NODE_RUST`] icon.
    NodeRust,
    /// When using the default Godot node icon.
    #[default]
    Node,
}

/// How to copy the files needed for the icons to be displayed.
#[derive(Debug, Default)]
pub struct IconsCopyStrategy {
    /// Whether or not to copy the NodeRust.svg file.
    pub copy_node_rust: bool,
    /// Whether or not to copy the Godot editor icons.
    pub copy_editor_icons: bool,
    /// Whether or not to copy if the files already exist.
    pub force_copy: bool,
}

impl IconsCopyStrategy {
    /// Creates a new instance of [`IconsCopyStrategy`], by giving it all its fields.
    ///
    /// # Parameters
    ///
    /// * `copy_node_rust` - Whether or not to copy the NodeRust.svg file.
    /// * `copy_editor_icons` - Whether or not to copy the Godot editor icons.
    /// * `force_copy` - Whether or not to copy if the files already exist.
    ///
    /// # Returns
    ///
    /// The [`IconsCopyStrategy`] instancte with its fields initialized.
    pub fn new(copy_node_rust: bool, copy_editor_icons: bool, force_copy: bool) -> Self {
        Self {
            copy_node_rust,
            copy_editor_icons,
            force_copy,
        }
    }

    /// Changes the `copy_node_rust` field to `true` and returns the same struct.
    ///
    /// # Returns
    ///
    /// The same [`IconsCopyStrategy`] it was passed to it with `copy_node_rust` set to `true`.
    pub fn copy_node_rust(mut self) -> Self {
        self.copy_node_rust = true;

        self
    }

    /// Changes the `copy_editor_icons` field to `true` and returns the same struct.
    ///
    /// # Returns
    ///
    /// The same [`IconsCopyStrategy`] it was passed to it with `copy_editor_icons` set to `true`.
    pub fn copy_editor_icons(mut self) -> Self {
        self.copy_editor_icons = true;

        self
    }

    /// Changes the `force_copy` field to `true` and returns the same struct.
    ///
    /// # Returns
    ///
    /// The same [`IconsCopyStrategy`] it was passed to it with `force_copy` set to `true`.
    pub fn force_copy(mut self) -> Self {
        self.force_copy = true;

        self
    }
}

/// The **relative** paths of the directories where the icons are stored.
#[derive(Debug)]
pub struct IconsDirectories {
    /// The path to the folder **relative** to `res://` where all the icons are stored. Defaults to the "addons" folder.
    pub base_directory: PathBuf,
    /// The path to the folder **relative** to `res://{base_directory}` where all the editor icons are stored. Defaults to the "editor" folder inside addons.
    pub editor_directory: PathBuf,
    /// The path to the folder **relative** to `res://{base_directory}` where all the custom icons for this library are stored. Defaults to "", so the same as the base directory.
    pub custom_directory: PathBuf,
}

impl Default for IconsDirectories {
    fn default() -> Self {
        Self {
            base_directory: "addons".into(),
            editor_directory: "editor".into(),
            custom_directory: "".into(),
        }
    }
}

impl IconsDirectories {
    /// Creates a new instance of [`IconsDirectories`], by giving it all its fields.
    ///
    /// # Parameters
    ///
    /// * `base_directory` - The path to the folder **relative** to `res://` where all the icons are stored. Defaults to the "addons" folder.
    /// * `editor_directory` - The path to the folder **relative** to `res://{base_directory}` where all the editor icons are stored. Defaults to the "editor" folder inside addons.
    /// * `custom_directory` - The path to the folder **relative** to `res://{base_directory}` where all the custom icons for this library are stored. Defaults to "", so the same as the base directory.
    ///
    /// # Returns
    ///
    /// The [`IconsDirectories`] instancte with its fields initialized.
    pub fn new(
        base_directory: PathBuf,
        editor_directory: PathBuf,
        custom_directory: PathBuf,
    ) -> Self {
        Self {
            base_directory,
            editor_directory,
            custom_directory,
        }
    }
}

/// The icon configuration for the `.gdextension` file generation.
#[derive(Default, Debug)]
pub struct IconsConfig {
    /// The default icon to use when no specified icon was provided.
    pub default: DefaultNodeIcon,
    /// The [`IconsCopyStrategy`] for the files needed for the icons to be displayed.
    pub copy_strategy: IconsCopyStrategy,
    /// The custom icons to use. It contains pairs of `ClassName: IconPath`, where IconPath is the path **relative** to the `custom_directory` specified in `directories`.
    pub custom_icons: Option<HashMap<String, PathBuf>>,
    /// The **relative** paths of the directories where the icons are stored.
    pub directories: IconsDirectories,
}

impl IconsConfig {
    /// Creates a new instance of [`IconsConfig`], by giving it all its fields.
    ///
    /// # Parameters
    ///
    /// * `default` - The default icon to use when no specified icon was provided.
    /// * `copy_strategy` - The [`IconsCopyStrategy`] for the files needed for the icons to be displayed.
    /// * `custom_icons` - The custom icons to use. It contains pairs of `ClassName: IconPath`, where IconPath is the path **relative** to the `custom_directory` specified in `directories`.
    /// * `directories` - The **relative** paths of the directories where the icons are stored.
    ///
    /// # Returns
    ///
    /// The [`IconsConfig`] instancte with its fields initialized.
    pub fn new(
        default: DefaultNodeIcon,
        copy_strategy: IconsCopyStrategy,
        custom_icons: Option<HashMap<String, PathBuf>>,
        directories: IconsDirectories,
    ) -> Self {
        Self {
            default,
            copy_strategy,
            custom_icons,
            directories,
        }
    }
}
