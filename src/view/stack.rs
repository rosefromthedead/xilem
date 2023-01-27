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

use std::{any::Any, marker::PhantomData};

use crate::{
    id::Id,
    widget::{Axis, ChangeFlags, WidgetTuple},
};

use super::{Cx, View, ViewSequence};

pub struct Stack<T, A, VS: ViewSequence<T, A>> {
    children: VS,
    spacing: f64,
    axis: Axis,
    phantom: PhantomData<fn() -> (T, A)>,
}

pub fn stack<T, A, VT: ViewSequence<T, A>>(
    children: VT,
    spacing: f64,
    axis: Axis,
) -> Stack<T, A, VT> {
    Stack::new(children, spacing, axis)
}

pub fn vstack<T, A, VT: ViewSequence<T, A>>(children: VT, spacing: f64) -> Stack<T, A, VT> {
    Stack::new(children, spacing, Axis::Vertical)
}

pub fn hstack<T, A, VT: ViewSequence<T, A>>(children: VT, spacing: f64) -> Stack<T, A, VT> {
    Stack::new(children, spacing, Axis::Horizontal)
}

impl<T, A, VS: ViewSequence<T, A>> Stack<T, A, VS> {
    pub fn new(children: VS, spacing: f64, axis: Axis) -> Self {
        let phantom = Default::default();
        Stack {
            children,
            spacing,
            axis,
            phantom,
        }
    }
}

impl<T, A, VS: ViewSequence<T, A>> View<T, A> for Stack<T, A, VS>
where
    VS::Elements: WidgetTuple,
{
    type State = VS::State;

    type Element = crate::widget::stack::Stack;

    fn build(&self, cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        let (id, (state, elements)) = cx.with_new_id(|cx| self.children.build(cx));
        let stack = crate::widget::stack::Stack::new(elements, self.spacing, self.axis);
        (id, state, stack)
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        id: &mut Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        let mut flags = ChangeFlags::empty();
        if self.axis != prev.axis {
            flags |= element.set_axis(self.axis);
        }
        flags |= cx.with_id(*id, |cx| {
            self.children
                .rebuild(cx, &prev.children, state, element.children_mut())
        });
        flags
    }

    fn message(
        &self,
        id_path: &[Id],
        state: &mut Self::State,
        message: Box<dyn Any>,
        app_state: &mut T,
    ) -> crate::event::MessageResult<A> {
        self.children.message(id_path, state, message, app_state)
    }
}
