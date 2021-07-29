// #![allow(unused_imports)]
#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into,
    clippy::if_same_then_else,
    clippy::float_cmp,
    clippy::collapsible_if
)]

#![doc(html_logo_url = "https://dudochkin-victor.github.io/assets/ux-components/logo.svg")]

mod backend;
pub use backend::*;

mod theme;
pub use theme::*;

pub use animate::*;

#[doc(hidden)]
pub mod prelude {
    pub use super::AppBarExt;
    pub use super::BackdropExt;
    pub use super::BannerExt;
    pub use super::ButtonExt;
    pub use super::CardExt;
    pub use super::CheckboxExt;
    pub use super::ChipExt;
    pub use super::CircularProgressExt;
    pub use super::DataTableExt;
    pub use super::DatePickerExt;
    pub use super::DialogExt;
    pub use super::DividerExt;
    pub use super::DrawerExt;
    pub use super::FabExt;
    pub use super::FormfieldExt;
    pub use super::IconButtonExt;
    pub use super::ImageListExt;
    pub use super::LinearProgressExt;
    pub use super::ListExt;
    pub use super::MenuExt;
    pub use super::RadioExt;
    pub use super::SheetExt;
    pub use super::SliderExt;
    pub use super::SnackbarExt;
    pub use super::SurfaceExt;
    pub use super::SwitchExt;
    pub use super::TabExt;
    pub use super::TextareaExt;
    pub use super::TextfieldExt;
    pub use super::TimePickerExt;
    pub use super::TooltipExt;
    pub use super::WidgetExt;
    pub use super::WindowExt;

    pub use dx;

    pub use animate::prelude::*;
    pub use ux_macro::*;

    pub mod application {
        pub use animate::{init, quit, run};
    }

    pub use super::Opacity;
}

pub trait Opacity {
    fn opacity(&self, val: u8) -> Self;
}

impl Opacity for Color {
    fn opacity(&self, val: u8) -> Self {
        let color = *self;
        let RgbColor { red, green, blue } = color.into();
        Self::rgba(red, green, blue, val)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
