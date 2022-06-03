use crate::{foundation::LocalKey, painting::EdgeInsetsGeometry};

use super::{BuildContext, Widget};

// createRenderObject(BuildContext context) → RenderPadding
// Creates an instance of the RenderObject class that this RenderObjectWidget represents, using the configuration described by this RenderObjectWidget.
// override

// updateRenderObject(BuildContext context, covariant RenderPadding renderObject) → void
// Copies the configuration described by this RenderObjectWidget to the given RenderObject, which will be of the same type as returned by this object's createRenderObject.
// override

// located in rendering
pub struct RenderObject {
    // alwaysNeedsCompositing → bool
// Whether this render object always needs compositing.
// @protected, read-only
// attached → bool
// Whether this node is in a tree whose root is attached to something.
// read-only, inherited
// constraints → Constraints
// The layout constraints most recently supplied by the parent.
// @protected, read-only
// debugCanParentUseSize → bool
// Whether the parent render object is permitted to use this render object's size.
// read-only
// debugCreator ↔ Object?
// The object responsible for creating this render object.
// read / write
// debugDisposed → bool?
// Whether this has been disposed.
// read-only
// debugDoingThisLayout → bool
// Whether performLayout for this render object is currently running.
// read-only
// debugDoingThisLayoutWithCallback → bool
// Whether invokeLayoutCallback for this render object is currently running.
// read-only
// debugDoingThisPaint → bool
// Whether paint for this render object is currently running.
// read-only
// debugDoingThisResize → bool
// Whether performResize for this render object is currently running.
// read-only
// debugLayer → ContainerLayer?
// In debug mode, the compositing layer that this render object uses to repaint.
// read-only
// debugNeedsLayout → bool
// Whether this render object's layout information is dirty.
// read-only
// debugNeedsPaint → bool
// Whether this render object's paint information is dirty.
// read-only
// debugSemantics → SemanticsNode?
// The semantics of this render object.
// read-only
// depth → int
// The depth of this node in the tree.
// read-only, inherited
// hashCode → int
// The hash code for this object.
// read-only, inherited
// isRepaintBoundary → bool
// Whether this render object repaints separately from its parent.
// read-only
// layer ↔ ContainerLayer?
// The compositing layer that this render object uses to repaint.
// @protected, @protected, read / write
// needsCompositing → bool
// Whether we or one of our descendants has a compositing layer.
// read-only
// owner → PipelineOwner?
// The owner for this node (null if unattached).
// read-only, override
// paintBounds → Rect
// An estimate of the bounds within which this render object will paint. Useful for debugging flags such as debugPaintLayerBordersEnabled.
// read-only
// parent → AbstractNode?
// The parent of this node in the tree.
// read-only, inherited
// parentData ↔ ParentData?
// Data for use by the parent render object.
// read / write
// runtimeType → Type
// A representation of the runtime type of the object.
// read-only, inherited
// semanticBounds → Rect
// The bounding box, in the local coordinate system, of this object, for accessibility purposes.
// read-only
// sizedByParent → bool
// Whether the constraints are the only input to the sizing algorithm (in particular, child nodes have no impact).
// @protected, read-only
}

impl RenderObject {
    // adoptChild(covariant RenderObject child) → void
    // Called by subclasses when they decide a render object is a child.
    // override
    // applyPaintTransform(covariant RenderObject child, Matrix4 transform) → void
    // Applies the transform that would be applied when painting the given child to the given matrix.
    // assembleSemanticsNode(SemanticsNode node, SemanticsConfiguration config, Iterable<SemanticsNode> children) → void
    // Assemble the SemanticsNode for this RenderObject.
    // attach(covariant PipelineOwner owner) → void
    // Mark this node as attached to the given owner.
    // override
    // clearSemantics() → void
    // Removes all semantics from this render object and its descendants.
    // @mustCallSuper
    // debugAssertDoesMeetConstraints() → void
    // Verify that the object's constraints are being met. Override this function in a subclass to verify that your state matches the constraints object. This function is only called in checked mode and only when needsLayout is false. If the constraints are not met, it should assert or throw an exception.
    // @protected
    // debugDescribeChildren() → List<DiagnosticsNode>
    // Returns a list of DiagnosticsNode objects describing this node's children.
    // override
    // debugFillProperties(DiagnosticPropertiesBuilder properties) → void
    // Add additional properties associated with the node.
    // @protected, override
    // debugPaint(PaintingContext context, Offset offset) → void
    // Override this method to paint debugging information.
    // debugRegisterRepaintBoundaryPaint({bool includedParent = true, bool includedChild = false}) → void
    // Called, in debug mode, if isRepaintBoundary is true, when either the this render object or its parent attempt to paint.
    // debugResetSize() → void
    // If a subclass has a "size" (the state controlled by parentUsesSize, whatever it is in the subclass, e.g. the actual size property of RenderBox), and the subclass verifies that in debug mode this "size" property isn't used when debugCanParentUseSize isn't set, then that subclass should override debugResetSize to reapply the current values of debugCanParentUseSize to that state.
    // @protected
    // describeApproximatePaintClip(covariant RenderObject child) → Rect?
    // Returns a rect in this object's coordinate system that describes the approximate bounding box of the clip rect that would be applied to the given child during the paint phase, if any.
    // describeForError(String name, {DiagnosticsTreeStyle style = DiagnosticsTreeStyle.shallow}) → DiagnosticsNode
    // Adds a debug representation of a RenderObject optimized for including in error messages.
    // describeSemanticsClip(covariant RenderObject? child) → Rect?
    // Returns a rect in this object's coordinate system that describes which SemanticsNodes produced by the child should be included in the semantics tree. SemanticsNodes from the child that are positioned outside of this rect will be dropped. Child SemanticsNodes that are positioned inside this rect, but outside of describeApproximatePaintClip will be included in the tree marked as hidden. Child SemanticsNodes that are inside of both rect will be included in the tree as regular nodes.
    // describeSemanticsConfiguration(SemanticsConfiguration config) → void
    // Report the semantics of this node, for example for accessibility purposes.
    // @protected
    // detach() → void
    // Mark this node as detached.
    // @mustCallSuper, inherited
    // dispose() → void
    // Release any resources held by this render object.
    // @mustCallSuper
    // dropChild(covariant RenderObject child) → void
    // Called by subclasses when they decide a render object is no longer a child.
    // override
    // getTransformTo(RenderObject? ancestor) → Matrix4
    // Applies the paint transform up the tree to ancestor.
    // handleEvent(PointerEvent event, covariant HitTestEntry entry) → void
    // Override this method to handle pointer events that hit this render object.
    // override
    // invokeLayoutCallback<T extends Constraints>(LayoutCallback<T> callback) → void
    // Allows mutations to be made to this object's child list (and any descendants) as well as to any other dirty nodes in the render tree owned by the same PipelineOwner as this object. The callback argument is invoked synchronously, and the mutations are allowed only during that callback's execution.
    // @protected
    // layout(Constraints constraints, {bool parentUsesSize = false}) → void
    // Compute the layout for this render object.
    // markNeedsCompositingBitsUpdate() → void
    // Mark the compositing state for this render object as dirty.
    // markNeedsLayout() → void
    // Mark this render object's layout information as dirty, and either register this object with its PipelineOwner, or defer to the parent, depending on whether this object is a relayout boundary or not respectively.
    // markNeedsLayoutForSizedByParentChange() → void
    // Mark this render object's layout information as dirty (like markNeedsLayout), and additionally also handle any necessary work to handle the case where sizedByParent has changed value.
    // markNeedsPaint() → void
    // Mark this render object as having changed its visual appearance.
    // markNeedsSemanticsUpdate() → void
    // Mark this node as needing an update to its semantics description.
    // markParentNeedsLayout() → void
    // Mark this render object's layout information as dirty, and then defer to the parent.
    // @protected
    // noSuchMethod(Invocation invocation) → dynamic
    // Invoked when a non-existent method or property is accessed.
    // inherited
    // paint(PaintingContext context, Offset offset) → void
    // Paint this render object into the given context at the given offset.
    // performLayout() → void
    // Do the work of computing the layout for this render object.
    // @protected
    // performResize() → void
    // Updates the render objects size using only the constraints.
    // @protected
    // reassemble() → void
    // Cause the entire subtree rooted at the given RenderObject to be marked dirty for layout, paint, etc, so that the effects of a hot reload can be seen, or so that the effect of changing a global debug flag (such as debugPaintSizeEnabled) can be applied.
    // redepthChild(AbstractNode child) → void
    // Adjust the depth of the given child to be greater than this node's own depth.
    // @protected, inherited
    // redepthChildren() → void
    // Adjust the depth of this node's children, if any.
    // inherited
    // replaceRootLayer(OffsetLayer rootLayer) → void
    // Replace the layer. This is only valid for the root of a render object subtree (whatever object scheduleInitialPaint was called on).
    // rotate({int? oldAngle, int? newAngle, Duration? time}) → void
    // Rotate this render object (not yet implemented).
    // scheduleInitialLayout() → void
    // Bootstrap the rendering pipeline by scheduling the very first layout.
    // scheduleInitialPaint(ContainerLayer rootLayer) → void
    // Bootstrap the rendering pipeline by scheduling the very first paint.
    // scheduleInitialSemantics() → void
    // Bootstrap the semantics reporting mechanism by marking this node as needing a semantics update.
    // sendSemanticsEvent(SemanticsEvent semanticsEvent) → void
    // Sends a SemanticsEvent associated with this render object's SemanticsNode.
    // setupParentData(covariant RenderObject child) → void
    // Override to setup parent data correctly for your children.
    // showOnScreen({RenderObject? descendant, Rect? rect, Duration duration = Duration.zero, Curve curve = Curves.ease}) → void
    // Attempt to make (a portion of) this or a descendant RenderObject visible on screen.
    // toDiagnosticsNode({String? name, DiagnosticsTreeStyle? style}) → DiagnosticsNode
    // Returns a debug representation of the object that is used by debugging tools and by DiagnosticsNode.toStringDeep.
    // inherited
    // toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) → String
    // A string representation of this object.
    // override
    // toStringDeep({String prefixLineOne = '', String? prefixOtherLines = '', DiagnosticLevel minLevel = DiagnosticLevel.debug}) → String
    // Returns a description of the tree rooted at this node. If the prefix argument is provided, then every line in the output will be prefixed by that string.
    // override
    // toStringShallow({String joiner = ', ', DiagnosticLevel minLevel = DiagnosticLevel.debug}) → String
    // Returns a one-line detailed description of the render object. This description is often somewhat long.
    // override
    // toStringShort() → String
    // Returns a human understandable name.
    // override
    // visitChildren(RenderObjectVisitor visitor) → void
    // Calls visitor for each immediate child of this render object.
    // visitChildrenForSemantics(RenderObjectVisitor visitor) → void
    // Called when collecting the semantics of this node.
}

pub struct RenderObjectElement {}

pub struct SingleChildRenderObjectElement {
    // widget: SingleChildRenderObjectWidget
}

pub trait RenderObjectWidget: Widget {
    // key → Key?

    // RenderObjectWidgets always inflate to a RenderObjectElement subclass.
    fn create_element(&self) -> RenderObjectElement;

    // Creates an instance of the RenderObject class that this RenderObjectWidget represents,
    // using the configuration described by this RenderObjectWidget.
    fn create_render_object(context: BuildContext) -> RenderObject;

    // A render object previously associated with this widget has been removed from the tree.
    // The given RenderObject will be of the same type as returned by this object's createRenderObject.
    fn did_unmount_render_object(render_object: RenderObject);

    // Copies the configuration described by this RenderObjectWidget to the given RenderObject,
    // which will be of the same type as returned by this object's createRenderObject.
    fn update_render_object(context: BuildContext, render_object: RenderObject);
}

pub trait SingleChildRenderObjectWidget: RenderObjectWidget {
    // child → Widget?
    // key → Key?

    // RenderObjectWidgets always inflate to a RenderObjectElement subclass.
    fn create_element(&self) -> SingleChildRenderObjectElement;
}

// Widget > RenderObjectWidget > SingleChildRenderObjectWidget
pub struct Padding {
    key: LocalKey,
    padding: Box<dyn EdgeInsetsGeometry>,
    child: Box<dyn Widget>,
}
