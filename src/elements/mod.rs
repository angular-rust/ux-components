#![allow(unused_variables)]
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

mod about_list_tile;
pub use self::about_list_tile::*;

mod action_chip;
pub use self::action_chip::*;

mod alert_dialog;
pub use self::alert_dialog::*;

mod align;
pub use self::align::*;

mod app_bar;
pub use self::app_bar::*;

mod back_button;
pub use self::back_button::*;

mod bottom_app_bar;
pub use self::bottom_app_bar::*;

mod bottom_navigation_bar;
pub use self::bottom_navigation_bar::*;

mod bottom_sheet;
pub use self::bottom_sheet::*;

mod button_bar;
pub use self::button_bar::*;

mod button_style_button;
pub use self::button_style_button::*;

mod button;
pub use self::button::*;

mod canvas;
pub use self::canvas::*;

mod card;
pub use self::card::*;

mod center;
pub use self::center::*;

mod checkbox;
pub use self::checkbox::*;

mod chip;
pub use self::chip::*;

mod choice_chip;
pub use self::choice_chip::*;

mod circle_avatar;
pub use self::circle_avatar::*;

mod circular_progress_indicator;
pub use self::circular_progress_indicator::*;

mod clip_oval;
pub use self::clip_oval::*;

mod clip_path;
pub use self::clip_path::*;

mod close_button;
pub use self::close_button::*;

mod column;
pub use self::column::*;

mod container;
pub use self::container::*;

mod custom_paint;
pub use self::custom_paint::*;

mod custom_scroll_view;
pub use self::custom_scroll_view::*;

mod data_table;
pub use self::data_table::*;

mod divider;
pub use self::divider::*;

mod drawer;
pub use self::drawer::*;

mod dropdown_button;
pub use self::dropdown_button::*;

mod element;
pub use self::element::*;

mod elevated_button;
pub use self::elevated_button::*;

mod expanded;
pub use self::expanded::*;

mod expansion_panel;
pub use self::expansion_panel::*;

mod filter_chip;
pub use self::filter_chip::*;

mod fitted_box;
pub use self::fitted_box::*;

mod flat_button;
pub use self::flat_button::*;

mod flexible_space_bar;
pub use self::flexible_space_bar::*;

mod flexible;
pub use self::flexible::*;

mod floating_action_button;
pub use self::floating_action_button::*;

mod flow;
pub use self::flow::*;

mod form;
pub use self::form::*;

mod gesture_detector;
pub use self::gesture_detector::*;

mod grid_view;
pub use self::grid_view::*;

mod icon_button;
pub use self::icon_button::*;

mod icon;
pub use self::icon::*;

mod image_icon;
pub use self::image_icon::*;

mod image;
pub use self::image::*;

mod ink_well;
pub use self::ink_well::*;

mod ink;
pub use self::ink::*;

mod input_chip;
pub use self::input_chip::*;

mod label;
pub use self::label::*;

mod linear_progress_indicator;
pub use self::linear_progress_indicator::*;

mod list_tile;
pub use self::list_tile::*;

mod list_view;
pub use self::list_view::*;

mod list;
pub use self::list::*;

mod material_app;
pub use self::material_app::*;

mod material_button;
pub use self::material_button::*;

mod material;
pub use self::material::*;

mod media_query;
pub use self::media_query::*;

mod modal_bottom_sheet;
pub use self::modal_bottom_sheet::*;

mod navigation_rail_destination;
pub use self::navigation_rail_destination::*;

mod navigation_rail;
pub use self::navigation_rail::*;

mod navigation_toolbar;
pub use self::navigation_toolbar::*;

mod offstage;
pub use self::offstage::*;

mod opacity;
pub use self::opacity::*;

mod outline_button;
pub use self::outline_button::*;

mod outlined_button;
pub use self::outlined_button::*;

mod panel;
pub use self::panel::*;

mod placeholder;
pub use self::placeholder::*;

mod popup_menu_button;
pub use self::popup_menu_button::*;

mod positioned;
pub use self::positioned::*;

mod preferred_size;
pub use self::preferred_size::*;

mod progress;
pub use self::progress::*;

mod radio;
pub use self::radio::*;

mod raised_button;
pub use self::raised_button::*;

mod raw_chip;
pub use self::raw_chip::*;

mod raw_material_button;
pub use self::raw_material_button::*;

mod refresh_indicator;
pub use self::refresh_indicator::*;

mod rich_text;
pub use self::rich_text::*;

mod row;
pub use self::row::*;

mod rust_logo;
pub use self::rust_logo::*;

mod safe_area;
pub use self::safe_area::*;

mod scaffold;
pub use self::scaffold::*;

mod scroll_view;
pub use self::scroll_view::*;

mod scrollable;
pub use self::scrollable::*;

mod simple_dialog;
pub use self::simple_dialog::*;

mod single_child_scroll_view;
pub use self::single_child_scroll_view::*;

mod sized_box;
pub use self::sized_box::*;

mod slider;
pub use self::slider::*;

mod sliver_app_bar;
pub use self::sliver_app_bar::*;

mod sliver_grid;
pub use self::sliver_grid::*;

mod sliver_list;
pub use self::sliver_list::*;

mod snack_bar;
pub use self::snack_bar::*;

mod stack;
pub use self::stack::*;

mod step;
pub use self::step::*;

mod stepper;
pub use self::stepper::*;

mod switch;
pub use self::switch::*;

mod tab_bar_view;
pub use self::tab_bar_view::*;

mod tab_bar;
pub use self::tab_bar::*;

mod tab_page_selector;
pub use self::tab_page_selector::*;

mod text_button;
pub use self::text_button::*;

mod text_field;
pub use self::text_field::*;

mod text;
pub use self::text::*;

mod textedit;
pub use self::textedit::*;

mod theme;
pub use self::theme::*;

mod toggle_buttons;
pub use self::toggle_buttons::*;

mod transform;
pub use self::transform::*;

mod user_accounts_drawer_header;
pub use self::user_accounts_drawer_header::*;

mod vertical_divider;
pub use self::vertical_divider::*;

mod widget_component;
pub use self::widget_component::*;

mod widgets_app;
pub use self::widgets_app::*;

mod window;
pub use self::window::*;

mod wrap;
pub use self::wrap::*;



#[derive(Default)]
pub struct NoneElement {
    pub component: Rc<RefCell<WidgetComponent>>,
}

impl Element for NoneElement {
    // Empty impl to exclude default behavior
    fn render(&self) {}
}

impl AsRef<RefCell<WidgetComponent>> for NoneElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}