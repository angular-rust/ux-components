pub trait DrawerCallback {
    fn on_drawer(&self, is_opened: bool);
}
