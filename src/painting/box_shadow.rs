use crate::prelude::Color;

use crate::ui::{BlurStyle, Offset};

//inherited ui::Shadow
pub struct BoxShadow {
    // ui::Shadow {
    // The standard deviation of the Gaussian to convolve with the shadow's shape.
    pub blur_radius: f32,
    
    // The blurRadius in sigmas instead of logical pixels.
    pub blur_sigma: f32,
    
    // Color that the shadow will be drawn with.
    pub color: Color,
    
    // The displacement of the shadow from the casting element.
    pub offset: Offset,
    // }

    // The BlurStyle to use for this shadow.
    pub blur_style: BlurStyle,
    
    // The amount the box should be inflated prior to applying the blur. 
    pub spread_radius: f32,
}

// impl Shadow for BoxShadow {
//     scale(f32 factor) → Shadow
//     Returns a new shadow with its offset and blurRadius scaled by the given factor. 
//     toPaint() → Paint
//     Create the Paint object that corresponds to this shadow description. 
// }



impl BoxShadow {

}