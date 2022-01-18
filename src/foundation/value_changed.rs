pub trait ValueChanged<T> {
    fn on_changed(&self, value: T);
}
