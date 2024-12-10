//! Module for the representation of the [`Target`], either `Godot`'s or `Rust`'s.

use super::{arch::Architecture, mode::Mode, sys::System};

/// Target to compile the `Godot` game and the `Rust GDExtension` to.
pub struct Target(pub System, pub Mode, pub Architecture);

impl Target {
    /// Gets the name of the `Rust` target triple this [`Target`] would use.
    ///
    /// # Returns
    ///
    /// The name of the `Rust` target triple of this [`Target`].
    pub fn get_rust_target_triple(&self) -> String {
        if self.2 == Architecture::Generic {
            return "".into();
        }
        match self.0 {
            System::Android => format!(
                "{}-linux-{}{}",
                self.2.get_rust_name(),
                self.0.get_name(),
                if self.2 == Architecture::Armv7 {
                    "eabi"
                } else {
                    ""
                }
            ),
            System::IOS => format!("{}-apple-{}", self.2.get_rust_name(), self.0.get_name()),
            System::Linux => format!(
                "{}-unknown-{}-gnu",
                self.2.get_rust_name(),
                self.0.get_name()
            ),
            System::MacOS => format!("{}-apple-darwin", self.2.get_rust_name()),
            System::Web => format!("{}-unknown-emscripten", self.2.get_rust_name()),
            System::Windows(windows_abi) => format!(
                "{}-pc-{}-{}",
                self.2.get_rust_name(),
                self.0.get_name(),
                windows_abi.get_rust_name(),
            ),
        }
    }

    /// Gets the name of the `Godot` target this [`Target`] would use.
    ///
    /// # Returns
    ///
    /// The name of the `Godot` target of this [`Target`].
    pub fn get_godot_target(&self) -> String {
        if self.2 == Architecture::Generic {
            format!("{}.{}", self.0.get_name(), self.1.get_godot_name())
        } else {
            format!(
                "{}.{}.{}",
                self.0.get_name(),
                self.1.get_godot_name(),
                self.2.get_godot_name()
            )
        }
    }
}
