use glazier::kurbo::{Affine, Size};
use vello::{
    peniko::{Color, Fill},
    SceneBuilder,
};

use crate::{widget::BoxConstraints, Widget};

pub struct BackgroundWidget<W: Widget> {
    pub(crate) widget: W,
    color: Color,
}

impl<W: Widget> BackgroundWidget<W> {
    pub fn new(widget: W, color: Color) -> Self {
        Self { widget, color }
    }
    pub(crate) fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl<W: Widget> Widget for BackgroundWidget<W> {
    fn event(&mut self, cx: &mut crate::widget::EventCx, event: &crate::widget::Event) {
        self.widget.event(cx, event)
    }

    fn lifecycle(
        &mut self,
        cx: &mut crate::widget::contexts::LifeCycleCx,
        event: &crate::widget::LifeCycle,
    ) {
        self.widget.lifecycle(cx, event)
    }

    fn update(&mut self, cx: &mut crate::widget::UpdateCx) {
        self.widget.update(cx)
    }

    fn layout(&mut self, cx: &mut crate::widget::LayoutCx, bc: &BoxConstraints) -> Size {
        self.widget.layout(cx, bc)
    }

    fn paint(&mut self, cx: &mut crate::widget::PaintCx, builder: &mut SceneBuilder) {
        builder.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            self.color,
            None,
            &cx.size().to_rect(),
        );
        self.widget.paint(cx, builder);
    }

    fn accessibility(&mut self, cx: &mut crate::widget::AccessCx) {
        self.widget.accessibility(cx)
    }
}
