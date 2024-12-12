//! Module for the generation of the icons section of the `.gdextension` file.

use std::{
    fs::File,
    io::{Result, Write},
};

use toml::Table;

use super::GDExtension;
use crate::{args::IconsConfig, NODE_RUST, NODE_RUST_FILENAME};

#[cfg(any(feature = "find_icons", feature = "simple_find_icons"))]
use std::{
    collections::HashMap, io::{BufRead, BufReader}
};
#[cfg(any(feature = "find_icons", feature = "simple_find_icons"))]
use glob::glob;
#[cfg(any(feature = "find_icons", feature = "simple_find_icons"))]
use crate::args::DefaultNodeIcon;

#[cfg(feature = "find_icons")]
mod parser;

/*
const base_checkers: [&str; 2] = ["base", "="];
const struct_checker: &str = "struct";

enum FirstCheck {
    Base,
    Equal,
    Struct,
    None,
}

enum CurrentTraversal {
    InComment,
    FindBase,
    FindEqual,
    FindIcon,
    FindStruct,
    FindName,
}
*/

impl GDExtension {
    /// Generates the icons section of the [`GDExtension`].
    ///
    /// # Parameters
    ///
    /// * `icon_config` - Configuration struct for the generation of icons. If `relative_directory` of the [`IconsDirectories`](crate::args::IconsDirectories) is [`None`] it will use the default value.
    ///
    /// # Returns
    ///
    ///
    /// * [`Ok`] (&mut [`GDExtension`]) - If there has been no problem infering the nodes and their corresponding icons nor copying them, the same [`GDExtension`] mutable reference it was passed to it.
    /// * [`Err`] ([`Error`](std::io::Error)) - If there was a problem reading the `src` files, or copying the icons to their corresponding folder.
    pub fn generate_icons(&mut self, icons_config: IconsConfig) -> Result<&mut Self> {
        let mut icons = Table::new();

        #[cfg(any(feature = "find_icons", feature = "simple_find_icons"))]
        if icons_config.default != DefaultNodeIcon::Node {
            let mut base_class_to_nodes = HashMap::<String, Vec<String>>::new();

            find_children(&mut base_class_to_nodes)?;

            for (icon, nodes) in base_class_to_nodes {
                for node in nodes {
                    icons.insert(
                        node,
                        format!(
                            "{}{}.svg",
                            &icons_config
                                .directories
                                .relative_directory
                                .unwrap_or_default()
                                .as_str(),
                            (&icons_config.directories.base_directory)
                                .join(&icons_config.directories.editor_directory)
                                .join(&icon)
                                .to_string_lossy()
                                .replace('\\', "/")
                        )
                        .into(),
                    );
                }
            }
        }

        if let Some(custom_icons) = &icons_config.custom_icons {
            for (node, icon) in custom_icons {
                icons.insert(
                    node.clone(),
                    format!(
                        "{}{}",
                        &icons_config
                            .directories
                            .relative_directory
                            .unwrap_or_default()
                            .as_str(),
                        (&icons_config.directories.base_directory)
                            .join(&icons_config.directories.custom_directory)
                            .join(icon)
                            .to_string_lossy()
                            .replace('\\', "/")
                    )
                    .into(),
                );
            }
        }
        let path_node_rust = icons_config
            .copy_strategy
            .path_node_rust
            .join(NODE_RUST_FILENAME);
        if icons_config.copy_strategy.copy_node_rust
            & (icons_config.copy_strategy.force_copy | !path_node_rust.exists())
        {
            File::create(path_node_rust)?.write_all(NODE_RUST.as_bytes())?;
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
#[cfg(any(feature = "find_icons", feature = "simple_find_icons"))]
fn find_children(base_class_to_nodes: &mut HashMap<String, Vec<String>>) -> Result<()> {
    Ok(())
}
