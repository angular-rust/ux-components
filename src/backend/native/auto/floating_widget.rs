// use glib::translate::*;

use std::fmt;
// use Widget;

// glib_wrapper! {
//     pub struct FloatingWidget(Object<ffi::FloatingWidget, ffi::FloatingWidgetClass, FloatingWidgetClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::floating_widget_get_type(),
//     }
// }

#[derive(Clone, Debug)]
pub struct FloatingWidget {

}

impl FloatingWidget {}

pub const NONE_FLOATING_WIDGET: Option<&FloatingWidget> = None;

impl fmt::Display for FloatingWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FloatingWidget")
    }
}
