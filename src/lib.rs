#![allow(unused_imports)]

mod backend;
pub use backend::*;

#[doc(hidden)]
pub mod prelude {

    pub trait Object: std::fmt::Debug + Clone + 'static {}
    pub trait Is<T: Object>: AsRef<T> + 'static {}

    pub use super::ActionExt;
    pub use super::ActorManagerExt;
    pub use super::AdjustmentExt;
    pub use super::BoxLayoutChildExt;
    pub use super::BoxLayoutExt;
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
    pub use super::ImageExt;
    pub use super::ItemViewExt;
    pub use super::KineticScrollViewExt;
    pub use super::LabelExt;
    pub use super::ListViewExt;
    pub use super::MenuExt;
    pub use super::NotebookExt;
    pub use super::PagerExt;
    pub use super::PathBarExt;
    pub use super::ProgressBarExt;
    pub use super::ScrollBarExt;
    pub use super::ScrollViewExt;
    pub use super::SettingsExt;
    pub use super::SliderExt;
    pub use super::SpinnerExt;
    pub use super::StackChildExt;
    pub use super::StackExt;
    pub use super::StyleExt;
    pub use super::TableChildExt;
    pub use super::TableExt;
    pub use super::TextureCacheExt;
    pub use super::ToggleExt;
    pub use super::ToolbarExt;
    pub use super::TooltipExt;
    pub use super::ViewportExt;
    pub use super::WidgetExt;
    pub use super::WindowExt;

    pub use ux_macro::*;
    pub use clutter;
    pub use clutter::prelude::*;
    pub use cogl;
    // pub use cogl::prelude::*;
}

impl prelude::Object for clutter::Actor {}
impl prelude::Is<clutter::Actor> for clutter::Actor {}

impl prelude::Object for clutter::Model {}
impl prelude::Is<clutter::Model> for clutter::Model {}

impl prelude::Object for clutter::OffscreenEffect {}
impl prelude::Is<clutter::OffscreenEffect> for clutter::OffscreenEffect {}

impl prelude::Object for clutter::Effect {}
impl prelude::Is<clutter::Effect> for clutter::Effect {}

impl prelude::Object for clutter::ActorMeta {}
impl prelude::Is<clutter::ActorMeta> for clutter::ActorMeta {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}