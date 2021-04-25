// #![allow(unused_imports)]
#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into,
    clippy::upper_case_acronyms,
    clippy::new_ret_no_self,
    clippy::wrong_self_convention,
    clippy::if_same_then_else,
    clippy::float_cmp,
    clippy::needless_return,
    clippy::collapsible_if
)]

mod backend;
pub use backend::*;

pub use animate::*;

#[doc(hidden)]
pub mod prelude {
    pub use super::ActorManagerExt;
    pub use super::AdjustmentExt;
    pub use super::BoxLayoutChildExt;
    // pub use super::BoxLayoutExt; // overlap
    pub use super::ButtonExt;
    pub use super::ButtonGroupExt;
    pub use super::ClipboardExt;
    pub use super::ComboBoxExt;
    pub use super::DialogExt;
    pub use super::EntryExt;
    pub use super::ExpanderExt;
    pub use super::FadeEffectExt;
    pub use super::FocusManagerExt;
    pub use super::GridExt;
    pub use super::IconExt;
    pub use super::IconThemeExt;
    // pub use super::ImageExt; // overlap
    pub use super::ItemViewExt;
    pub use super::KineticScrollViewExt;
    pub use super::LabelExt;
    pub use super::ListViewExt;
    pub use super::MenuExt;
    pub use super::NotebookExt;
    pub use super::PagerExt;
    pub use super::PathBarExt;
    pub use super::ProgressBarExt;
    pub use super::PushActionExt;
    pub use super::ScrollBarExt;
    pub use super::ScrollViewExt;
    pub use super::SettingsExt;
    pub use super::SliderExt;
    pub use super::SpinnerExt;
    pub use super::StackChildExt;
    pub use super::StackExt;
    pub use super::StyleExt;
    pub use super::SurfaceExt;
    pub use super::TableChildExt;
    pub use super::TableExt;
    pub use super::TextureCacheExt;
    pub use super::ToggleExt;
    pub use super::ToolbarExt;
    pub use super::TooltipExt;
    pub use super::ViewportExt;
    pub use super::WidgetExt;
    pub use super::WindowExt;

    pub use dx;
    
    pub use animate::prelude::*;
    pub use ux_macro::*;
    pub use primitives::prelude::*;

    pub mod application {
        pub use animate::{run, init, quit};
    }

    pub use super::Opacity;
}

pub trait Opacity {
    fn opacity(&self, val :u8) -> Self;
}

impl Opacity for Color {
    fn opacity(&self, val :u8) -> Self {
        let color = *self;
        let RgbColor { red, green, blue} = color.into();
        Self::RGBA(red, green, blue, val)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
