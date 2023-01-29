use glazier::kurbo::{Affine, BezPath, Line, Point, RoundedRect, Size};
use vello::{
    peniko::{BrushRef, Cap, Color, Fill, Stroke},
    SceneBuilder,
};

use crate::{event::Message, id::IdPath, Widget};

use super::{Event, LifeCycle};

pub struct Checkbox {
    path: IdPath,
    pub enabled: bool,
}

impl Checkbox {
    pub fn new(path: IdPath, enabled: bool) -> Self {
        Checkbox { path, enabled }
    }
}

impl Widget for Checkbox {
    fn event(&mut self, cx: &mut super::EventCx, event: &Event) {
        match event {
            Event::MouseDown(_) => {
                cx.set_active(true);
            }
            Event::MouseUp(_) => {
                self.enabled = !self.enabled;
                cx.add_message(Message::new(self.path.clone(), self.enabled));
                cx.set_active(false);
            }
            Event::TargetedAccessibilityAction(_) => todo!(),
            _ => {}
        }
    }

    fn lifecycle(&mut self, cx: &mut super::contexts::LifeCycleCx, event: &super::LifeCycle) {
        match event {
            LifeCycle::HotChanged(a) => {}
        }
    }

    fn update(&mut self, cx: &mut super::UpdateCx) {}

    fn layout(&mut self, cx: &mut super::LayoutCx, bc: &super::BoxConstraints) -> Size {
        bc.constrain((16.0, 16.0))
    }

    fn accessibility(&mut self, cx: &mut super::AccessCx) {
        todo!()
    }

    fn paint(&mut self, cx: &mut super::PaintCx, b: &mut SceneBuilder) {
        b.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            BrushRef::Solid(Color::rgb8(20, 120, 200)),
            None,
            &RoundedRect::from_origin_size(Point::ZERO, cx.size(), 4.0),
        );
        if self.enabled {
            let mut tick = BezPath::new();
            tick.move_to((4.0, 8.0));
            tick.line_to((8.0, 12.0));
            tick.line_to((12.0, 4.0));
            b.stroke(
                &Stroke::new(2.0).with_caps(Cap::Round),
                Affine::IDENTITY,
                BrushRef::Solid(Color::rgb8(220, 220, 220)),
                None,
                &tick,
            );
        }
    }
}
