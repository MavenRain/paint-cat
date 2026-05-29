//! Walk a `LayoutTree` and emit a [`DisplayList`].

use dom_cat::{Document, Node};
use layout_cat::{LayoutBox, LayoutTree, Point, Rect};

use crate::command::PaintCommand;
use crate::display_list::DisplayList;

/// Build a display list for `tree`.  `dom` is consulted for text-node
/// content under each layout box.
#[must_use]
pub fn build(tree: &LayoutTree, dom: &Document) -> DisplayList {
    let commands = tree
        .root_box()
        .map(|root| paint_box(root, dom, Vec::new()))
        .unwrap_or_default();
    DisplayList::new(commands)
}

fn paint_box(
    layout_box: &LayoutBox,
    dom: &Document,
    acc: Vec<PaintCommand>,
) -> Vec<PaintCommand> {
    let with_bg = emit_background(layout_box, acc);
    let with_border = emit_borders(layout_box, with_bg);
    let with_text = emit_text(layout_box, dom, with_border);
    layout_box
        .children()
        .iter()
        .fold(with_text, |acc, child| paint_box(child, dom, acc))
}

fn emit_background(layout_box: &LayoutBox, acc: Vec<PaintCommand>) -> Vec<PaintCommand> {
    let color = layout_box.style().background_color();
    if color.alpha() == 0.0 {
        acc
    } else {
        let rect = padding_box_rect(layout_box);
        append(
            acc,
            PaintCommand::FillRect {
                rect,
                color,
            },
        )
    }
}

fn emit_borders(layout_box: &LayoutBox, acc: Vec<PaintCommand>) -> Vec<PaintCommand> {
    let b = layout_box.border();
    if b.top() == 0.0 && b.right() == 0.0 && b.bottom() == 0.0 && b.left() == 0.0 {
        acc
    } else {
        let outer = border_box_rect(layout_box);
        let inner = padding_box_rect(layout_box);
        let color = layout_box.style().color();
        let with_top = if b.top() > 0.0 {
            append(
                acc,
                PaintCommand::FillRect {
                    rect: Rect::new(outer.origin(), outer.width(), b.top()),
                    color,
                },
            )
        } else {
            acc
        };
        let with_bottom = if b.bottom() > 0.0 {
            append(
                with_top,
                PaintCommand::FillRect {
                    rect: Rect::new(
                        Point::new(outer.origin().x(), inner.origin().y() + inner.height()),
                        outer.width(),
                        b.bottom(),
                    ),
                    color,
                },
            )
        } else {
            with_top
        };
        let with_left = if b.left() > 0.0 {
            append(
                with_bottom,
                PaintCommand::FillRect {
                    rect: Rect::new(outer.origin(), b.left(), outer.height()),
                    color,
                },
            )
        } else {
            with_bottom
        };
        if b.right() > 0.0 {
            append(
                with_left,
                PaintCommand::FillRect {
                    rect: Rect::new(
                        Point::new(inner.origin().x() + inner.width(), outer.origin().y()),
                        b.right(),
                        outer.height(),
                    ),
                    color,
                },
            )
        } else {
            with_left
        }
    }
}

fn emit_text(
    layout_box: &LayoutBox,
    dom: &Document,
    acc: Vec<PaintCommand>,
) -> Vec<PaintCommand> {
    let dom_id = layout_box.dom_node();
    let text_children: Vec<String> = dom
        .get(dom_id)
        .map(Node::children)
        .unwrap_or(&[])
        .iter()
        .filter_map(|&child_id| match dom.get(child_id) {
            Some(Node::Text(t)) => Some(t.content().to_owned()),
            Some(Node::Document(_) | Node::Element(_) | Node::Comment(_)) | None => None,
        })
        .collect();
    if text_children.is_empty() {
        acc
    } else {
        let content = text_children.join("");
        let trimmed = content.trim();
        if trimmed.is_empty() {
            acc
        } else {
            let font_size = layout_box
                .style()
                .font_size()
                .resolve(layout_box.rect().width(), 16.0)
                .unwrap_or(16.0);
            append(
                acc,
                PaintCommand::FillText {
                    rect: layout_box.rect(),
                    text: trimmed.to_owned(),
                    color: layout_box.style().color(),
                    font_size,
                },
            )
        }
    }
}

fn append(acc: Vec<PaintCommand>, cmd: PaintCommand) -> Vec<PaintCommand> {
    acc.into_iter().chain(std::iter::once(cmd)).collect()
}

fn padding_box_rect(layout_box: &LayoutBox) -> Rect {
    let content = layout_box.rect();
    let p = layout_box.padding();
    Rect::new(
        Point::new(content.origin().x() - p.left(), content.origin().y() - p.top()),
        content.width() + p.left() + p.right(),
        content.height() + p.top() + p.bottom(),
    )
}

fn border_box_rect(layout_box: &LayoutBox) -> Rect {
    let pad = padding_box_rect(layout_box);
    let b = layout_box.border();
    Rect::new(
        Point::new(pad.origin().x() - b.left(), pad.origin().y() - b.top()),
        pad.width() + b.left() + b.right(),
        pad.height() + b.top() + b.bottom(),
    )
}

