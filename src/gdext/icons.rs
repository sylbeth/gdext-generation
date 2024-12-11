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

        self.icons = Some(icons);

        Ok(self)
    }
}
    }
}
