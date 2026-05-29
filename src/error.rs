//! Paint error type.

use dom_cat::Error as DomError;
use layout_cat::Error as LayoutError;

/// All errors `paint-cat` can produce.  v0 has no native error
/// conditions; the variants exist so future strict-mode callers can
/// surface upstream parser/layout failures here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A layout-stage error.
    Layout(LayoutError),
    /// A DOM error.
    Dom(DomError),
}

impl From<LayoutError> for Error {
    fn from(value: LayoutError) -> Self {
        Self::Layout(value)
    }
}

impl From<DomError> for Error {
    fn from(value: DomError) -> Self {
        Self::Dom(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Layout(e) => write!(f, "layout error: {e}"),
            Self::Dom(e) => write!(f, "dom error: {e}"),
        }
    }
}

impl std::error::Error for Error {}
