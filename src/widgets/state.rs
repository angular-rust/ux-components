use crate::widgets::Widget;

use super::{BuildContext, StatefulWidget};

// activate() -> void
// Called when this object is reinserted into the tree after having been removed via deactivate.
// @mustCallSuper, @protected
//
// build(BuildContext context) -> Widget
// Describes the part of the user interface represented by this widget.
// @protected
//
// deactivate() -> void
// Called when this object is removed from the tree.
// @mustCallSuper, @protected
//
// debugFillProperties(DiagnosticPropertiesBuilder properties) -> void
// Add additional properties associated with the node.
// override
//
// didChangeDependencies() -> void
// Called when a dependency of this State object changes.
// @mustCallSuper, @protected
//
// didUpdateWidget(covariant T oldWidget) -> void
// Called whenever the widget configuration changes.
// @mustCallSuper, @protected
//
// dispose() -> void
// Called when this object is removed from the tree permanently.
// @mustCallSuper, @protected
//
// initState() -> void
// Called when this object is inserted into the tree.
//
// reassemble() -> void
// Called whenever the application is reassembled during debugging, for example during hot reload.
// @mustCallSuper, @protected
//
// setState(VoidCallback fn) -> void
// Notify the framework that the internal state of this object has changed.

// pub struct State<T: StatefulWidget>(T);

pub trait State<T: StatefulWidget> {
    fn build(&self, context: Option<BuildContext>) -> &dyn Widget;
}
