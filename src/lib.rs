//! Display-list construction for the paint stage.
//!
//! # Example
//!
//! ```
//! # fn main() -> Result<(), paint_cat::Error> {
//! use paint_cat::build;
//!
//! let html_doc = html_cat::parse("<html><body><p>hi</p></body></html>")
//!     .map_err(|_| paint_cat::Error::Dom(dom_cat::Error::InvalidSelector { selector: String::new() }))?;
//! let dom = dom_cat::Document::from_html_doc(&html_doc);
//! let sheet = css_cat::parse("p { background-color: red; padding: 8px; }")
//!     .map_err(|_| paint_cat::Error::Dom(dom_cat::Error::InvalidSelector { selector: String::new() }))?;
//! let tree = layout_cat::layout(&dom, &sheet, layout_cat::Viewport::new(800, 600));
//! let display_list = build(&tree, &dom);
//! assert!(!display_list.commands().is_empty());
//! # Ok(())
//! # }
//! ```

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(clippy::similar_names)]
#![allow(clippy::float_cmp)]

pub mod build;
pub mod command;
pub mod display_list;
pub mod error;

pub use build::build;
pub use command::PaintCommand;
pub use display_list::DisplayList;
pub use error::Error;
