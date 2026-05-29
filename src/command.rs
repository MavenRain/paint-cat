//! `PaintCommand`: one renderable atom.

use layout_cat::{Color, Rect};

/// One paint command.  Backend renderers translate these into actual
/// draw calls.
#[derive(Debug, Clone, PartialEq)]
pub enum PaintCommand {
    /// Fill a rectangle with `color`.
    FillRect {
        /// The rectangle to fill.
        rect: Rect,
        /// The fill color.
        color: Color,
    },
    /// Draw a one-pixel-wide stroke around `rect` in `color`.  Backends
    /// that prefer line primitives can decompose this; v0 leaves it as
    /// a single command for clarity.
    StrokeRect {
        /// The rectangle to stroke.
        rect: Rect,
        /// The stroke color.
        color: Color,
        /// The stroke width in CSS pixels.
        width: f64,
    },
    /// Render `text` inside `rect` using `color`.  Font selection /
    /// shaping is deferred to the backend.
    FillText {
        /// The bounding rectangle (typically the parent box's content rect).
        rect: Rect,
        /// The text content.
        text: String,
        /// The text color.
        color: Color,
        /// The font size in CSS pixels.
        font_size: f64,
    },
}

impl Eq for PaintCommand {}
