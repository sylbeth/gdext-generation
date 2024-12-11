//! Module for the generation of the dependencies section of the `.gdextension` file.

#[allow(unused_imports)]
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use toml::Table;

use super::GDExtension;
use crate::{features::{sys::System, target::Target}, PROJECT_FOLDER};

impl GDExtension {
    /// Generates the dependencies section of the [`GDExtension`].
    ///
    /// # Parameters
    ///
    /// * `dependencies` - Map of dependencies, where the key is the target and the value is a [`Vec`] with the paths to the dependencies **relative** to the project folder. For example, if the path for `Godot` would be `"res://path/to/dep"`, the path provided must be `"path/to/dep"`. If the path contains non valid Unicode, it will be stored calling [`to_string_lossy`](Path::to_string_lossy).
    ///
    /// # Returns
    ///
    /// The same [`GDExtension`] mutable reference it was passed to it.
    pub fn generate_deps(&mut self, dependencies: HashMap<Target, Vec<PathBuf>>) -> &mut Self {
        let mut dependencies_table = Table::new();

        for (target, paths) in dependencies {
            let target_name = target.get_godot_target();
            let mut current_dependencies = Table::new();
            for path in paths {
                current_dependencies.insert(
                    format!("{PROJECT_FOLDER}{}", path.to_string_lossy()),
                    match target.0 {
                        System::MacOS => "Contents/Frameworks",
                        _ => "",
                    }
                    .into(),
                );
            }
            dependencies_table.insert(target_name, current_dependencies.into());
        }

        self.dependencies = Some(dependencies_table);

        self
    }
}
