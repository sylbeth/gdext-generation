//! Module with the structs and enums

use std::{collections::HashMap, env::var, path::PathBuf};

use super::BaseDirectory;

/// Represents one of the three avilable default nodes for Rust.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(usize)]
#[cfg(feature = "find_icons")]
pub enum NodeRust {
    /// Small version of the icon based on the `godot-rust` logo.
    #[default]
    Small,
    /// Large version of the icon based on the `godot-rust` logo.
    Large,
    /// Icon based on `Rust`'s Ferris.
    Ferris,
}

/// Node icon to use as the default node when none are specified.
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg(feature = "find_icons")]
pub enum DefaultNodeIcon {
    /// When using a custom icon. The path used is relative to the base directory for icons.
    Custom(PathBuf),
    /// When using the icon of the base class of the node. They will always be searched for in the editor directory for icons.
    BaseClass,
    #[allow(rustdoc::private_intra_doc_links)]
    /// When using one of the [`NODES_RUST`](crate::NODES_RUST) icon. The path used is relative to the to the base directory for icons, but it's only to the folder that contains the `NodeRust` files, it must NOT have the filename in it.
    NodeRust(NodeRust, PathBuf),
    /// When using the default Godot node icon.
    #[default]
    Node,
}

/// How to copy the files needed for the icons to be displayed.
#[derive(Default, Debug)]
pub struct IconsCopyStrategy {
    /// Whether or not to copy the `NodeRust` file. Available with "find_icons" feature.
    #[cfg(feature = "find_icons")]
    pub copy_node_rust: bool,
    /// Whether or not to copy all the `NodeRust` files.
    pub copy_all: bool,
    /// Path to the folder where the icon will be copied relative to the *crate folder*.
    pub path_node_rust: PathBuf,
    /// Whether or not to copy if the files already exist.
    pub force_copy: bool,
}

impl IconsCopyStrategy {
    /// Creates a new instance of [`IconsCopyStrategy`], by giving it all its fields.
    ///
    /// # Parameters
    ///
    /// * `copy_node_rust` - Whether or not to copy the NodeRust.svg file. Available with "find_icons" feature.
    /// * `copy_all` - Whether or not to copy all the `NodeRust` files.
    /// * `path_node_rust` - Path to the icon copied relative to the *crate folder*.
    /// * `force_copy` - Whether or not to copy if the files already exist.
    ///
    /// # Returns
    ///
    /// The [`IconsCopyStrategy`] instancte with its fields initialized.
    pub fn new(
        #[cfg(feature = "find_icons")] copy_node_rust: bool,
        copy_all: bool,
        path_node_rust: PathBuf,
        force_copy: bool,
    ) -> Self {
        Self {
            #[cfg(feature = "find_icons")]
            copy_node_rust,
            copy_all,
            path_node_rust,
            force_copy,
        }
    }

    /// Changes the `copy_node_rust` field to `true` and returns the same struct.
    ///
    /// # Returns
    ///
    /// The same [`IconsCopyStrategy`] it was passed to it with `copy_node_rust` set to `true`.
    #[cfg(feature = "find_icons")]
    pub fn copying_node_rust(mut self) -> Self {
        self.copy_node_rust = true;

        self
    }

    /// Changes the `path_node_rust` field to the one indicated and returns the same struct.
    ///
    /// # Parameters
    ///
    /// * `path_node_rust` - Path to the icon copied relative to the *crate folder*.
    ///
    /// # Returns
    ///
    /// The same [`IconsCopyStrategy`] it was passed to it with `path_node_rust` set to one passed by parameter.
    pub fn with_path_node_rust(mut self, path_node_rust: PathBuf) -> Self {
        self.path_node_rust = path_node_rust;

        self
    }

    /// Changes the `copy_all` field to `true` and returns the same struct.
    ///
    /// # Returns
    ///
    /// The same [`IconsCopyStrategy`] it was passed to it with `copy_node_rust` set to `true`.
    pub fn copying_all(mut self) -> Self {
        self.copy_all = true;

        self
    }

    /// Changes the `force_copy` field to `true` and returns the same struct.
    ///
    /// # Returns
    ///
    /// The same [`IconsCopyStrategy`] it was passed to it with `force_copy` set to `true`.
    pub fn forcing_copy(mut self) -> Self {
        self.force_copy = true;

        self
    }
}

/// The **relative** paths of the directories where the icons are stored. They will be stored with [`to_string_lossy`](std::path::Path::to_string_lossy), so the directories must be composed of Unicode characters.
#[derive(Debug)]
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
    /// The [`IconsDirectories`] instance with its fields initialized.
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

    /// Creates a new instance of [`IconsDirectories`], by giving it the necessary [`PathBuf`] fields. It assumes the default as the base these folders are relative to, [`BaseDirectory::ProjectFolder`].
    ///
    /// # Parameters
    ///
    /// * `base_directory` - The path to the folder **relative** to `{relative_dir.as_str()}` where all the icons are stored. Defaults to the "addons" folder.
    /// * `editor_directory` - The path to the folder **relative** to `{relative_dir.as_str()}{base_directory}` where all the editor icons are stored. Defaults to the "editor" folder inside addons.
    /// * `custom_directory` - The path to the folder **relative** to `{relative_dir.as_str()}{base_directory}` where all the custom icons for this library are stored. Defaults to "", so the same as the base directory.
    ///
    /// # Returns
    ///
    /// The [`IconsDirectories`] instance with its fields initialized.
    pub fn from_directories(
        base_directory: PathBuf,
        editor_directory: PathBuf,
        custom_directory: PathBuf,
    ) -> Self {
        Self {
            base_directory,
            editor_directory,
            custom_directory,
            relative_directory: None,
        }
    }

    /// Modifies the instance of [`IconsDirectories`], by giving it the necessary [`PathBuf`] fields.
    ///
    /// # Parameters
    ///
    /// * `base_directory` - The path to the folder **relative** to `{relative_dir.as_str()}` where all the icons are stored. Defaults to the "addons" folder.
    /// * `editor_directory` - The path to the folder **relative** to `{relative_dir.as_str()}{base_directory}` where all the editor icons are stored. Defaults to the "editor" folder inside addons.
    /// * `custom_directory` - The path to the folder **relative** to `{relative_dir.as_str()}{base_directory}` where all the custom icons for this library are stored. Defaults to "", so the same as the base directory.
    ///
    /// # Returns
    ///
    /// The [`IconsDirectories`] instance with its directories changed.
    pub fn with_directories(
        mut self,
        base_directory: PathBuf,
        editor_directory: PathBuf,
        custom_directory: PathBuf,
    ) -> Self {
        self.base_directory = base_directory;
        self.editor_directory = editor_directory;
        self.custom_directory = custom_directory;
        self
    }

    /// Modifies the instance of [`IconsDirectories`], by giving it the `base_directory` field.
    ///
    /// # Parameters
    ///
    /// * `base_directory` - The path to the folder **relative** to `{relative_dir.as_str()}` where all the icons are stored. Defaults to the "addons" folder.
    pub fn with_base_directory(mut self, base_directory: PathBuf) -> Self {
        self.base_directory = base_directory;
        self
    }

    /// Modifies the instance of [`IconsDirectories`], by giving it the `editor_directory` field.
    ///
    /// # Parameters
    ///
    /// * `editor_directory` - The path to the folder **relative** to `{relative_dir.as_str()}{base_directory}` where all the editor icons are stored. Defaults to the "editor" folder inside addons.
    pub fn with_editor_directory(mut self, editor_directory: PathBuf) -> Self {
        self.editor_directory = editor_directory;
        self
    }

    /// Modifies the instance of [`IconsDirectories`], by giving it the `custom_directory` field.
    ///
    /// # Parameters
    ///
    /// * `custom_directory` - The path to the folder **relative** to `{relative_dir.as_str()}{base_directory}` where all the custom icons for this library are stored. Defaults to "", so the same as the base directory.
    pub fn with_custom_directory(mut self, custom_directory: PathBuf) -> Self {
        self.custom_directory = custom_directory;
        self
    }

    /// Modifies the instance of [`IconsDirectories`], by giving it the `relative_directory` field.
    ///
    /// # Parameters
    ///
    /// * `relative_directory` - The folder to use as a base for the base directory of icons.
    pub fn with_relative_directory(mut self, relative_directory: BaseDirectory) -> Self {
        self.relative_directory = Some(relative_directory);
        self
    }
}

/// The icon configuration for the `.gdextension` file generation.
#[derive(Default, Debug)]
pub struct IconsConfig {
    /// The default icon to use when no specified icon was provided. Available with "find_icons" feature.
    #[cfg(feature = "find_icons")]
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
    /// * `default` - The default icon to use when no specified icon was provided. If none of the find_icons features are activated, it's not there, and `Godot`'s Node is assumed instead. Available with feature "find_icons".
    /// * `copy_strategy` - The [`IconsCopyStrategy`] for the files needed for the icons to be displayed.
    /// * `custom_icons` - The custom icons to use. It contains pairs of `ClassName: IconPath`, where IconPath is the path **relative** to the `custom_directory` specified in `directories`.
    /// * `directories` - The **relative** paths of the directories where the icons are stored.
    ///
    /// # Returns
    ///
    /// The [`IconsConfig`] instancte with its fields initialized.
    pub fn new(
        #[cfg(feature = "find_icons")] default: DefaultNodeIcon,
        copy_strategy: IconsCopyStrategy,
        custom_icons: Option<HashMap<String, PathBuf>>,
        directories: IconsDirectories,
    ) -> Self {
        Self {
            #[cfg(feature = "find_icons")]
            default,
            copy_strategy,
            custom_icons,
            directories,
        }
    }
}
