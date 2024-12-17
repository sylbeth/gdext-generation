//! Module for the definition of the [`Configuration`] struct for the configuration section of the `.gdextension` file.

use crate::args::EntrySymbol;

#[allow(unused_imports)]
use super::GDExtension;
use serde::{Deserialize, Serialize};

/// Configuration section of the `.gdextension` file.
#[derive(Deserialize, Serialize, Debug)]
pub struct Configuration {
    /// Name of the entry function for initializing the [`GDExtension`]. By default, its name is `"gdext_rust_init"`, but it can be changed by using the attribute `entry_point` (`godot-rust <= 0.2.0`) or `entry_symbol` (`>= 0.2.1`).
    /// # Examples
    /// In lib.rs:
    /// ```
    /// #[gdextension(entry_symbol = libmy_rust_init)]
    /// unsafe impl ExtensionLibrary for MyExtension {}
    /// ```
    entry_symbol: String,
    /// Minimum compatible version of `Godot`. This prevents older versions of `Godot` from loading [`GDExtension`]s that depend on features from newer versions of `Godot`. It's formatted as follows: `<major>.<minor>`.
    compatibility_minimum: Option<f64>,
    /// Maximum compatible version of `Godot`. This prevents newer versions of `Godot` from loading the [`GDExtension`]. It's formatted as follows: `<major>.<minor>`.
    compatibility_maximum: Option<f64>,
    /// Whether or not to allow the reloading of the [`GDExtension`] upon recompilation. Supported only for `Godot 4.2` and later. Meant generally for development and debug purposes, and it can fail, it always is safer to close and reopen the engine, but it's a good quality of life feature in general.
    reloadable: Option<bool>,
    /// The [`GDExtension`] is part of a `v2 Android` plugin. During export this flag will indicate to the editor that the [`GDExtension`] native shared libraries are exported by the `Android` plugin `AAR` binaries.
    android_aar_plugin: Option<bool>,
}

impl Configuration {
    /// Creates a new instance of [`Configuration`], by using parameters with sensible types instead of the types [`Configuration`] will store.
    ///
    /// # Parameters
    ///
    /// * `entry_symbol` - [`EntrySymbol`] for initializing the [`GDExtension`]. It uses its `to_string` method to provide its representation.
    /// * `compatibility_minimum` - Minimum compatible version of `Godot`, with format `(major, minor)`, in case [`Some`] is provided.
    /// * `compatibility_maximum` - Maximum compatible version of `Godot`, with format `(major, minor)`, in case [`Some`] is provided.
    /// * `is_reloadable` - Whether or not to allow the reloading of the [`GDExtension`] upon recompilation.
    /// * `are_exported_by_android_aar_plugin` - Whether or not the [`GDExtension`] native shared libraries are exported by the `Android` plugin `AAR` binaries.
    ///
    /// # Returns
    ///
    /// The [`Configuration`] with the necessary fields properly parsed.
    pub fn new(
        entry_symbol: EntrySymbol,
        compatibility_minimum: Option<(u8, u8)>,
        compatibility_maximum: Option<(u8, u8)>,
        is_reloadable: bool,
        are_exported_by_android_aar_plugin: bool,
    ) -> Self {
        Self {
            entry_symbol: entry_symbol.to_string(),
            compatibility_minimum: compatibility_minimum
                .map(|(major, minor)| format!("{}.{}", major, minor).parse().unwrap_or(4.1)),
            compatibility_maximum: compatibility_maximum.and_then(|(major, minor)| match format!(
                "{}.{}",
                major, minor
            )
            .parse()
            {
                Ok(com_min) => Some(com_min),
                _ => None,
            }),
            reloadable: is_reloadable.then_some(true),
            android_aar_plugin: are_exported_by_android_aar_plugin.then_some(true),
        }
    }

    /// Creates a new instance of [`Configuration`], by using the parameters as are.
    ///
    /// # Parameters
    ///
    /// * `entry_symbol` - Name of the entry function for initializing the [`GDExtension`].
    /// * `compatibility_minimum` - Minimum compatible version of `Godot`, with format `major.minor`, in case [`Some`] is provided.
    /// * `compatibility_maximum` - Maximum compatible version of `Godot`, with format `major.minor`, in case [`Some`] is provided.
    /// * `reloadable` - Whether or not to allow the reloading of the [`GDExtension`] upon recompilation, in case [`Some`] is provided.
    /// * `android_aar_plugin` - Whether or not the [`GDExtension`] native shared libraries are exported by the `Android` plugin `AAR` binaries in case [`Some`] is provided.
    ///
    /// # Returns
    ///
    /// The [`Configuration`] with the necessary fields properly parsed.
    pub fn raw_new(
        entry_symbol: String,
        compatibility_minimum: Option<f64>,
        compatibility_maximum: Option<f64>,
        reloadable: Option<bool>,
        android_aar_plugin: Option<bool>,
    ) -> Self {
        Self {
            entry_symbol,
            compatibility_minimum,
            compatibility_maximum,
            reloadable,
            android_aar_plugin,
        }
    }
}

impl Default for Configuration {
    /// The [`Configuration`] found in the `godot-rust` book.
    fn default() -> Self {
        Configuration::new(
            EntrySymbol::GodotRustDefault,
            Some((4, 1)),
            None,
            true,
            false,
        )
    }
}
