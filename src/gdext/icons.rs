//! Module for the generation of the icons section of the `.gdextension` file.

use super::GDExtension;

impl GDExtension {
    /// Generates the icons section of the [`GDExtension`].
    ///
    /// # Returns
    ///
    /// The same [`GDExtension`] mutable reference it was passed to it.
    pub fn generate_icons(&mut self) -> &mut Self {
        self
    }
}
