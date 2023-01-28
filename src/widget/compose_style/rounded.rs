use glazier::kurbo::{Affine, Point, RoundedRect, RoundedRectRadii, Size};
use vello::{
    peniko::{BlendMode, Brush, Compose, Mix, Stroke},
    SceneBuilder,
};

use crate::{widget::BoxConstraints, Widget};

pub struct BorderWidget<W: Widget> {
    pub(crate) widget: W,
    pub radii: RoundedRectRadii,
    pub stroke: Stroke,
    pub brush: Brush,
}

impl<W: Widget> BorderWidget<W> {
    pub fn new(widget: W, radii: RoundedRectRadii, stroke: Stroke, brush: Brush) -> Self {
        Self {
            widget,
            radii,
            stroke,
            brush,
        }
    }
}

impl<W: Widget> Widget for BorderWidget<W> {
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
        let halfwidth = self.stroke.width as f64 / 2.0;
        let origin = Point::new(halfwidth, halfwidth);
        let border = RoundedRect::from_origin_size(
            origin,
            cx.size() - Size::from((halfwidth, halfwidth)),
            self.radii,
        );
        builder.push_layer(
            BlendMode::new(Mix::Normal, Compose::Copy),
            1.0,
            Affine::IDENTITY,
            &border,
        );
        self.widget.paint(cx, builder);
        builder.pop_layer();
        builder.stroke(&self.stroke, Affine::IDENTITY, &self.brush, None, &border);
    }

    fn accessibility(&mut self, cx: &mut crate::widget::AccessCx) {
        self.widget.accessibility(cx)
    }
}
