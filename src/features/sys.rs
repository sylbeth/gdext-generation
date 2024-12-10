//! Module for the [`System`] a `Godot` game using `Rust GDExtension` can be compiled for.

use super::arch::Architecture;
use crate::args::WindowsCompiler;

/// System to compile the `Godot` game and the `Rust GDExtension` for.
#[derive(Debug, Clone, Copy)]
pub enum System {
    /// Android system.
    Android,
    /// iOS system.
    IOS,
    /// Linux system.
    Linux,
    /// MacOS system.
    Macos,
    /// Web browser.
    Web,
    /// Windows system.
    Windows(WindowsCompiler),
}

impl System {
    /// Gets all [`System`]s available.
    ///
    /// # Parameters
    ///
    /// * `windows_compiler` - Compiler used to build for `Windows`.
    ///
    /// # Returns
    ///
    /// An array with all available [`System`]s.
    pub fn get_systems(windows_compiler: WindowsCompiler) -> [Self; 6] {
        [
            Self::Android,
            Self::IOS,
            Self::Linux,
            Self::Macos,
            Self::Web,
            Self::Windows(windows_compiler),
        ]
    }

    /// Gets all [`Architecture`]s available for a [`System`].
    ///
    /// # Returns
    ///
    /// A [`Vec`] with all available [`Architecture`] for the [`System`].
    pub fn get_architectures(&self) -> Vec<Architecture> {
        match self {
            Self::Android => vec![
                Architecture::Armv7,
                Architecture::Arm64,
                Architecture::X86_32,
                Architecture::X86_64,
            ],
            Self::IOS => vec![Architecture::Arm64],
            Self::Linux => vec![
                Architecture::Arm64,
                Architecture::Rv64,
                Architecture::X86_64,
            ],
            Self::Macos => vec![Architecture::Arm64, Architecture::X86_64],
            Self::Web => vec![Architecture::Wasm32],
            Self::Windows(_) => vec![
                Architecture::Arm64,
                Architecture::X86_32,
                Architecture::X86_64,
            ],
        }
    }

    /// Gets the name of the [`System`] in lowercase.
    ///
    /// # Returns
    ///
    /// The name of the [`System`] in lowercase.
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::IOS => "ios",
            Self::Linux => "linux",
            Self::Macos => "macos",
            Self::Web => "web",
            Self::Windows(_) => "windows",
        }
    }

    /// Gets the name of the compiled library for the given system.
    ///
    /// # Parameters
    ///
    /// * `lib_name` - Name of the library crate that is being compiled. It can be retrieved with the environmental variable: "`CARGO_PKG_NAME"`, but it must be turned into snake_case.
    ///
    /// # Returns
    ///
    /// The name of the file that's going to be compiled.
    pub fn get_lib_export_name(&self, lib_name: &str) -> String {
        format!(
            "{}{}.{}",
            match self {
                Self::IOS | Self::Linux | Self::Macos => "lib",
                Self::Android | Self::Windows(_) | Self::Web => "",
            },
            lib_name,
            match self {
                Self::Android | Self::Linux => "so",
                Self::IOS => "ios.framework",
                Self::Macos => "dylib",
                Self::Web => "wasm",
                Self::Windows(_) => "dll",
            }
        )
    }
}
