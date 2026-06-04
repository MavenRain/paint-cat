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

    /// Return a new [`DisplayList`] with every [`PaintCommand`]
    /// scaled by `factor` via [`PaintCommand::scaled`].  See that
    /// method for the per-variant semantics; the order and count of
    /// commands is preserved.
    ///
    /// # Examples
    ///
    /// ```
    /// use layout_cat::{Color, Point, Rect};
    /// use paint_cat::{DisplayList, PaintCommand};
    ///
    /// let list = DisplayList::new(vec![PaintCommand::FillRect {
    ///     rect: Rect::new(Point::new(0.0, 0.0), 10.0, 10.0),
    ///     color: Color::rgba(1.0, 0.0, 0.0, 1.0),
    /// }]);
    /// let zoomed = list.scaled(2.0);
    /// let _ = zoomed.len();
    /// ```
    #[must_use]
    pub fn scaled(&self, factor: f64) -> Self {
        Self {
            commands: self.commands.iter().map(|cmd| cmd.scaled(factor)).collect(),
        }
    }
}
