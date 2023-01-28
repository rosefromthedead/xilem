use vello::peniko::Color;

use crate::View;

use super::style::{BackgroundView, PaddingView};

pub trait ViewExt<T, A>: Sized {
    fn padding(self, width: f64) -> PaddingView<Self>;
    fn background(self, color: Color) -> BackgroundView<Self>;
}

impl<T, A, V: View<T, A>> ViewExt<T, A> for V {
    fn padding(self, width: f64) -> PaddingView<Self> {
        PaddingView::new(width, self)
    }

    fn background(self, color: Color) -> BackgroundView<Self> {
        BackgroundView::new(color, self)
    }
}
