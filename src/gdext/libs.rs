//! Module for the generation of the libraries section of the `.gdextension` file.

#[allow(unused_imports)]
use std::path::{Path, PathBuf};

use super::GDExtension;
use crate::{
    args::WindowsCompiler,
    features::{arch::Architecture, mode::Mode, sys::System, target::Target},
};

impl GDExtension {
    /// Generates the libraries section of the [`GDExtension`].
    ///
    /// # Parameters
    ///
    /// * `lib_name` - Name of the library crate that is being compiled. It can be retrieved with the environmental variable: "`CARGO_PKG_NAME"`, but it must be turned into snake_case.
    /// * `windows_compiler` - Compiler used to build for `Windows`.
    /// * `target_dir` - Path to the build folder (specified inside the variable `[build] target-dir` of `.cargo/config.toml`) relative to the project file. For example, if the path for `Godot` would be `"res://path/to/build"`, the path provided must be `"path/to/build"`. If the path contains non valid Unicode, it will be stored calling [`to_string_lossy`](Path::to_string_lossy).
    ///
    /// # Returns
    ///
    /// The same [`GDExtension`] mutable reference it was passed to it.
    pub fn generate_libs(
        &mut self,
        lib_name: &str,
        windows_compiler: WindowsCompiler,
        target_dir: PathBuf,
    ) -> &mut Self {
        for system in System::get_systems(windows_compiler) {
            for architecture in system.get_architectures() {
                for mode in Mode::get_modes() {
                    let target = Target(system, mode, architecture);
                    self.libraries.insert(
                        target.get_godot_target(),
                        if target.2 == Architecture::Generic {
                            format!(
                                "res://{}//{}/{}",
                                target_dir.to_string_lossy(),
                                target.1.get_rust_name(),
                                target.0.get_lib_export_name(lib_name),
                            )
                            .into()
                        } else {
                            format!(
                                "res://{}/{}/{}/{}",
                                target_dir.to_string_lossy(),
                                target.get_rust_target_triple(),
                                target.1.get_rust_name(),
                                target.0.get_lib_export_name(lib_name),
                            )
                            .into()
                        },
                    );
                }
            }
        }

        self
    }
}
