use crate::{
    elements::ImageElement,
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    material::AlignmentGeometry,
    painting::ImageProvider,
    ui::{BlendMode, FilterQuality},
    widgets::{Element, Widget},
};

pub struct Image {
    pub key: Key,
    pub image: ImageProvider<String>,
    // pub frame_builder: ImageFrameBuilder,
    // pub loading_builder: ImageLoadingBuilder,
    // pub error_builder: ImageErrorWidgetBuilder,
    pub semantic_label: String,
    pub exclude_from_semantics: bool,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    // pub opacity: Animation<f32>,
    pub color_blend_mode: BlendMode,
    // pub fit: BoxFit,
    pub alignment: AlignmentGeometry,
    // pub repeat: ImageRepeat,
    // pub center_slice: crate::Rect<f32>,
    pub match_text_direction: bool,
    pub gapless_playback: bool,
    pub is_anti_alias: bool,
    pub filter_quality: FilterQuality,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            key: Default::default(),
            image: Default::default(),
            // frame_builder: Default::default(),
            // loading_builder: Default::default(),
            // error_builder: Default::default(),
            semantic_label: Default::default(),
            exclude_from_semantics: Default::default(),
            width: Default::default(),
            height: Default::default(),
            color: Default::default(),
            // opacity: Default::default(),
            color_blend_mode: Default::default(),
            // fit: Default::default(),
            alignment: Default::default(),
            // repeat: Default::default(),
            // center_slice: Default::default(),
            match_text_direction: Default::default(),
            gapless_playback: Default::default(),
            is_anti_alias: Default::default(),
            filter_quality: Default::default(),
        }
    }
}

impl Widget for Image {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create ImageElement");
        box ImageElement::new(self)
    }
}

impl WidgetProperties for Image {
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
