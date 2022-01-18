#![allow(unused_variables)]
#![allow(dead_code)]

// mod rich_text;

mod action;
pub use self::action::*;

mod align;
pub use self::align::*;

mod build_context;
pub use self::build_context::*;

mod center;
pub use self::center::*;

mod column;
pub use self::column::*;

mod container;
pub use self::container::*;

mod expanded;
pub use self::expanded::*;

mod flow;
pub use self::flow::*;

mod focus_node;
pub use self::focus_node::*;

mod generate_app_title;
pub use self::generate_app_title::*;

mod global_key;
pub use self::global_key::*;

mod grid_view;
pub use self::grid_view::*;

mod icon_data;
pub use self::icon_data::*;

mod icon_theme_data;
pub use self::icon_theme_data::*;

mod icon;
pub use self::icon::*;

mod image_icon;
pub use self::image_icon::*;

mod image;
pub use self::image::*;

mod initial_route_list_factory;
pub use self::initial_route_list_factory::*;

mod intent;
pub use self::intent::*;

mod list_view;
pub use self::list_view::*;

mod locale_list_resolution_callback;
pub use self::locale_list_resolution_callback::*;

mod locale_resolution_callback;
pub use self::locale_resolution_callback::*;

mod navigation_toolbar;
pub use self::navigation_toolbar::*;

mod navigator_observer;
pub use self::navigator_observer::*;

mod navigator_state;
pub use self::navigator_state::*;

mod navigator;
pub use self::navigator::*;

mod null_widget;
pub use self::null_widget::*;

mod page;
pub use self::page::*;

mod placeholder;
pub use self::placeholder::*;

mod pop_page_callback;
pub use self::pop_page_callback::*;

mod preferred_size_widget;
pub use self::preferred_size_widget::*;

mod route_factory;
pub use self::route_factory::*;

mod route_list_factory;
pub use self::route_list_factory::*;

mod route_settings;
pub use self::route_settings::*;

mod route;
pub use self::route::*;

mod row;
pub use self::row::*;

mod scroll_behavior;
pub use self::scroll_behavior::*;

mod shortcut_activator;
pub use self::shortcut_activator::*;

mod stack;
pub use self::stack::*;

mod state;
pub use self::state::*;

mod stateful_widget;
pub use self::stateful_widget::*;

mod text;
pub use self::text::*;

mod transition_builder;
pub use self::transition_builder::*;

mod transition_delegate;
pub use self::transition_delegate::*;

mod widget_builder;
pub use self::widget_builder::*;

mod widgets_app;
pub use self::widgets_app::*;

use crate::elements::Element;

// Inherited from DiagnosticableTree
pub trait Widget {
    // @factory, @protected
    /// Inflates this configuration to a concrete instance
    fn create_element(&self) -> Box<dyn Element>;

    // // override DiagnosticableTree
    // /// Add additional properties associated with the node.
    // fn debug_fill_properties(properties: DiagnosticPropertiesBuilder);

    // // override DiagnosticableTree
    // /// A short, textual description of this widget.
    // fn to_string_short();
}
