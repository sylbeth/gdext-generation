//! Module for the generation of the dependencies section of the `.gdextension` file.

#[allow(unused_imports)]
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use toml_edit::{Decor, InlineTable, Key};

use super::GDExtension;
use crate::{
    features::{sys::System, target::Target},
    PROJECT_FOLDER,
};

impl GDExtension {
    /// Generates the dependencies section of the [`GDExtension`].
    ///
    /// # Parameters
    ///
    /// * `dependencies` - Map of dependencies, where the key is the target and the value is a [`Vec`] with the paths to the dependencies **relative** to the project folder. For example, if the path for `Godot` would be `"res://path/to/dep"`, the path provided must be `"path/to/dep"`. If the path contains non valid Unicode, it will be stored calling [`to_string_lossy`](Path::to_string_lossy).
    ///
    /// # Returns
    ///
    /// The [`Vec`] of targets and their dependencies to add well formatted to the [`toml_edit::DocumentMut`].
    pub fn generate_deps(
        dependencies: HashMap<Target, Vec<PathBuf>>,
    ) -> Vec<(String, InlineTable)> {
        let mut dependencies_vector = Vec::new();
        // Decor for the formatting of the inline keys.
        let leaf_decor = Decor::new("\n    ", " ");

        for (target, paths) in dependencies {
            let target_name = target.get_godot_target();
            let mut current_dependencies = InlineTable::new();
            for path in paths {
                current_dependencies.insert_formatted(
                    &Key::from(format!(
                        "{PROJECT_FOLDER}{}",
                        path.to_string_lossy().replace('\\', "/")
                    ))
                    .with_leaf_decor(leaf_decor.clone()),
                    match target.0 {
                        System::MacOS => "Contents/Frameworks",
                        _ => "",
                    }
                    .into(),
                );
            }

            // There should at least be one target-dependencies, and thus, a newline can be safely added.
            current_dependencies
                .iter_mut()
                .last()
                .unwrap()
                .1
                .decor_mut()
                .set_suffix("\n");

            dependencies_vector.push((target_name, current_dependencies));
        }

        // Generating the empty table where the dependencies will be formatted in. This is not needed anymore, it's generated using toml_edit instead.
        //self.dependencies = Some(toml::Table::new());

        dependencies_vector
    }
}
