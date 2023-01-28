use glazier::kurbo::{Affine, Size};
use vello::{SceneBuilder, SceneFragment};

use crate::{widget::BoxConstraints, Widget};

pub struct PaddingWidget<W: Widget> {
    pub(crate) widget: W,
    pub width: f64,
}

impl<W: Widget> PaddingWidget<W> {
    pub fn new(widget: W, width: f64) -> Self {
        Self { widget, width }
    }
}

impl<W: Widget> Widget for PaddingWidget<W> {
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
        cx.request_layout();
        self.widget.update(cx)
    }

    fn layout(&mut self, cx: &mut crate::widget::LayoutCx, bc: &BoxConstraints) -> Size {
        let padding_size = Size::new(self.width * 2., self.width * 2.);
        let bc = bc.shrink(padding_size);
        let result = self.widget.layout(cx, &bc) + padding_size;
        result
    }

    fn paint(&mut self, cx: &mut crate::widget::PaintCx, builder: &mut SceneBuilder) {
        let mut fragment = SceneFragment::new();
        let mut sub_builder = SceneBuilder::for_fragment(&mut fragment);
        self.widget.paint(cx, &mut sub_builder);
        sub_builder.finish();
        builder.append(&fragment, Some(Affine::translate((self.width, self.width))));
    }

    fn accessibility(&mut self, cx: &mut crate::widget::AccessCx) {
        self.widget.accessibility(cx)
    }
}
