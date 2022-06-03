#![allow(unused_imports)]
use std::{any::TypeId, collections::HashMap};

use crate::{
    elements::{Element, MaterialAppElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    ui::Locale,
    widgets::{
        Action, GenerateAppTitle, GlobalKey, InitialRouteListFactory, Intent,
        LocaleListResolutionCallback, LocaleResolutionCallback, NavigatorObserver, NavigatorState,
        NoneWidget, RouteFactory, ScrollBehavior, ShortcutActivator, TransitionBuilder, Widget,
        WidgetBuilder,
    },
};

use super::{LocalizationsDelegate, ScaffoldMessengerState, ThemeData, ThemeMode};

pub struct MaterialApp {
    pub key: Key,
    pub navigator_key: GlobalKey<NavigatorState>,
    pub scaffold_messenger_key: GlobalKey<ScaffoldMessengerState>,
    pub home: Box<dyn Widget>,
    pub routes: HashMap<String, Box<WidgetBuilder>>,
    pub initial_route: String,
    pub on_generate_route: Option<Box<dyn RouteFactory<String>>>, // TODO: String generic should be fixed
    pub on_generate_initial_routes: Option<Box<dyn InitialRouteListFactory>>,
    pub on_unknown_route: Option<Box<dyn RouteFactory<String>>>, // TODO: String generic should be fixed
    pub navigator_observers: Vec<NavigatorObserver>,
    pub builder: Option<Box<dyn TransitionBuilder>>,
    pub title: String,
    pub on_generate_title: Option<Box<dyn GenerateAppTitle>>,
    pub color: Color,
    pub theme: ThemeData,
    pub dark_theme: ThemeData,
    pub high_contrast_theme: ThemeData,
    pub high_contrast_dark_theme: ThemeData,
    pub theme_mode: ThemeMode,
    pub locale: Locale,
    pub localizations_delegates: Vec<LocalizationsDelegate>,
    pub locale_list_resolution_callback: Option<Box<dyn LocaleListResolutionCallback>>,
    pub locale_resolution_callback: Option<Box<dyn LocaleResolutionCallback>>,
    pub supported_locales: Vec<Locale>,
    pub debug_show_material_grid: bool,
    pub show_performance_overlay: bool,
    pub checkerboard_raster_cache_images: bool,
    pub checkerboard_offscreen_layers: bool,
    pub show_semantics_debugger: bool,
    pub debug_show_checked_mode_banner: bool,
    pub shortcuts: HashMap<ShortcutActivator, Box<dyn Intent>>,
    // pub actions: HashMap<TypeId, Action<Box<dyn Intent>>>,
    pub restoration_scope_id: String,
    pub scroll_behavior: ScrollBehavior,
    pub use_inherited_media_query: bool,
}

impl Default for MaterialApp {
    fn default() -> Self {
        Self {
            key: Default::default(),
            navigator_key: Default::default(),
            scaffold_messenger_key: Default::default(),
            home: box NoneWidget,
            routes: Default::default(),
            initial_route: Default::default(),
            on_generate_route: Default::default(),
            on_generate_initial_routes: Default::default(),
            on_unknown_route: Default::default(),
            navigator_observers: Default::default(),
            builder: Default::default(),
            title: Default::default(),
            on_generate_title: Default::default(),
            color: Default::default(),
            theme: Default::default(),
            dark_theme: Default::default(),
            high_contrast_theme: Default::default(),
            high_contrast_dark_theme: Default::default(),
            theme_mode: Default::default(),
            locale: Default::default(),
            localizations_delegates: Default::default(),
            locale_list_resolution_callback: Default::default(),
            locale_resolution_callback: Default::default(),
            supported_locales: Default::default(),
            debug_show_material_grid: Default::default(),
            show_performance_overlay: Default::default(),
            checkerboard_raster_cache_images: Default::default(),
            checkerboard_offscreen_layers: Default::default(),
            show_semantics_debugger: Default::default(),
            debug_show_checked_mode_banner: Default::default(),
            shortcuts: Default::default(),
            restoration_scope_id: Default::default(),
            scroll_behavior: Default::default(),
            use_inherited_media_query: Default::default(),
        }
    }
}

impl Widget for MaterialApp {
    fn create_element(&self) -> Box<dyn Element> {
        box MaterialAppElement::new(self)
    }
}

impl WidgetProperties for MaterialApp {
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
