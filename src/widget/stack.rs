// Copyright 2022 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use glazier::kurbo::{Affine, Rect, Size};
use vello::{
    peniko::{BrushRef, Color, Fill},
    SceneBuilder,
};

use super::{
    contexts::LifeCycleCx, raw_event::Event, Axis, BoxConstraints, ChangeFlags, EventCx, LayoutCx,
    LifeCycle, PaintCx, Pod, UpdateCx, Widget,
};

pub struct Stack {
    children: Vec<Pod>,
    spacing: f64,
    axis: Axis,
}

impl Stack {
    pub fn new(children: Vec<Pod>, spacing: f64, axis: Axis) -> Self {
        Stack {
            children,
            spacing,
            axis,
        }
    }

    pub fn children_mut(&mut self) -> &mut Vec<Pod> {
        &mut self.children
    }

    pub fn set_axis(&mut self, axis: Axis) -> ChangeFlags {
        self.axis = axis;
        ChangeFlags::LAYOUT | ChangeFlags::PAINT | ChangeFlags::UPDATE
    }
}

impl Widget for Stack {
    fn event(&mut self, cx: &mut EventCx, event: &Event) {
        for child in &mut self.children {
            child.event(cx, event);
        }
    }

    fn lifecycle(&mut self, cx: &mut LifeCycleCx, event: &LifeCycle) {
        for child in &mut self.children {
            child.lifecycle(cx, event);
        }
    }

    fn update(&mut self, cx: &mut UpdateCx) {
        for child in &mut self.children {
            child.update(cx);
        }
        cx.request_layout();
    }

    fn layout(&mut self, cx: &mut LayoutCx, bc: &BoxConstraints) -> Size {
        // Transpose - Constructs a size with orientation depending on whether we are vertical or horizontal
        let t = |length, perp| {
            if self.axis == Axis::Horizontal {
                Size::new(length, perp)
            } else {
                Size::new(perp, length)
            }
        };
        // Length - Retrieves the length from a Size
        let l = |size: Size| {
            if self.axis == Axis::Horizontal {
                size.width
            } else {
                size.height
            }
        };
        // Perpendicular length - Retrieves the perpendicular length (width of a vstack, height of an hstack) from a Size
        let p = |size: Size| {
            if self.axis == Axis::Horizontal {
                size.height
            } else {
                size.width
            }
        };

        let mut bc = bc.shrink(t(self.spacing * (self.children.len() - 1) as f64, 0.0));
        bc = bc.loosen();
        // Offer a portion of remaining length to each child
        for (i, child) in self.children.iter_mut().enumerate().rev() {
            let child_max_length = l(bc.max()) / i as f64 + 1.0;
            let child_bc = bc.shrink_max_to(self.axis, child_max_length);
            let child_size = child.layout(cx, &child_bc);

            // Shrink the remaining space by the size that was just allocated to a child
            bc = bc.shrink(t(l(child_size), 0.0));
        }
        // Place children, using computed height and alignments
        let mut length = 0.0;
        let mut perp = 0.0f64;
        for (i, child) in self.children.iter_mut().enumerate() {
            if i != 0 {
                length += self.spacing;
            }

            child.state.origin = t(length, 0.0).to_vec2().to_point(); // sorry about that

            let child_size = child.state.size;
            perp = perp.max(p(child_size));
            length += l(child_size);
        }
        t(length, perp)
    }

    fn paint(&mut self, cx: &mut PaintCx, b: &mut SceneBuilder) {
        b.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            BrushRef::Solid(Color::rgb8(0x20, 0x20, 0x28)),
            None,
            &Rect::from_origin_size((0.0, 0.0), cx.widget_state.size),
        );
        for child in &mut self.children {
            child.paint(cx);
            b.append(
                child.fragment(),
                Some(Affine::translate(child.state.origin.to_vec2())),
            );
        }
    }

    fn accessibility(&mut self, cx: &mut super::AccessCx) {
        todo!()
    }
}
