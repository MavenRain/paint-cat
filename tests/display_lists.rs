//! Integration tests for display-list construction.

#![allow(clippy::float_cmp)]

use paint_cat::{DisplayList, Error, PaintCommand, build};

fn fail(_msg: &'static str) -> Error {
    Error::Dom(dom_cat::Error::InvalidSelector {
        selector: String::new(),
    })
}

fn run(html: &str, css: &str) -> Result<DisplayList, Error> {
    let html_doc = html_cat::parse(html).map_err(|_| fail("html parse"))?;
    let dom = dom_cat::Document::from_html_doc(&html_doc);
    let sheet = css_cat::parse(css).map_err(Error::from)?;
    let tree = layout_cat::layout(&dom, &sheet, layout_cat::Viewport::new(800, 600));
    Ok(build(&tree, &dom))
}

#[test]
fn no_commands_for_unstyled_element() -> Result<(), Error> {
    // A bare <p> with no background, border, or visible text after trim
    // should produce zero commands.  Whitespace-only "hi" is rendered.
    let list = run("<html><body><div></div></body></html>", "")?;
    list.is_empty()
        .then_some(())
        .ok_or_else(|| fail("expected empty list"))
}

#[test]
fn emits_background_fill() -> Result<(), Error> {
    let list = run(
        "<html><body><div></div></body></html>",
        "div { background-color: red; height: 100px; }",
    )?;
    list.commands()
        .iter()
        .any(|c| matches!(c, PaintCommand::FillRect { .. }))
        .then_some(())
        .ok_or_else(|| fail("expected FillRect for background"))
}

#[test]
fn emits_text_for_paragraph() -> Result<(), Error> {
    let list = run("<html><body><p>hello</p></body></html>", "")?;
    list.commands()
        .iter()
        .any(|c| matches!(c, PaintCommand::FillText { text, .. } if text == "hello"))
        .then_some(())
        .ok_or_else(|| fail("expected FillText for paragraph"))
}

#[test]
fn skips_transparent_background() -> Result<(), Error> {
    let list = run(
        "<html><body><div></div></body></html>",
        "div { background-color: transparent; }",
    )?;
    list.commands()
        .iter()
        .all(|c| !matches!(c, PaintCommand::FillRect { .. }))
        .then_some(())
        .ok_or_else(|| fail("transparent should not emit FillRect"))
}

#[test]
fn emits_border_fills_when_widths_set() -> Result<(), Error> {
    let list = run(
        "<html><body><div></div></body></html>",
        "div { border-width: 4px; height: 50px; }",
    )?;
    let fill_count = list
        .commands()
        .iter()
        .filter(|c| matches!(c, PaintCommand::FillRect { .. }))
        .count();
    (fill_count >= 4)
        .then_some(())
        .ok_or_else(|| fail("expected >=4 FillRects for 4 borders"))
}

#[test]
fn parent_before_child() -> Result<(), Error> {
    let list = run(
        "<html><body><div><p>x</p></div></body></html>",
        "div { background-color: blue; height: 50px; } p { background-color: red; }",
    )?;
    let positions: Vec<Option<f64>> = list
        .commands()
        .iter()
        .map(|c| match c {
            PaintCommand::FillRect { color, .. } if color.blue() > 0.5 && color.red() < 0.5 => {
                Some(0.0)
            }
            PaintCommand::FillRect { color, .. } if color.red() > 0.5 && color.blue() < 0.5 => {
                Some(1.0)
            }
            _other => None,
        })
        .filter_map(|p| p)
        .collect();
    // We expect the blue (parent) to come before the red (child) in
    // paint order.
    let pair = positions.iter().take(2).copied().collect::<Vec<_>>();
    (pair.first() == Some(&0.0) && pair.get(1) == Some(&1.0))
        .then_some(())
        .ok_or_else(|| fail("parent-before-child order broken"))
}
