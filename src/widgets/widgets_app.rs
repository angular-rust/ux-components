#![allow(unused_imports)]
use std::collections::HashMap;

use crate::{
    elements::{Element, WidgetsAppElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    material::LocalizationsDelegate,
    painting::TextStyle,
    ui::Locale,
};

use super::{
    Action, GenerateAppTitle, GlobalKey, InitialRouteListFactory, Intent,
    LocaleListResolutionCallback, LocaleResolutionCallback, NavigatorObserver, NavigatorState,
    NullWidget, RouteFactory, ShortcutActivator, TransitionBuilder, Widget, WidgetBuilder,
};

pub struct WidgetsApp {
    pub key: Key,
    pub navigator_key: GlobalKey<NavigatorState>,
    pub on_generate_route: Option<Box<dyn RouteFactory<String>>>,
    pub on_generate_initial_routes: Option<Box<dyn InitialRouteListFactory>>,
    pub on_unknown_route: Option<Box<dyn RouteFactory<String>>>,
    pub navigator_observers: Vec<NavigatorObserver>,
    pub initial_route: String,
    // pub page_route_builder: PageRouteFactory,
    pub home: Box<dyn Widget>,
    pub routes: HashMap<String, Box<dyn WidgetBuilder>>,
    pub builder: Option<Box<dyn TransitionBuilder>>,
    pub title: String,
    pub on_generate_title: Option<Box<dyn GenerateAppTitle>>,
    pub text_style: TextStyle,
    pub color: Color,
    pub locale: Locale,
    pub localizations_delegates: Vec<LocalizationsDelegate>,
    pub locale_list_resolution_callback: Option<Box<dyn LocaleListResolutionCallback>>,
    pub locale_resolution_callback: Option<Box<dyn LocaleResolutionCallback>>,
    pub supported_locales: Vec<Locale>,
    pub show_performance_overlay: bool,
    pub checkerboard_raster_cache_images: bool,
    pub checkerboard_offscreen_layers: bool,
    pub show_semantics_debugger: bool,
    pub debug_show_widget_inspector: bool,
    pub debug_show_checked_mode_banner: bool,
    // pub inspector_select_button_builder: InspectorSelectButtonBuilder,
    pub shortcuts: HashMap<ShortcutActivator, Box<dyn Intent>>,
    // pub actions: HashMap<Type, Action<Intent>>,
    pub restoration_scope_id: String,
    pub use_inherited_media_query: bool,
}

impl Default for WidgetsApp {
    fn default() -> Self {
        Self {
            key: Default::default(),
            navigator_key: Default::default(),
            on_generate_route: Default::default(),
            on_generate_initial_routes: Default::default(),
            on_unknown_route: Default::default(),
            navigator_observers: Default::default(),
            initial_route: Default::default(),
            // page_route_builder: Default::default(),
            home: box NullWidget,
            routes: Default::default(),
            builder: Default::default(),
            title: Default::default(),
            on_generate_title: Default::default(),
            text_style: Default::default(),
            color: Default::default(),
            locale: Default::default(),
            localizations_delegates: Default::default(),
            locale_list_resolution_callback: Default::default(),
            locale_resolution_callback: Default::default(),
            supported_locales: Default::default(),
            show_performance_overlay: Default::default(),
            checkerboard_raster_cache_images: Default::default(),
            checkerboard_offscreen_layers: Default::default(),
            show_semantics_debugger: Default::default(),
            debug_show_widget_inspector: Default::default(),
            debug_show_checked_mode_banner: Default::default(),
            // inspector_select_button_builder: Default::default(),
            shortcuts: Default::default(),
            // actions: Default::default(),
            restoration_scope_id: Default::default(),
            use_inherited_media_query: Default::default(),
        }
    }
}

impl Widget for WidgetsApp {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create WidgetsAppElement");
        box WidgetsAppElement::new(self)
    }
}

impl WidgetProperties for WidgetsApp {
    fn key(&self) -> &Key {
        &self.key
    }

    fn x(&self) -> f32 {
        // self.x
        0.0
    }

    fn y(&self) -> f32 {
        // self.y
        0.0
    }

    fn w(&self) -> f32 {
        // self.w
        0.0
    }

    fn h(&self) -> f32 {
        // self.h
        0.0
    }

    fn w_min(&self) -> f32 {
        // self.w_min
        0.0
    }

    fn h_min(&self) -> f32 {
        // self.h_min
        0.0
    }

    fn w_max(&self) -> f32 {
        // self.w_max
        0.0
    }

    fn h_max(&self) -> f32 {
        // self.h_max
        0.0
    }

    fn parent(&self) -> Option<Id> {
        // self.parent
        None
    }

    fn depth(&self) -> f32 {
        // self.depth
        0.0
    }

    fn visible(&self) -> bool {
        // self.visible
        true
    }

    fn mouse_input(&self) -> bool {
        // self.mouse_input
        true
    }

    fn key_input(&self) -> bool {
        // self.key_input
        true
    }

    fn renderable(&self) -> bool {
        // self.renderable
        true
    }

    fn internal_visible(&self) -> bool {
        // self.internal_visible
        true
    }
}
