use crate::{
    elements::{MediaQueryElement, Element},
    foundation::{Id, Key, WidgetProperties},
    widgets::Widget,
};

use super::{NoneWidget, MediaQueryData, BuildContext};

pub struct MediaQuery {
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,
    // Contains information about the current media.
    pub data: MediaQueryData,
    // The widget below this widget in the tree.
    pub child: Box<dyn Widget>,
}

impl MediaQuery {
// Creates a widget that provides MediaQueryData to its descendants.
// MediaQuery({Key? key, required MediaQueryData data, required Widget child})

// Creates a new MediaQuery that inherits from the ambient MediaQuery from the given context, but removes the specified padding.
// MediaQuery.removePadding({Key? key, required BuildContext context, bool removeLeft = false, bool removeTop = false, bool removeRight = false, bool removeBottom = false, required Widget child})

// Creates a new MediaQuery that inherits from the ambient MediaQuery from the given context, but removes the specified view insets.
// MediaQuery.removeViewInsets({Key? key, required BuildContext context, bool removeLeft = false, bool removeTop = false, bool removeRight = false, bool removeBottom = false, required Widget child})

// Creates a new MediaQuery that inherits from the ambient MediaQuery from the given context, but removes the specified view padding.
// MediaQuery.removeViewPadding({Key? key, required BuildContext context, bool removeLeft = false, bool removeTop = false, bool removeRight = false, bool removeBottom = false, required Widget child})


// createElement() → InheritedElement
// Inflates this configuration to a concrete instance.
// inherited
// debugDescribeChildren() → List<DiagnosticsNode>
// Returns a list of DiagnosticsNode objects describing this node's children.
// @protected, inherited
// debugFillProperties(DiagnosticPropertiesBuilder properties) → void
// Add additional properties associated with the node.
// override
// noSuchMethod(Invocation invocation) → dynamic
// Invoked when a non-existent method or property is accessed.
// inherited
// toDiagnosticsNode({String? name, DiagnosticsTreeStyle? style}) → DiagnosticsNode
// Returns a debug representation of the object that is used by debugging tools and by DiagnosticsNode.toStringDeep.
// inherited
// toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) → String
// A string representation of this object.
// inherited
// toStringDeep({String prefixLineOne = '', String? prefixOtherLines, DiagnosticLevel minLevel = DiagnosticLevel.debug}) → String
// Returns a string representation of this node and its descendants.
// inherited
// toStringShallow({String joiner = ', ', DiagnosticLevel minLevel = DiagnosticLevel.debug}) → String
// Returns a one-line detailed description of the object.
// inherited
// toStringShort() → String
// A short, textual description of this widget.
// inherited
// updateShouldNotify(covariant MediaQuery oldWidget) → bool
// Whether the framework should notify widgets that inherit from this widget.
// override

// Static methods

// Returns the boldText accessibility setting for the nearest MediaQuery ancestor, or false if no such ancestor exists. 
// boldTextOverride(BuildContext context) → bool

// Provides a MediaQuery which is built and updated using the latest WidgetsBinding.window values. 
// fromWindow({Key? key, required Widget child}) → Widget

// Returns highContrast for the nearest MediaQuery ancestor or false, if no such ancestor exists. 
// highContrastOf(BuildContext context) → bool

// The data from the closest instance of this class that encloses the given context, if any. 
// maybeOf(BuildContext context) → MediaQueryData?

    // The data from the closest instance of this class that encloses the given context. 
    pub fn of(context: BuildContext) -> MediaQueryData {
        todo!()
    }

// Returns platformBrightness for the nearest MediaQuery ancestor or Brightness.light, if no such ancestor exists. 
// platformBrightnessOf(BuildContext context) → Brightness

// Returns textScaleFactor for the nearest MediaQuery ancestor or 1.0, if no such ancestor exists. 
// textScaleFactorOf(BuildContext context) → double

}

impl Default for MediaQuery {
    fn default() -> Self {
        Self {
            key: Default::default(),
            data: Default::default(),
            child: box NoneWidget,
        }
    }
}

impl Widget for MediaQuery {
    fn create_element(&self) -> Box<dyn Element> {
        box MediaQueryElement::new(self)
    }
}

impl WidgetProperties for MediaQuery {
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
