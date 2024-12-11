//! Module for the [`Architecture`] a `Godot` game using `Rust GDExtension` can be released for and their representations as `Godot` and `Rust` targets.

/// Architecture to compile the `Godot` game and the `Rust GDExtension` for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Architecture {
    /// The i868 architecture.
    X86_32,
    /// The x86_64 architecture.
    X86_64,
    /// The Arm32v7 architecture.
    Armv7,
    /// The AArch64 architecture.
    Arm64,
    /// The Risc-V 64 architecture.
    Rv64,
    /// The WebAssembly architecture.
    Wasm32,
    /// MacOS universal library using [`Architecture::Arm64`] and [`Architecture::X86_64`], or a generic architecture for the rest.
    Generic,
}

impl Architecture {
    /// Gets the name of the [`Architecture`] used in `Rust` target triples.
    ///
    /// # Returns
    ///
    /// The name of the [`Architecture`] for the `Rust` target triple.
    pub fn get_rust_name(&self) -> &'static str {
        match self {
            Self::X86_32 => "i686",
            Self::X86_64 => "x86_64",
            Self::Armv7 => "armv7",
            Self::Arm64 => "aarch64",
            Self::Rv64 => "riscv64gc",
            Self::Wasm32 => "wasm32",
            Self::Generic => "",
        }
    }

    /// Gets the name of the [`Architecture`] used in `Godot` targets.
    ///
    /// # Returns
    ///
    /// The name of the [`Architecture`] for the `Godot` target.
    pub fn get_godot_name(&self) -> &'static str {
        match self {
            Self::X86_32 => "x86_32",
            Self::X86_64 => "x86_64",
            Self::Armv7 => "arm_32",
            Self::Arm64 => "arm_64",
            Self::Rv64 => "rv_64",
            Self::Wasm32 => "wasm32",
            Self::Generic => "",
        }
    }
}
