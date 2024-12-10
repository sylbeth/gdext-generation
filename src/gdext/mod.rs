//! Module for the definition of the structs to be serialized to build the `.gdextension` file, and the functions to generate the file.

pub mod config;
pub mod deps;
pub mod icons;
pub mod libs;

use serde::{Deserialize, Serialize};
use toml::Table;

use self::config::Configuration;

/// Name of the default entry function `godot-rust` uses for initializing the [`GDExtension`].
pub const DEFAULT_ENTRY_SYMBOL: &str = "gdext_rust_init";

/// `.gdextension` file representation.
#[derive(Deserialize, Serialize, Debug)]
pub struct GDExtension {
    /// Configuration section of the `.gdextension` file.
    configuration: Configuration,
    /// Libraries section of the `.gdextension` file. Links the `godot` target to the compiled [`GDExtension`] libraries. It contains relationships of `godot_target: GDExtensionCdylibPath`.
    libraries: Table,
    /// Icons section of the `.gdextension` file. Links the [`GDExtension`] classes to the files to use as their editor icons. It contains relationships of `ClassName: IconPath`.
    icons: Option<Table>,
    /// Dependencies section of the `.gdextension` file. It contains tables with key `running_system.build_mode`, whose entries are `GDExtensionCdylibPath: dependency`.
    dependencies: Option<Table>,
}

impl GDExtension {
    /// Creates a new instance of [`GDExtension`], with libraries, icons and dependencies empty and with the assigned [`Configuration`].
    ///
    /// # Parameters
    ///
    /// * `configuration` - [`Configuration`] to be assigned to the [`GDExtension`].
    ///
    /// # Returns
    /// 
    /// The [`GDExtension`] with the [`Configuration`] assigned.
    pub fn from_config(configuration: Configuration) -> Self {
        Self {
            configuration,
            libraries: Table::new(),
            icons: None,
            dependencies: None,
        }
    }
}
