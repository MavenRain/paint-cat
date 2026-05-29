# paint-cat

Display-list construction: walks a [`layout-cat`](https://crates.io/crates/layout-cat) `LayoutTree` and emits a sequence of `PaintCommand`s (`FillRect`, `StrokeRect`, `FillText`) for a backend renderer to consume.

Backend-agnostic; no platform graphics dependencies.

`paint-cat` is the fifth sub-crate of a `comp-cat-rs` Servo-replacement webview runtime targeting Tauri integration.  Same framework constraints: no `mut`, no `Rc`/`Arc`, no interior mutability, no panics.

## Example

```rust
use paint_cat::{build, Error};

fn main() -> Result<(), Error> {
    let html_doc = html_cat::parse("<html><body><p>hi</p></body></html>")?;
    let dom = dom_cat::Document::from_html_doc(&html_doc);
    let sheet = css_cat::parse("p { background-color: red; padding: 8px; }")?;
    let tree = layout_cat::layout(&dom, &sheet, layout_cat::Viewport::new(800, 600));
    let display_list = build(&tree, &dom);
    assert!(!display_list.commands().is_empty());
    Ok(())
}
```

## v0 scope

- Walk `LayoutTree` in document order, emitting commands per box.
- Background: `FillRect` with `background-color` if non-transparent.
- Border: 4 `FillRect`s (top/right/bottom/left) when border-width > 0.
- Text: `FillText` for `Text` DOM children, using the parent box's content rect and color.
- Stacking: child boxes paint after their parent (back-to-front).

## Deferred to v0.2+

- z-index ordering / stacking contexts.
- `border-style`, `box-shadow`, `border-radius`, `clip-path`.
- Backgrounds beyond solid color.
- Text shaping / font selection.
- Compositing / opacity.

## License

MIT OR Apache-2.0
