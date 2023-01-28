use crate::{widget::ChangeFlags, View};

pub fn checkbox<T, A>(on_change: impl Fn(&mut T, bool) -> A + Send + 'static) -> impl View<T, A> {
    Checkbox {
        on_change: Box::new(on_change),
    }
}

pub struct Checkbox<T, A> {
    on_change: Box<dyn Fn(&mut T, bool) -> A + Send>,
}

impl<T, A> View<T, A> for Checkbox<T, A> {
    type State = ();
    type Element = crate::widget::checkbox::Checkbox;

    fn build(&self, cx: &mut super::Cx) -> (crate::id::Id, Self::State, Self::Element) {
        let (id, el) =
            cx.with_new_id(|cx| crate::widget::checkbox::Checkbox::new(cx.id_path().clone()));
        (id, (), el)
    }

    fn rebuild(
        &self,
        cx: &mut super::Cx,
        prev: &Self,
        id: &mut crate::id::Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        ChangeFlags::empty()
    }

    fn message(
        &self,
        id_path: &[crate::id::Id],
        state: &mut Self::State,
        message: Box<dyn std::any::Any>,
        app_state: &mut T,
    ) -> crate::event::MessageResult<A> {
        crate::event::MessageResult::Action((self.on_change)(
            app_state,
            *message.downcast_ref().unwrap(),
        ))
    }
}
