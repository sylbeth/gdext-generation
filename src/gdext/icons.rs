//! Module for the generation of the icons section of the `.gdextension` file.

use super::GDExtension;
use crate::{args::IconsConfig, PROJECT_FOLDER};

impl GDExtension {
    /// Generates the icons section of the [`GDExtension`].
    ///
    /// # Parameters
    ///
    /// * `icon_config` - Configuration struct for the generation of icons.
    ///
    /// # Returns
    ///
    ///
    /// * [`Ok`] (&mut [`GDExtension`]) - If there has been no problem infering the nodes and their corresponding icons nor copying them, the same [`GDExtension`] mutable reference it was passed to it.
    /// * [`Err`] ([`Error`](std::io::Error)) - If there was a problem reading the `src` files, or copying the icons to their corresponding folder.
    pub fn generate_icons(&mut self, icons_config: IconsConfig) -> Result<&mut Self> {
        let mut icons = Table::new();

        let mut base_class_to_nodes = HashMap::<String, Vec<String>>::new();

        find_children(&mut base_class_to_nodes)?;

        for (icon, nodes) in base_class_to_nodes {
            for node in nodes {
                icons.insert(
                    node,
                    format!(
                        "{PROJECT_FOLDER}{}.svg",
                        (&icons_config.directories.base_directory)
                            .join(&icons_config.directories.editor_directory)
                            .join(&icon)
                            .to_string_lossy()
                    )
                    .into());
            }
        }

        if let Some(custom_icons) = &icons_config.custom_icons {
            for (node, icon) in custom_icons {
                icons.insert(
                    node.clone(),
                    format!(
                        "{PROJECT_FOLDER}{}",
                        (&icons_config.directories.base_directory)
                            .join(&icons_config.directories.custom_directory)
                            .join(icon)
                            .to_string_lossy()
                    )
                    .into(),
                );
            }
        }

        self.icons = Some(icons);

        Ok(self)
    }
}

/// Finds the structs that have inherited each base class, updating the base_class_to_nodes HashMap.
///
/// # Parameters
///
/// `base_class_to_nodes` - [`HashMap`] to fill with relationships `base_class: [struct1, ..., structn]`, of the structs that have inherited the base_class.
///
/// # Returns
///
/// * [`Ok`] - If the `base_class_to_nodes` [`HashMap`] could be filled.
/// * [`Err`] - Otherwise.
fn find_children(base_class_to_nodes: &mut HashMap<String, Vec<String>>) -> Result<()> {
    }
    Ok(())
}
