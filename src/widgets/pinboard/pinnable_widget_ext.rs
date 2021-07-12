use druid::{Data, EventCtx, Widget, WidgetExt};
use druid::widget::ControllerHost;

use super::{
    Pinnable,
    on_dependent_changed::OnDependentChanged,
};

pub trait PinnableWidgetExt<T, W> where T: Data + Pinnable, W: Widget<T> {
    fn on_dependent_changed(self, callback: impl Fn(&mut EventCtx, &mut T, &T) -> () + 'static) -> ControllerHost<W, OnDependentChanged<T>>;
}

impl<T: Data + Pinnable, W: Widget<T> + 'static> PinnableWidgetExt<T, W> for W {
    fn on_dependent_changed(self, callback: impl Fn(&mut EventCtx, &mut T, &T) -> () + 'static) -> ControllerHost<W, OnDependentChanged<T>> {
        self.controller(OnDependentChanged::new(callback))
    }
}
