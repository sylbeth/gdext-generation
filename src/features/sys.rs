//! Module for the [`System`] a `Godot` game using `Rust GDExtension` can be compiled for.

use super::arch::Architecture;

/// System to compile the `Godot` game and the `Rust GDExtension` for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum System {
    /// Android system.
    Android,
    /// iOS system.
    IOS,
    /// Linux system.
    Linux,
    /// MacOS system.
    MacOS,
    /// Web browser.
    Web,
    /// Windows system.
    Windows(WindowsABI),
}

impl System {
    /// Gets all [`System`]s available.
    ///
    /// # Parameters
    ///
    /// * `windows_abi` - Env and ABI used to build for `Windows`.
    ///
    /// # Returns
    ///
    /// An array with all available [`System`]s.
    pub fn get_systems(windows_abi: WindowsABI) -> [Self; 6] {
        [
            Self::Android,
            Self::IOS,
            Self::Linux,
            Self::MacOS,
            Self::Web,
            Self::Windows(windows_abi),
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
                Architecture::Generic,
                Architecture::Armv7,
                Architecture::Arm64,
                Architecture::X86_32,
                Architecture::X86_64,
            ],
            Self::IOS => vec![Architecture::Generic, Architecture::Arm64],
            Self::Linux => vec![
                Architecture::Generic,
                Architecture::Arm64,
                Architecture::Rv64,
                Architecture::X86_64,
            ],
            Self::MacOS => vec![
                Architecture::Generic,
                Architecture::Arm64,
                Architecture::X86_64,
            ],
            Self::Web => vec![Architecture::Generic, Architecture::Wasm32],
            Self::Windows(_) => vec![
                Architecture::Generic,
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
            Self::MacOS => "macos",
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
                // The `godot-rust` book has android libraries without the lib in front, but it may be an error.
                Self::IOS | Self::Linux | Self::MacOS => "lib",
                Self::Android | Self::Windows(_) | Self::Web => "",
            },
            lib_name,
            match self {
                Self::Android | Self::Linux => "so",
                Self::IOS => "ios.framework",
                Self::MacOS => "dylib",
                Self::Web => "wasm",
                Self::Windows(_) => "dll",
            }
        )
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
