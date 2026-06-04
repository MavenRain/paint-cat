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
    let sheet = css_cat::parse(css).map_err(|_| fail("css parse"))?;
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
    let positions: Vec<u8> = list
        .commands()
        .iter()
        .filter_map(|c| match c {
            PaintCommand::FillRect { color, .. } if color.blue() > 0.5 && color.red() < 0.5 => {
                Some(0_u8)
            }
            PaintCommand::FillRect { color, .. } if color.red() > 0.5 && color.blue() < 0.5 => {
                Some(1_u8)
            }
            PaintCommand::FillRect { .. }
            | PaintCommand::StrokeRect { .. }
            | PaintCommand::FillText { .. } => None,
        })
        .collect();
    (positions.first() == Some(&0) && positions.get(1) == Some(&1))
        .then_some(())
        .ok_or_else(|| fail("parent-before-child order broken"))
}

#[test]
fn scaled_fillrect_doubles_geometry() -> Result<(), Error> {
    use layout_cat::{Color, Point, Rect};
    let cmd = PaintCommand::FillRect {
        rect: Rect::new(Point::new(10.0, 20.0), 100.0, 50.0),
        color: Color::rgba(0.0, 0.0, 0.0, 1.0),
    };
    match cmd.scaled(2.0) {
        PaintCommand::FillRect { rect, .. } => (rect.origin().x() == 20.0
            && rect.origin().y() == 40.0
            && rect.width() == 200.0
            && rect.height() == 100.0)
            .then_some(())
            .ok_or_else(|| fail("FillRect geometry did not double")),
        PaintCommand::StrokeRect { .. } | PaintCommand::FillText { .. } => {
            Err(fail("expected FillRect variant"))
        }
    }
}

#[test]
fn scaled_filltext_scales_font_size() -> Result<(), Error> {
    use layout_cat::{Color, Point, Rect};
    let cmd = PaintCommand::FillText {
        rect: Rect::new(Point::new(0.0, 0.0), 100.0, 30.0),
        text: "hi".to_owned(),
        color: Color::rgba(0.0, 0.0, 0.0, 1.0),
        font_size: 16.0,
    };
    match cmd.scaled(1.5) {
        PaintCommand::FillText {
            font_size, rect, ..
        } => ((font_size - 24.0).abs() < f64::EPSILON
            && (rect.width() - 150.0).abs() < f64::EPSILON)
            .then_some(())
            .ok_or_else(|| fail("FillText font_size or rect did not scale")),
        PaintCommand::FillRect { .. } | PaintCommand::StrokeRect { .. } => {
            Err(fail("expected FillText variant"))
        }
    }
}

#[test]
fn scaled_strokerect_triples_width() -> Result<(), Error> {
    use layout_cat::{Color, Point, Rect};
    let cmd = PaintCommand::StrokeRect {
        rect: Rect::new(Point::new(0.0, 0.0), 100.0, 50.0),
        color: Color::rgba(0.0, 0.0, 0.0, 1.0),
        width: 2.0,
    };
    match cmd.scaled(3.0) {
        PaintCommand::StrokeRect { width, .. } => ((width - 6.0).abs() < f64::EPSILON)
            .then_some(())
            .ok_or_else(|| fail("StrokeRect width did not triple")),
        PaintCommand::FillRect { .. } | PaintCommand::FillText { .. } => {
            Err(fail("expected StrokeRect variant"))
        }
    }
}

#[test]
fn display_list_scaled_preserves_order_and_count() -> Result<(), Error> {
    use layout_cat::{Color, Point, Rect};
    let original = DisplayList::new(vec![
        PaintCommand::FillRect {
            rect: Rect::new(Point::new(0.0, 0.0), 10.0, 10.0),
            color: Color::rgba(1.0, 0.0, 0.0, 1.0),
        },
        PaintCommand::FillText {
            rect: Rect::new(Point::new(10.0, 10.0), 80.0, 20.0),
            text: "x".to_owned(),
            color: Color::rgba(0.0, 0.0, 0.0, 1.0),
            font_size: 12.0,
        },
    ]);
    let scaled = original.scaled(2.0);
    let same_count = scaled.len() == original.len();
    let first_is_fillrect = matches!(
        scaled.commands().first(),
        Some(PaintCommand::FillRect { .. })
    );
    let second_is_filltext = matches!(
        scaled.commands().get(1),
        Some(PaintCommand::FillText { .. })
    );
    (same_count && first_is_fillrect && second_is_filltext)
        .then_some(())
        .ok_or_else(|| fail("scaled list lost count or reordered"))
}
