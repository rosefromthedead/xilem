use glazier::kurbo::RoundedRectRadii;
use vello::peniko::{Brush, Color};

use crate::View;

use super::style::{BackgroundView, BorderView, PaddingView};

pub trait ViewExt<T, A>: Sized {
    fn padding(self, width: f64) -> PaddingView<Self>;
    fn background(self, color: Color) -> BackgroundView<Self>;
    fn border(
        self,
        radii: impl Into<RoundedRectRadii>,
        width: f32,
        brush: Brush,
    ) -> BorderView<Self>;
}

impl<T, A, V: View<T, A>> ViewExt<T, A> for V {
    fn padding(self, width: f64) -> PaddingView<Self> {
        PaddingView::new(width, self)
    }

    fn background(self, color: Color) -> BackgroundView<Self> {
        BackgroundView::new(color, self)
    }

    fn border(
        self,
        radii: impl Into<RoundedRectRadii>,
        width: f32,
        brush: Brush,
    ) -> BorderView<Self> {
        BorderView::new(radii.into(), width, brush, self)
    }
}
