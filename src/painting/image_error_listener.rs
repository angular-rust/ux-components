pub trait ImageErrorListener<T> {
    fn on_error(&self, error: T);
}
