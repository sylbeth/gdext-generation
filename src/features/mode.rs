//! Module for the [`Mode`] a `Godot` game using `Rust GDExtension` can be compiled in.

/// Mode to compile the `Godot` game and the `Rust GDExtension` in.
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    /// Debug mode.
    Debug,
    /// Release mode.
    Release,
    /// Editor mode (for Godot, since it's the same as debug for Rust).
    Editor,
}

impl Mode {
    /// Gets all build [`Mode`]s available.
    ///
    /// # Returns
    ///
    /// An array with all available [`Mode`]s.
    pub fn get_modes() -> [Self; 3] {
        [Self::Debug, Self::Release, Self::Editor]
    }

    /// Gets the name of the build [`Mode`] used in `Rust` target folders.
    ///
    /// # Returns
    ///
    /// The name of the build [`Mode`] as is written in the `Rust` target folder.
    pub fn get_rust_name(&self) -> &'static str {
        match self {
            Self::Debug | Self::Editor => "debug",
            Self::Release => "release",
        }
    }

    /// Gets the name of the build [`Mode`] used in `Godot` targets.
    ///
    /// # Returns
    ///
    /// The name of the build [`Mode`] as is written in the `Godot` target folder.
    pub fn get_godot_name(&self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Release => "release",
            Self::Editor => "editor",
        }
    }
}
