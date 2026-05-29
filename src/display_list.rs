//! `DisplayList`: the ordered sequence of paint commands.

use crate::command::PaintCommand;

/// A complete display list.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DisplayList {
    commands: Vec<PaintCommand>,
}

impl DisplayList {
    /// Build a display list from a command vector.
    #[must_use]
    pub fn new(commands: Vec<PaintCommand>) -> Self {
        Self { commands }
    }

    /// An empty display list.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// The commands in paint order (back-to-front).
    #[must_use]
    pub fn commands(&self) -> &[PaintCommand] {
        &self.commands
    }

    /// Total command count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Whether the list is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}
