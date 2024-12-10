//! Module with the structs and enums needed to call the main function of the library.

/// Compiler used to build the `Rust GDExtension` for `Windows`.
#[derive(Debug, Clone, Copy)]
pub enum WindowsCompiler {
    /// Microsoft Visual C++ compiler.
    MSVC,
    /// The MinGW compiler (MSYS2 port of GCC).
    MinGW,
    /// The Clang compiler (LLVM drop-in for GCC).
    Clang,
}

impl WindowsCompiler {
    /// Gets the name of the [`WindowsCompiler`] used in `Rust` target triples.
    ///
    /// # Returns
    ///
    /// The name of the [`WindowsCompiler`] for the `Rust` target triple.
    pub fn get_rust_name(&self) -> &'static str {
        match self {
            Self::MSVC => "msvc",
            Self::MinGW => "gnu",
            Self::Clang => "gnullvm",
        }
    }
}
