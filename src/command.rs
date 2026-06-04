//! `PaintCommand`: one renderable atom.

use layout_cat::{Color, Point, Rect};

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

impl PaintCommand {
    /// Return a new [`PaintCommand`] with all geometric quantities
    /// multiplied by `factor`.  For [`Self::FillRect`] the rect's
    /// origin / width / height all scale; for [`Self::StrokeRect`]
    /// the rect and the stroke `width`; for [`Self::FillText`] the
    /// rect and the `font_size`.  Colours and text content are
    /// preserved unchanged.
    ///
    /// Useful for zoom-style backends that want the rasterizer to
    /// re-draw at the larger size (crisp text via re-rasterization
    /// at `zoom * font_size`) rather than bitmap-scaling the output.
    ///
    /// # Examples
    ///
    /// ```
    /// use layout_cat::{Color, Point, Rect};
    /// use paint_cat::PaintCommand;
    ///
    /// let original = PaintCommand::FillText {
    ///     rect: Rect::new(Point::new(0.0, 0.0), 100.0, 30.0),
    ///     text: "hi".to_owned(),
    ///     color: Color::rgba(0.0, 0.0, 0.0, 1.0),
    ///     font_size: 16.0,
    /// };
    /// let scaled = original.scaled(2.0);
    /// match scaled {
    ///     PaintCommand::FillText { font_size, rect, .. } => {
    ///         let font_size_doubled = (font_size - 32.0).abs() < f64::EPSILON;
    ///         let width_doubled = (rect.width() - 200.0).abs() < f64::EPSILON;
    ///         let _ = font_size_doubled && width_doubled;
    ///     }
    ///     PaintCommand::FillRect { .. } | PaintCommand::StrokeRect { .. } => {}
    /// }
    /// ```
    #[must_use]
    pub fn scaled(&self, factor: f64) -> Self {
        match self {
            Self::FillRect { rect, color } => Self::FillRect {
                rect: scale_rect(rect, factor),
                color: *color,
            },
            Self::StrokeRect { rect, color, width } => Self::StrokeRect {
                rect: scale_rect(rect, factor),
                color: *color,
                width: width * factor,
            },
            Self::FillText {
                rect,
                text,
                color,
                font_size,
            } => Self::FillText {
                rect: scale_rect(rect, factor),
                text: text.clone(),
                color: *color,
                font_size: font_size * factor,
            },
        }
    }
}

fn scale_rect(rect: &Rect, factor: f64) -> Rect {
    Rect::new(
        Point::new(rect.origin().x() * factor, rect.origin().y() * factor),
        rect.width() * factor,
        rect.height() * factor,
    )
}
