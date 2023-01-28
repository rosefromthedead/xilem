use vello::peniko::Color;

use crate::{
    widget::{
        compose_style::{background::BackgroundWidget, padding::PaddingWidget},
        ChangeFlags,
    },
    View,
};

pub struct PaddingView<V> {
    width: f64,
    view: V,
}

impl<T, A, V: View<T, A>> View<T, A> for PaddingView<V>
where
    V::Element: 'static,
{
    type State = V::State;

    type Element = PaddingWidget<V::Element>;

    fn build(&self, cx: &mut crate::view::Cx) -> (crate::id::Id, Self::State, Self::Element) {
        let (id, state, element) = self.view.build(cx);
        let element = PaddingWidget::new(element, self.width);
        (id, state, element)
    }

    fn rebuild(
        &self,
        cx: &mut crate::view::Cx,
        prev: &Self,
        id: &mut crate::id::Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        let mut flags = ChangeFlags::empty();
        if prev.width != self.width {
            element.set_width(self.width);
            flags |= ChangeFlags::LAYOUT | ChangeFlags::PAINT;
        }
        flags |= cx.with_id(*id, |cx| {
            self.view
                .rebuild(cx, &prev.view, id, state, &mut element.widget)
        });
        flags
    }

    fn message(
        &self,
        id_path: &[crate::id::Id],
        state: &mut Self::State,
        message: Box<dyn std::any::Any>,
        app_state: &mut T,
    ) -> crate::event::MessageResult<A> {
        self.view.message(id_path, state, message, app_state)
    }
}

impl<V> PaddingView<V> {
    pub fn new(width: f64, view: V) -> Self {
        PaddingView { width, view }
    }
}

pub struct BackgroundView<V> {
    color: Color,
    view: V,
}

impl<T, A, V: View<T, A>> View<T, A> for BackgroundView<V>
where
    V::Element: 'static,
{
    type State = V::State;

    type Element = BackgroundWidget<V::Element>;

    fn build(&self, cx: &mut crate::view::Cx) -> (crate::id::Id, Self::State, Self::Element) {
        let (id, state, element) = self.view.build(cx);
        let element = BackgroundWidget::new(element, self.color);
        (id, state, element)
    }

    fn rebuild(
        &self,
        cx: &mut crate::view::Cx,
        prev: &Self,
        id: &mut crate::id::Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        let mut flags = ChangeFlags::empty();
        if prev.color != self.color {
            element.set_color(self.color);
            flags |= ChangeFlags::LAYOUT | ChangeFlags::PAINT;
        }
        flags |= cx.with_id(*id, |cx| {
            self.view
                .rebuild(cx, &prev.view, id, state, &mut element.widget)
        });
        flags
    }

    fn message(
        &self,
        id_path: &[crate::id::Id],
        state: &mut Self::State,
        message: Box<dyn std::any::Any>,
        app_state: &mut T,
    ) -> crate::event::MessageResult<A> {
        self.view.message(id_path, state, message, app_state)
    }
}

impl<V> BackgroundView<V> {
    pub fn new(color: Color, view: V) -> Self {
        BackgroundView { color, view }
    }
}
