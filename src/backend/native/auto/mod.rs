#![allow(unused_variables)]
#![allow(dead_code)]
mod action;
// pub use self::action::{ActionExt, ActionClass};
pub use self::action::{Action, NONE_ACTION};

mod actor_manager;
// pub use self::actor_manager::{ActorManagerClass, ActorManagerExt};
pub use self::actor_manager::{ActorManager, NONE_ACTOR_MANAGER};

mod adjustment;
// pub use self::adjustment::{AdjustmentExt, AdjustmentClass};
pub use self::adjustment::{Adjustment, NONE_ADJUSTMENT};

mod application;
// pub use self::application::ApplicationExt, ApplicationClass};
pub use self::application::{Application, NONE_APPLICATION};

mod box_layout;
// pub use self::box_layout::BoxLayoutExt, BoxLayoutClass};
pub use self::box_layout::{BoxLayout, NONE_BOX_LAYOUT};

mod box_layout_child;
// pub use self::box_layout_child::{BoxLayoutChildClass, BoxLayoutChildExt};
pub use self::box_layout_child::{BoxLayoutChild, NONE_BOX_LAYOUT_CHILD};

mod button;
// pub use self::button::{ButtonExt, ButtonClass};
pub use self::button::{Button, NONE_BUTTON};

mod button_group;
// pub use self::button_group::{ButtonGroupExt, ButtonGroupClass};
pub use self::button_group::{ButtonGroup, NONE_BUTTON_GROUP};

mod clipboard;
// pub use self::clipboard::{ClipboardExt, ClipboardClass};
pub use self::clipboard::{Clipboard, NONE_CLIPBOARD};

mod combo_box;
// pub use self::combo_box::{ComboBoxExt, ComboBoxClass};
pub use self::combo_box::{ComboBox, NONE_COMBO_BOX};

mod dialog;
// pub use self::dialog::{DialogExt, DialogClass};
pub use self::dialog::{Dialog, NONE_DIALOG};

mod entry;
// pub use self::entry::{EntryExt, EntryClass};
pub use self::entry::{Entry, NONE_ENTRY};

mod expander;
// pub use self::expander::{ExpanderExt, ExpanderClass};
pub use self::expander::{Expander, NONE_EXPANDER};

mod fade_effect;
// pub use self::fade_effect::{FadeEffectExt, FadeEffectClass};
pub use self::fade_effect::{FadeEffect, NONE_FADE_EFFECT};

mod floating_widget;
// pub use self::floating_widget::FloatingWidgetClass;
pub use self::floating_widget::{FloatingWidget, NONE_FLOATING_WIDGET};

mod focus_manager;
// pub use self::focus_manager::{FocusManagerExt, FocusManagerClass};
pub use self::focus_manager::{FocusManager, NONE_FOCUS_MANAGER};

mod frame;
// pub use self::frame::FrameClass;
pub use self::frame::{Frame, NONE_FRAME};

mod grid;
// pub use self::grid::{GridExt, GridClass};
pub use self::grid::{Grid, NONE_GRID};

mod icon;
// pub use self::icon::{IconExt, IconClass};
pub use self::icon::{Icon, NONE_ICON};

mod icon_theme;
// pub use self::icon_theme::{IconThemeExt, IconThemeClass};
pub use self::icon_theme::{IconTheme, NONE_ICON_THEME};

mod image;
// pub use self::image::{ImageExt, ImageClass};
pub use self::image::{Image, NONE_IMAGE};

mod item_view;
// pub use self::item_view::{ItemViewExt, ItemViewClass};
pub use self::item_view::{ItemView, NONE_ITEM_VIEW};

mod kinetic_scroll_view;
// pub use self::kinetic_scroll_view::{KineticScrollViewClass, KineticScrollViewExt};
pub use self::kinetic_scroll_view::{KineticScrollView, NONE_KINETIC_SCROLL_VIEW};

mod label;
// pub use self::label::{LabelExt, LabelClass};
pub use self::label::{Label, NONE_LABEL};

mod list_view;
// pub use self::list_view::{ListViewExt, ListViewClass};
pub use self::list_view::{ListView, NONE_LIST_VIEW};

mod menu;
// pub use self::menu::{MenuExt, MenuClass};
pub use self::menu::{Menu, NONE_MENU};

mod notebook;
// pub use self::notebook::{NotebookExt, NotebookClass};
pub use self::notebook::{Notebook, NONE_NOTEBOOK};

mod pager;
// pub use self::pager::{PagerExt, PagerClass};
pub use self::pager::{Pager, NONE_PAGER};

mod path_bar;
// pub use self::path_bar::{PathBarExt, PathBarClass};
pub use self::path_bar::{PathBar, NONE_PATH_BAR};

mod progress_bar;
// pub use self::progress_bar::{ProgressBarExt, ProgressBarClass};
pub use self::progress_bar::{ProgressBar, NONE_PROGRESS_BAR};

mod scroll_bar;
// pub use self::scroll_bar::{ScrollBarExt, ScrollBarClass};
pub use self::scroll_bar::{ScrollBar, NONE_SCROLL_BAR};

mod scroll_view;
// pub use self::scroll_view::{ScrollViewExt, ScrollViewClass};
pub use self::scroll_view::{ScrollView, NONE_SCROLL_VIEW};

mod settings;
// pub use self::settings::{SettingsExt, SettingsClass};
pub use self::settings::{Settings, NONE_SETTINGS};

mod slider;
// pub use self::slider::{SliderExt, SliderClass};
pub use self::slider::{Slider, NONE_SLIDER};

mod spinner;
// pub use self::spinner::{SpinnerExt, SpinnerClass};
pub use self::spinner::{Spinner, NONE_SPINNER};

mod stack;
// pub use self::stack::{StackExt, StackClass};
pub use self::stack::{Stack, NONE_STACK};

mod stack_child;
// pub use self::stack_child::{StackChildExt, StackChildClass};
pub use self::stack_child::{StackChild, NONE_STACK_CHILD};

mod style;
// pub use self::style::{StyleExt, StyleClass};
pub use self::style::{Style, NONE_STYLE};

mod table;
// pub use self::table::{TableExt, TableClass};
pub use self::table::{Table, NONE_TABLE};

mod table_child;
// pub use self::table_child::{TableChildExt, TableChildClass};
pub use self::table_child::{TableChild, NONE_TABLE_CHILD};

mod texture_cache;
// pub use self::texture_cache::{TextureCacheExt, TextureCacheClass};
pub use self::texture_cache::{TextureCache, NONE_TEXTURE_CACHE};

mod toggle;
// pub use self::toggle::{ToggleExt, ToggleClass};
pub use self::toggle::{Toggle, NONE_TOGGLE};

mod toolbar;
// pub use self::toolbar::{ToolbarExt, ToolbarClass};
pub use self::toolbar::{Toolbar, NONE_TOOLBAR};

mod tooltip;
// pub use self::tooltip::{TooltipExt, TooltipClass};
pub use self::tooltip::{Tooltip, NONE_TOOLTIP};

mod viewport;
// pub use self::viewport::{ViewportExt, ViewportClass};
pub use self::viewport::{Viewport, NONE_VIEWPORT};

mod widget;
// pub use self::widget::{WidgetExt, WidgetClass};
pub use self::widget::{Widget, NONE_WIDGET};

mod window;
// pub use self::window::{WindowExt, WindowClass};
pub use self::window::{Window, NONE_WINDOW};

// #[doc(hidden)]
// pub mod traits {
//     pub use super::ActionExt;
//     pub use super::ActorManagerExt;
//     pub use super::AdjustmentExt;
//     pub use super::ApplicationExt;
//     pub use super::BoxLayoutChildExt;
//     pub use super::BoxLayoutExt;
//     pub use super::ButtonExt;
//     pub use super::ButtonGroupExt;
//     pub use super::ClipboardExt;
//     pub use super::ComboBoxExt;
//     pub use super::DialogExt;
//     pub use super::EntryExt;
//     pub use super::ExpanderExt;
//     pub use super::FadeEffectExt;
//     pub use super::FocusManagerExt;
//     pub use super::GridExt;
//     pub use super::IconExt;
//     pub use super::IconThemeExt;
//     pub use super::ImageExt;
//     pub use super::ItemViewExt;
//     pub use super::KineticScrollViewExt;
//     pub use super::LabelExt;
//     pub use super::ListViewExt;
//     pub use super::MenuExt;
//     pub use super::NotebookExt;
//     pub use super::PagerExt;
//     pub use super::PathBarExt;
//     pub use super::ProgressBarExt;
//     pub use super::ScrollBarExt;
//     pub use super::ScrollViewExt;
//     pub use super::SettingsExt;
//     pub use super::SliderExt;
//     pub use super::SpinnerExt;
//     pub use super::StackChildExt;
//     pub use super::StackExt;
//     pub use super::StyleExt;
//     pub use super::TableChildExt;
//     pub use super::TableExt;
//     pub use super::TextureCacheExt;
//     pub use super::ToggleExt;
//     pub use super::ToolbarExt;
//     pub use super::TooltipExt;
//     pub use super::ViewportExt;
//     pub use super::WidgetExt;
//     pub use super::WindowExt;
// }
