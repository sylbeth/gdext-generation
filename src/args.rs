//! Module with the structs and enums needed to call the main function of the library.

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
