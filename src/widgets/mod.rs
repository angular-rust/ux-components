#![allow(unused_variables)]
#![allow(dead_code)]

use crate::{
    elements::{Element, NoneElement},
    ui::Size,
};

mod enums;
pub use self::enums::*;

// mod rich_text;

mod action;
pub use self::action::*;

mod align;
pub use self::align::*;

mod build_context;
pub use self::build_context::*;

mod center;
pub use self::center::*;

mod clip_oval;
pub use self::clip_oval::*;

mod clip_path;
pub use self::clip_path::*;

mod column;
pub use self::column::*;

mod container;
pub use self::container::*;

mod custom_paint;
pub use self::custom_paint::*;

mod custom_scroll_view;
pub use self::custom_scroll_view::*;

mod expanded;
pub use self::expanded::*;

mod fitted_box;
pub use self::fitted_box::*;

mod flexible;
pub use self::flexible::*;

mod flow;
pub use self::flow::*;

mod focus_node;
pub use self::focus_node::*;

mod form;
pub use self::form::*;

mod generate_app_title;
pub use self::generate_app_title::*;

mod gesture_detector;
pub use self::gesture_detector::*;

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

mod media_query_data;
pub use self::media_query_data::*;

mod media_query;
pub use self::media_query::*;

mod navigation_toolbar;
pub use self::navigation_toolbar::*;

mod navigator_observer;
pub use self::navigator_observer::*;

mod navigator_state;
pub use self::navigator_state::*;

mod navigator;
pub use self::navigator::*;

mod offstage;
pub use self::offstage::*;

mod opacity;
pub use self::opacity::*;

mod padding;
pub use self::padding::*;

mod page;
pub use self::page::*;

mod placeholder;
pub use self::placeholder::*;

mod pop_page_callback;
pub use self::pop_page_callback::*;

mod positioned;
pub use self::positioned::*;

mod preferred_size_widget;
pub use self::preferred_size_widget::*;

mod preferred_size;
pub use self::preferred_size::*;

mod rich_text;
pub use self::rich_text::*;

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

mod rust_logo;
pub use self::rust_logo::*;

mod safe_area;
pub use self::safe_area::*;

mod scroll_behavior;
pub use self::scroll_behavior::*;

mod scroll_controller;
pub use self::scroll_controller::*;

mod scroll_position;
pub use self::scroll_position::*;

mod shortcut_activator;
pub use self::shortcut_activator::*;

mod single_child_scroll_view;
pub use self::single_child_scroll_view::*;

mod sized_box;
pub use self::sized_box::*;

mod sliver_child_builder_delegate;
pub use self::sliver_child_builder_delegate::*;

mod sliver_child_delegate;
pub use self::sliver_child_delegate::*;

mod sliver_grid;
pub use self::sliver_grid::*;

mod sliver_list;
pub use self::sliver_list::*;

mod stack;
pub use self::stack::*;

mod state;
pub use self::state::*;

mod stateful_widget;
pub use self::stateful_widget::*;

mod stream_builder;
pub use self::stream_builder::*;

mod text_editing_controller;
pub use self::text_editing_controller::*;

mod text;
pub use self::text::*;

mod transform;
pub use self::transform::*;

mod transition_builder;
pub use self::transition_builder::*;

mod transition_delegate;
pub use self::transition_delegate::*;

mod widget_builder;
pub use self::widget_builder::*;

mod widgets_app;
pub use self::widgets_app::*;

mod wrap;
pub use self::wrap::*;


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

    // Check for null object, aka None
    fn is_none(&self) -> bool {
        false
    }
}


/// NoneWidget used only as default widget to simplify declarative sintax
#[derive(Default)]
pub struct NoneWidget;

impl PreferredSizeWidget for NoneWidget {
    fn preferred_size(&self) -> Size {
        Size(0.0, 0.0)
    }
}

impl Widget for NoneWidget {
    fn create_element(&self) -> Box<dyn Element> {
        box NoneElement::default()
    }

    fn is_none(&self) -> bool {
        true
    }    
}


// pub trait WidgetBuilder {
//     fn build(&self, context: Option<BuildContext>) -> &dyn Widget;
// }

// #[derive(Default)]
// pub struct NullWidgetBuilder;

// impl WidgetBuilder for NullWidgetBuilder {
//     fn build(&self, context: Option<BuildContext>) -> &dyn Widget {
//         todo!()
//     }
// }