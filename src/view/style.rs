use vello::peniko::Color;

use crate::{
    id::Id,
    widget::{
        compose_style::{background::BackgroundWidget, padding::PaddingWidget},
        ChangeFlags, Pod,
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
    type State = (Id, V::State);

    type Element = PaddingWidget;

    fn build(&self, cx: &mut crate::view::Cx) -> (crate::id::Id, Self::State, Self::Element) {
        let (id, (child_id, state, element)) = cx.with_new_id(|cx| self.view.build(cx));
        let element = PaddingWidget::new(Pod::new(element), self.width);
        (id, (child_id, state), element)
    }

    fn rebuild(
        &self,
        cx: &mut crate::view::Cx,
        prev: &Self,
        id: &mut crate::id::Id,
        (child_id, state): &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        let mut flags = ChangeFlags::empty();
        if prev.width != self.width {
            element.set_width(self.width);
            flags |= ChangeFlags::LAYOUT | ChangeFlags::PAINT;
        }
        flags |= cx.with_id(*id, |cx| {
            let child_element = element.widget.downcast_mut().unwrap();
            self.view
                .rebuild(cx, &prev.view, child_id, state, child_element)
        });
        flags
    }

    fn message(
        &self,
        id_path: &[crate::id::Id],
        (child_id, state): &mut Self::State,
        message: Box<dyn std::any::Any>,
        app_state: &mut T,
    ) -> crate::event::MessageResult<A> {
        let (left, right) = id_path.split_at(1);
        assert!(left[0] == *child_id);
        self.view.message(right, state, message, app_state)
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
    type State = (Id, V::State);

    type Element = BackgroundWidget;

    fn build(&self, cx: &mut crate::view::Cx) -> (crate::id::Id, Self::State, Self::Element) {
        let (id, (child_id, state, element)) = cx.with_new_id(|cx| self.view.build(cx));
        let element = BackgroundWidget::new(Pod::new(element), self.color);
        (id, (child_id, state), element)
    }

    fn rebuild(
        &self,
        cx: &mut crate::view::Cx,
        prev: &Self,
        id: &mut crate::id::Id,
        (child_id, state): &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        let mut flags = ChangeFlags::empty();
        if prev.color != self.color {
            element.set_color(self.color);
            flags |= ChangeFlags::LAYOUT | ChangeFlags::PAINT;
        }
        flags |= cx.with_id(*id, |cx| {
            let child_element = element.widget.downcast_mut().unwrap();
            self.view
                .rebuild(cx, &prev.view, child_id, state, child_element)
        });
        flags
    }

    fn message(
        &self,
        id_path: &[crate::id::Id],
        (child_id, state): &mut Self::State,
        message: Box<dyn std::any::Any>,
        app_state: &mut T,
    ) -> crate::event::MessageResult<A> {
        let (left, right) = id_path.split_at(1);
        assert!(left[0] == *child_id);
        self.view.message(right, state, message, app_state)
    }
}

impl<V> BackgroundView<V> {
    pub fn new(color: Color, view: V) -> Self {
        BackgroundView { color, view }
    }
}
