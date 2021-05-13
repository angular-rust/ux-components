use crate::theme::{Fill, StyleDefinition};
use animate::color;

pub fn mdc_button() -> StyleDefinition {
    StyleDefinition {
        background: Some(Fill::Solid(color::GRAY_9)),
        color: Some(color::WHITE),
        // height
        // padding
        fontfamily: Some("Roboto".into()),
        fontsize: Some(12.0),
        fontweight: Some("Roboto".into()),
        border_radius: Some(4.0),
    }
}
