use dioxus::prelude::{Signal, WritableExt};

pub trait ResetSignal<T: Default + 'static> {
    fn reset(&mut self);
}

impl<T: Default + 'static> ResetSignal<T> for Signal<T> {
    fn reset(&mut self) {
        self.set(T::default());
        self.replace(T::default());
    }
}
