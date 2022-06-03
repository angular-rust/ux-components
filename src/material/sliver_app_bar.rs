use crate::{
    elements::{SliverAppBarElement, Element},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::{NoneShapeBorder, ShapeBorder, TextStyle},
    services::SystemUiOverlayStyle,
    ui::{Brightness, Size},
    widgets::{IconThemeData, NoneWidget, PreferredSizeWidget, Widget},
};

use super::TextTheme;

pub struct SliverAppBar {
    // A list of Widgets to display in a row after the title widget.
    pub actions: Vec<Box<dyn Widget>>,
    
    // The color, opacity, and size to use for the icons that appear in the app bar's actions.
    pub actions_icon_theme: IconThemeData,
    
    // Controls whether we should try to imply the leading widget if null.
    pub automatically_imply_leading: bool,
    
    // The fill color to use for an app bar's Material.
    pub background_color: Color,
    
    // @Deprecated('This property is obsolete and is false by default. ' 'This feature was deprecated after v2.4.0-0.0.pre.'), final
    // This property is deprecated and is false by default.
    pub backwards_compatibility: bool,
    
    
    // This widget appears across the bottom of the app bar.
    pub bottom: Box<dyn PreferredSizeWidget>,
    
    // @Deprecated('This property is no longer used, please use systemOverlayStyle instead. ' 'This feature was deprecated after v2.4.0-0.0.pre.'), final
    // This property is deprecated, please use systemOverlayStyle instead.
    pub brightness: Brightness,
    
    // Whether the title should be centered.
    pub center_title: bool,
    
    // Defines the height of the app bar when it is collapsed.
    pub collapsed_height: f32,
    
    // The z-coordinate at which to place this app bar relative to its parent.
    pub elevation: f32,
    
    // Whether the title should be wrapped with header Semantics.
    pub exclude_header_semantics: bool,
    
    // The size of the app bar when it is fully expanded.
    pub expanded_height: f32,
    
    // This widget is stacked behind the toolbar and the tab bar. Its height will be the same as the app bar's overall height.
    pub flexible_space: Box<dyn Widget>,
    
    // Whether the app bar should become visible as soon as the user scrolls towards the app bar.
    pub floating: bool,
    
    // Whether to show the shadow appropriate for the elevation even if the content is not scrolled under the AppBar.
    pub force_elevated: bool,
    
    // The default color for Text and Icons within the app bar.
    pub foreground_color: Color,
    
    // The color, opacity, and size to use for toolbar icons.
    pub icon_theme: IconThemeData,
    
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,
    
    // A widget to display before the toolbar's title.
    pub leading: Box<dyn Widget>,
    
    // Defines the width of leading widget.
    pub leading_width: f32,
    
    // The callback function to be executed when a user over-scrolls to the offset specified by stretchTriggerOffset.
    // pub on_stretch_trigger: AsyncCallback,
    
    // Whether the app bar should remain visible at the start of the scroll view.
    pub pinned: bool,
    
    // Whether this app bar is being displayed at the top of the screen.
    pub primary: bool,
    
    // The color of the shadow below the app bar.
    pub shadow_color: Color,
    
    // The shape of the app bar's Material as well as its shadow.
    pub shape: Box<dyn ShapeBorder>,
    
    // If snap and floating are true then the floating app bar will "snap" into view.
    pub snap: bool,
    
    // Whether the app bar should stretch to fill the over-scroll area.
    pub stretch: bool,
    
    // The offset of overscroll required to activate onStretchTrigger.
    pub stretch_trigger_offset: f32,
    
    // Specifies the style to use for the system overlays that overlap the AppBar.
    pub system_overlay_style: SystemUiOverlayStyle,
    
    // This property is deprecated, please use toolbarTextStyle and titleTextStyle instead.
    // @Deprecated('This property is no longer used, please use toolbarTextStyle and titleTextStyle instead. ' 'This feature was deprecated after v2.4.0-0.0.pre.'), final
    pub text_theme: TextTheme,
    
    // The primary widget displayed in the app bar.
    pub title: Box<dyn Widget>,
    
    // The spacing around title content on the horizontal axis. This spacing is applied even if there is no leading content or actions. 
    // If you want title to take all the space available, set this value to 0.0.
    pub title_spacing: f32,
    
    // The default text style for the AppBar's title widget.
    pub title_text_style: TextStyle,
    
    // Defines the height of the toolbar component of an AppBar.
    pub toolbar_height: f32,
    
    // The default text style for the AppBar's leading, and actions widgets, but not its title.
    pub toolbar_text_style: TextStyle,
}

impl Default for SliverAppBar {
    fn default() -> Self {
        Self {
            actions: Default::default(),
            actions_icon_theme: Default::default(),
            automatically_imply_leading: Default::default(),
            background_color: Default::default(),
            backwards_compatibility: Default::default(),
            bottom: box NoneWidget,
            brightness: Default::default(),
            center_title: Default::default(),
            collapsed_height: Default::default(),
            elevation: Default::default(),
            exclude_header_semantics: Default::default(),
            expanded_height: Default::default(),
            flexible_space: box NoneWidget,
            floating: Default::default(),
            force_elevated: Default::default(),
            foreground_color: Default::default(),
            icon_theme: Default::default(),
            key: Default::default(),
            leading: box NoneWidget,
            leading_width: Default::default(),
            // on_stretch_trigger: Default::default(),
            pinned: Default::default(),
            primary: Default::default(),
            shadow_color: Default::default(),
            shape: box NoneShapeBorder,
            snap: Default::default(),
            stretch: Default::default(),
            stretch_trigger_offset: Default::default(),
            system_overlay_style: Default::default(),
            text_theme: Default::default(),
            title: box NoneWidget,
            title_spacing: Default::default(),
            title_text_style: Default::default(),
            toolbar_height: Default::default(),
            toolbar_text_style: Default::default(),            
        }
    }
}

impl PreferredSizeWidget for SliverAppBar {
    fn preferred_size(&self) -> Size {
        Size(0.0, self.toolbar_height) // +  bottom widget's preferred height
    }
}

impl Widget for SliverAppBar {
    fn create_element(&self) -> Box<dyn Element> {
        box SliverAppBarElement::new(self)
    }
}

impl WidgetProperties for SliverAppBar {
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
