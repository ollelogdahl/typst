use std::mem;

use super::*;
use crate::eval::Abs;
use crate::geom::{Linear, Sides};
use crate::paper::{Paper, PaperClass};

/// `page`: Configure pages.
///
/// # Positional arguments
/// - The name of a paper, e.g. `a4` (optional).
///
/// # Keyword arguments
/// - `width`: The width of pages (length).
/// - `height`: The height of pages (length).
/// - `margins`: The margins for all sides (length or relative to side lengths).
/// - `left`: The left margin (length or relative to width).
/// - `right`: The right margin (length or relative to width).
/// - `top`: The top margin (length or relative to height).
/// - `bottom`: The bottom margin (length or relative to height).
/// - `flip`: Flips custom or paper-defined width and height (boolean).
pub async fn page(_: Span, mut args: DictValue, ctx: LayoutContext<'_>) -> Pass<Value> {
    let mut f = Feedback::new();
    let mut style = ctx.style.page;

    if let Some(paper) = args.take::<Paper>() {
        style.class = paper.class;
        style.size = paper.size();
    }

    if let Some(Abs(width)) = args.take_key::<Abs>("width", &mut f) {
        style.class = PaperClass::Custom;
        style.size.width = width;
    }

    if let Some(Abs(height)) = args.take_key::<Abs>("height", &mut f) {
        style.class = PaperClass::Custom;
        style.size.height = height;
    }

    if let Some(margins) = args.take_key::<Linear>("margins", &mut f) {
        style.margins = Sides::uniform(Some(margins));
    }

    if let Some(left) = args.take_key::<Linear>("left", &mut f) {
        style.margins.left = Some(left);
    }

    if let Some(top) = args.take_key::<Linear>("top", &mut f) {
        style.margins.top = Some(top);
    }

    if let Some(right) = args.take_key::<Linear>("right", &mut f) {
        style.margins.right = Some(right);
    }

    if let Some(bottom) = args.take_key::<Linear>("bottom", &mut f) {
        style.margins.bottom = Some(bottom);
    }

    if args.take_key::<bool>("flip", &mut f).unwrap_or(false) {
        mem::swap(&mut style.size.width, &mut style.size.height);
    }

    args.unexpected(&mut f);
    Pass::commands(vec![SetPageStyle(style)], f)
}

/// `pagebreak`: Ends the current page.
pub async fn pagebreak(_: Span, args: DictValue, _: LayoutContext<'_>) -> Pass<Value> {
    let mut f = Feedback::new();
    args.unexpected(&mut f);
    Pass::commands(vec![BreakPage], f)
}
