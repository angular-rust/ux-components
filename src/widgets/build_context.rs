// dependOnInheritedElement(InheritedElement ancestor, {Object aspect}) -> InheritedWidget
// Registers this build context with ancestor such that when ancestor's widget changes this build context is rebuilt.
//
// dependOnInheritedWidgetOfExactType<T extends InheritedWidget>({Object? aspect}) -> T?
// Obtains the nearest widget of the given type T, which must be the type of a concrete InheritedWidget subclass, and registers this build context with that widget such that when that widget changes (or a new widget of that type is introduced, or the widget goes away), this build context is rebuilt so that it can obtain new values from that widget.
//
// describeElement(String name, {DiagnosticsTreeStyle style = DiagnosticsTreeStyle.errorProperty}) -> DiagnosticsNode
// Returns a description of an Element from the current build context.
//
// describeMissingAncestor({required Type expectedAncestorType}) -> List<DiagnosticsNode>
// Adds a description of a specific type of widget missing from the current build context's ancestry tree.
//
// describeOwnershipChain(String name) -> DiagnosticsNode
// Adds a description of the ownership chain from a specific Element to the error report.
//
// describeWidget(String name, {DiagnosticsTreeStyle style = DiagnosticsTreeStyle.errorProperty}) -> DiagnosticsNode
// Returns a description of the Widget associated with the current build context.
//
// findAncestorRenderObjectOfType<T extends RenderObject>() -> T?
// Returns the RenderObject object of the nearest ancestor RenderObjectWidget widget that is an instance of the given type T.
//
// findAncestorStateOfType<T extends State<StatefulWidget>>() -> T?
// Returns the State object of the nearest ancestor StatefulWidget widget that is an instance of the given type T.
//
// findAncestorWidgetOfExactType<T extends Widget>() -> T?
// Returns the nearest ancestor widget of the given type T, which must be the type of a concrete Widget subclass.
//
// findRenderObject() -> RenderObject?
// The current RenderObject for the widget. If the widget is a RenderObjectWidget, this is the render object that the widget created for itself. Otherwise, it is the render object of the first descendant RenderObjectWidget.
//
// findRootAncestorStateOfType<T extends State<StatefulWidget>>() -> T?
// Returns the State object of the furthest ancestor StatefulWidget widget that is an instance of the given type T.
//
// getElementForInheritedWidgetOfExactType<T extends InheritedWidget>() -> InheritedElement?
// Obtains the element corresponding to the nearest widget of the given type T, which must be the type of a concrete InheritedWidget subclass.
//
// visitAncestorElements(bool visitor(Element element)) -> void
// Walks the ancestor chain, starting with the parent of this build context's widget, invoking the argument for each ancestor. The callback is given a reference to the ancestor widget's corresponding Element object. The walk stops when it reaches the root widget or when the callback returns false. The callback must not return null.
//
// visitChildElements(ElementVisitor visitor) -> void
// Walks the children of this widget.

pub struct BuildContext;
