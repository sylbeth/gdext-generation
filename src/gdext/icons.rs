//! Module for the generation of the icons section of the `.gdextension` file.

use std::{
    fs::File,
    io::{Result, Write},
};

use toml::Table;

use super::GDExtension;
use crate::{args::icons::IconsConfig, NODES_RUST, NODES_RUST_FILENAMES};

#[cfg(feature = "find_icons")]
use crate::args::DefaultNodeIcon;
#[cfg(feature = "find_icons")]
use glob::glob;
#[cfg(feature = "find_icons")]
use regex::{Match, Regex};
#[cfg(feature = "find_icons")]
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

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

        #[cfg(feature = "find_icons")]
        if icons_config.default != DefaultNodeIcon::Node {
            let mut base_class_to_nodes = HashMap::<String, Vec<String>>::new();

            find_children(&mut base_class_to_nodes)?;

            for (icon, nodes) in base_class_to_nodes {
                for node in nodes {
                    icons.insert(
                        node,
                        match icons_config.default {
                            DefaultNodeIcon::BaseClass => format!(
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
                            DefaultNodeIcon::Custom(ref custom_path) => format!(
                                "{}{}",
                                &icons_config
                                    .directories
                                    .relative_directory
                                    .unwrap_or_default()
                                    .as_str(),
                                (&icons_config.directories.base_directory)
                                    .join(&custom_path)
                                    .to_string_lossy()
                                    .replace('\\', "/")
                            )
                            .into(),
                            DefaultNodeIcon::NodeRust(node_rust, ref rust_path) => format!(
                                "{}{}/{}",
                                &icons_config
                                    .directories
                                    .relative_directory
                                    .unwrap_or_default()
                                    .as_str(),
                                (&icons_config.directories.base_directory)
                                    .join(&rust_path)
                                    .to_string_lossy()
                                    .replace('\\', "/"),
                                NODES_RUST_FILENAMES[node_rust as usize],
                            )
                            .into(),
                            DefaultNodeIcon::Node => "ERROR".into(),
                        },
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

        #[allow(unused_mut)]
        let mut copy_files = icons_config.copy_strategy.copy_all;
        #[cfg(feature = "find_icons")]
        {
            copy_files |= icons_config.copy_strategy.copy_node_rust;
        }

        if copy_files {
            let base_directory_path = icons_config.copy_strategy.path_node_rust;
            let mut nodes_rust = Vec::new();

            if icons_config.copy_strategy.copy_all {
                nodes_rust.extend(NODES_RUST_FILENAMES.into_iter().zip(NODES_RUST));
            } else {
                #[cfg(feature = "find_icons")]
                if icons_config.copy_strategy.copy_node_rust {
                    if let DefaultNodeIcon::NodeRust(node_rust, _) = icons_config.default {
                        nodes_rust.push((
                            NODES_RUST_FILENAMES[node_rust as usize],
                            NODES_RUST[node_rust as usize],
                        ));
                    }
                }
            }

            for (file_name, node_rust) in nodes_rust {
                let path_node_rust = (&base_directory_path).join(file_name);
                if icons_config.copy_strategy.force_copy | !path_node_rust.exists() {
                    File::create(path_node_rust)?.write_all(node_rust.as_bytes())?;
                }
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
#[cfg(feature = "find_icons")]
fn find_children(base_class_to_nodes: &mut HashMap<String, Vec<String>>) -> Result<()> {
    // Only works if base = BaseClass contains no comments in between.
    let base_class_regex =
        Regex::new(r"base\s*\=\s*[\w_\d]+\s*[),]").expect("Invalid regex pattern.");
    // Only works if struct StructName contains no comments in between.
    let struct_regex = Regex::new(r"struct\s*[\w_\d]+\s*[{;<]").expect("Invalid regex pattern.");

    let mut base_class = String::new();
    let mut struct_class;
    let mut found_base;

    for path_glob in glob("./src/**/*.rs").unwrap() {
        let path;
        match path_glob {
            Ok(pathbuf) => path = pathbuf,
            Err(_) => continue,
        }
        found_base = false;
        for line in BufReader::new(File::open(path)?).lines() {
            let line: String = line?;
            if !line.starts_with("///") & line.contains("base") & line.contains("=") {
                base_class = if let Some(base_class_match) = base_class_regex.find(&line) {
                    Match::as_str(&base_class_match)
                        .replace("base", "")
                        .replace('=', "")
                } else {
                    continue;
                };
                // Eliminate the , or ).
                base_class.pop();
                let base_class_trimmed = base_class.trim();
                if !base_class_to_nodes.contains_key(base_class_trimmed) {
                    base_class_to_nodes.insert(base_class_trimmed.to_owned(), Vec::new());
                }
                found_base = true;
            } else if found_base & !line.starts_with("///") & line.contains("struct") {
                struct_class = if let Some(struct_class_match) = struct_regex.find(&line) {
                    Match::as_str(&struct_class_match).replace("struct", "")
                } else {
                    continue;
                };
                // Eliminate the ;, { or <.
                struct_class.pop();
                let struct_class_trimmed = struct_class.trim();
                base_class_to_nodes
                    .get_mut(&base_class)
                    .expect("The map doesn't contain the key that was just pushed to it.")
                    .push(struct_class_trimmed.into());
                found_base = false;
            }
        }
    }

    Ok(())
}
