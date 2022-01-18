use crate::{ui::Size, widgets::Widget};

pub trait PreferredSizeWidget: Widget {
    fn preferred_size(&self) -> Size;
}
