//! Module with the structs and enums needed to call the main function of the library.

#[allow(unused_imports)]
use std::{
    collections::HashMap,
    env::var,
    path::{Path, PathBuf},
};

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

/// Env and ABI used to build the `Rust GDExtension` for `Windows`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// Node icon to use as the default node when none are specified.
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg(any(feature = "find_icons", feature = "simple_find_icons"))]
pub enum DefaultNodeIcon {
    /// When using a custom icon. The path used is relative to the base directory for icons.
    Custom(PathBuf),
    /// When using the icon of the base class of the node. They will always be searched for in the editor directory for icons.
    BaseClass,
    /// When using the [`NODE_RUST`] icon. The path used is relative to the to the base directory for icons, but it's only to the folder that contains the `NodeRust.svg`, it must NOT have "NodeRust.svg" in it.
    NodeRust(PathBuf),
    /// When using the default Godot node icon.
    #[default]
    Node,
}

/// How to copy the files needed for the icons to be displayed.
#[derive(Debug, Default)]
#[cfg(feature = "icons")]
pub struct IconsCopyStrategy {
    /// Whether or not to copy the NodeRust.svg file.
    pub copy_node_rust: bool,
    /// Path to the folder where the icon will be copied relative to the *crate folder*.
    pub path_node_rust: PathBuf,
    /// Whether or not to copy if the files already exist.
    pub force_copy: bool,
}

#[cfg(feature = "icons")]
impl IconsCopyStrategy {
    /// Creates a new instance of [`IconsCopyStrategy`], by giving it all its fields.
    ///
    /// # Parameters
    ///
    /// * `copy_node_rust` - Whether or not to copy the NodeRust.svg file.
    /// * `path_node_rust` - Path to the icon copied relative to the *crate folder*.
    /// * `force_copy` - Whether or not to copy if the files already exist.
    ///
    /// # Returns
    ///
    /// The [`IconsCopyStrategy`] instancte with its fields initialized.
    pub fn new(copy_node_rust: bool, path_node_rust: PathBuf, force_copy: bool) -> Self {
        Self {
            copy_node_rust,
            path_node_rust,
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

/// The **relative** paths of the directories where the icons are stored. They will be stored with [`to_string_lossy`](Path::to_string_lossy), so the directories must be composed of Unicode characters.
#[derive(Debug)]
#[cfg(feature = "icons")]
pub struct IconsDirectories {
    /// The path to the folder **relative** to `{relative_dir.as_str()}` where all the icons are stored. Defaults to the "addons" folder.
    pub base_directory: PathBuf,
    /// The path to the folder **relative** to `{relative_dir.as_str()}{base_directory}` where all the editor icons are stored. Defaults to the "editor" folder inside addons.
    pub editor_directory: PathBuf,
    /// The path to the folder **relative** to `{relative_dir.as_str()}{base_directory}` where all the custom icons for this library are stored. Defaults to the "{crate_name}" folder inside addons.
    pub custom_directory: PathBuf,
    /// The folder to use as a base for the base directory of icons. If [`None`] is provided, the one used to call [`generate_gdextension_file`](crate::generate_gdextension_file) will be used instead.
    pub relative_directory: Option<BaseDirectory>,
}

#[cfg(feature = "icons")]
impl Default for IconsDirectories {
    fn default() -> Self {
        Self {
            base_directory: "addons".into(),
            editor_directory: "editor".into(),
            custom_directory: var("CARGO_PKG_NAME").map_or("rust".into(), |entry_symbol| {
                entry_symbol.replace('-', "_").into()
            }),
            relative_directory: None,
        }
    }
}

#[cfg(feature = "icons")]
impl IconsDirectories {
    /// Creates a new instance of [`IconsDirectories`], by giving it all its fields.
    ///
    /// # Parameters
    ///
    /// * `base_directory` - The path to the folder **relative** to `{relative_dir.as_str()}` where all the icons are stored. Defaults to the "addons" folder.
    /// * `editor_directory` - The path to the folder **relative** to `{relative_dir.as_str()}{base_directory}` where all the editor icons are stored. Defaults to the "editor" folder inside addons.
    /// * `custom_directory` - The path to the folder **relative** to `{relative_dir.as_str()}{base_directory}` where all the custom icons for this library are stored. Defaults to "", so the same as the base directory.
    /// * `relative_directory` - The folder to use as a base for the base directory of icons. If [`None`] is provided, the one used to call [`generate_gdextension_file`](crate::generate_gdextension_file) will be used instead.
    ///
    /// # Returns
    ///
    /// The [`IconsDirectories`] instancte with its fields initialized.
    pub fn new(
        base_directory: PathBuf,
        editor_directory: PathBuf,
        custom_directory: PathBuf,
        relative_directory: Option<BaseDirectory>,
    ) -> Self {
        Self {
            base_directory,
            editor_directory,
            custom_directory,
            relative_directory,
        }
    }
}

/// The icon configuration for the `.gdextension` file generation.
#[derive(Default, Debug)]
#[cfg(feature = "icons")]
pub struct IconsConfig {
    /// The default icon to use when no specified icon was provided.
    #[cfg(any(feature = "find_icons", feature = "simple_find_icons"))]
    pub default: DefaultNodeIcon,
    /// The [`IconsCopyStrategy`] for the files needed for the icons to be displayed.
    pub copy_strategy: IconsCopyStrategy,
    /// The custom icons to use. It contains pairs of `ClassName: IconPath`, where IconPath is the path **relative** to the `custom_directory` specified in `directories`.
    pub custom_icons: Option<HashMap<String, PathBuf>>,
    /// The **relative** paths of the directories where the icons are stored.
    pub directories: IconsDirectories,
}

#[cfg(feature = "icons")]
impl IconsConfig {
    /// Creates a new instance of [`IconsConfig`], by giving it all its fields.
    ///
    /// # Parameters
    ///
    /// * `default` - The default icon to use when no specified icon was provided. If none of the find_icons features are activated, it's not there, and `Godot`'s Node is assumed instead.
    /// * `copy_strategy` - The [`IconsCopyStrategy`] for the files needed for the icons to be displayed.
    /// * `custom_icons` - The custom icons to use. It contains pairs of `ClassName: IconPath`, where IconPath is the path **relative** to the `custom_directory` specified in `directories`.
    /// * `directories` - The **relative** paths of the directories where the icons are stored.
    ///
    /// # Returns
    ///
    /// The [`IconsConfig`] instancte with its fields initialized.
    pub fn new(
        #[cfg(any(feature = "find_icons", feature = "simple_find_icons"))] default: DefaultNodeIcon,
        copy_strategy: IconsCopyStrategy,
        custom_icons: Option<HashMap<String, PathBuf>>,
        directories: IconsDirectories,
    ) -> Self {
        Self {
            #[cfg(any(feature = "find_icons", feature = "simple_find_icons"))]
            default,
            copy_strategy,
            custom_icons,
            directories,
        }
    }
}
