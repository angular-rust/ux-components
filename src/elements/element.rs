#![allow(unused_mut)]
use cgmath::Point2;
use std::cell::RefCell;
use stretch::node::Node;

use crate::prelude::OnDemand;

use crate::{
    foundation::{ChildBounds, Helper, Id, KeyEvent, MouseEvent, Slot, TextEvent, WidgetClipEvent},
    widgets::Widget,
};

use super::WidgetComponent;

pub trait ElementVisitor {
    fn visit_child_elements(&self, element: &dyn Element);
}

/// Inherited from Element
pub trait ComponentElement: Element {
    // @protected
    /// Subclasses should override this function to actually call the
    /// appropriate build function (e.g., StatelessWidget.build or State.build)
    /// for their widget.
    // not a WidgetBuilder
    fn build(&self) -> &dyn Widget;

    // override
    /// Remove the given child from the element's child list, in preparation for the
    /// child being reused elsewhere in the element tree.
    fn forget_child(&self, child: &dyn Element);

    // override
    /// Add this element to the tree in the given slot of the given parent.
    fn mount(&self, parent: Option<&dyn Element>, new_slot: Option<Slot>);

    // override
    /// Calls the StatelessWidget.build method of the StatelessWidget object
    /// (for stateless widgets) or the State.build method of the State object
    /// (for stateful widgets) and then updates the widget tree.
    fn perform_rebuild();

    // override
    /// Calls the argument for each child. Must be overridden by subclasses
    /// that support having children.
    fn visit_children(&self, visitor: &dyn ElementVisitor);
}

/// Inherited from Element
pub trait RenderObjectElement: Element {
    // // override
    // /// Add renderObject to the render tree at the location specified by newSlot.
    // fn attach_render_object(&self, new_slot: Slot);

    // // override
    // /// Transition from the "active" to the "inactive" lifecycle state.
    // fn deactivate(&self);

    // // override
    // /// Add additional properties associated with the node.
    // fn debugFillProperties(properties: DiagnosticPropertiesBuilder);

    // // override
    // /// Remove renderObject from the render tree.
    // fn detachRenderObject();

    // // @protected
    // /// Insert the given child into renderObject at the given slot.
    // fn insertRenderObjectChild(child: RenderObject, slot: Option<Slot>);

    // // override
    // /// Add this element to the tree in the given slot of the given parent.
    // fn mount(parent: Option<dyn Element>, new_slot: Option<Slot>);

    // // @protected
    // /// Move the given child from the given old slot to the given new slot.
    // fn moveRenderObjectChild(child: RenderObject, oldSlot: Option<Slot>, newSlot: Option<Slot>);

    // // override
    // /// Called by rebuild() after the appropriate checks have been made.
    // fn performRebuild();

    // // @protected
    // // Remove the given child from renderObject.
    // fn removeRenderObjectChild(child: RenderObject, slot: Option<Slot>);

    // // override
    // /// Transition from the "inactive" to the "defunct" lifecycle state.
    // fn unmount();

    // // override
    // /// Change the widget used to configure this element.
    // fn update(newWidget: RenderObjectWidget);

    // // @protected
    // /// Updates the children of this element to use new widgets.
    // fn updateChildren(
    //     oldChildren: Vec<Element>,
    //     newWidgets: Vec<Widget>,
    //     forgottenChildren: Option<HashSet<Element>>,
    //     slots: Option<Vec<Object>>,
    // ) -> Vec<Element>;
}

// activate() -> void
// Transition from the "inactive" to the "active" lifecycle state.
// @mustCallSuper
//
// attachRenderObject(Object? newSlot) -> void
// Add renderObject to the render tree at the location specified by newSlot.
//
// deactivate() -> void
// Transition from the "active" to the "inactive" lifecycle state.
// @mustCallSuper
//
// deactivateChild(Element child) -> void
// Move the given element to the list of inactive elements and detach its render object from the render tree.
// @protected
//
// debugDeactivated() -> void
// Called, in debug mode, after children have been deactivated (see deactivate).
// @mustCallSuper
//
// debugDescribeChildren() -> List<DiagnosticsNode>
// Returns a list of DiagnosticsNode objects describing this node's children.
// override
//
// debugFillProperties(DiagnosticPropertiesBuilder properties) -> void
// Add additional properties associated with the node.
// override
//
// debugGetCreatorChain(int limit) -> String
// Returns a description of what caused this element to be created.
//
// debugGetDiagnosticChain() -> List<Element>
// Returns the parent chain from this element back to the root of the tree.
//
// debugVisitOnstageChildren(ElementVisitor visitor) -> void
// Calls the argument for each child considered onstage.
//
// dependOnInheritedElement(InheritedElement ancestor, {Object? aspect}) -> InheritedWidget
// Registers this build context with ancestor such that when ancestor's widget changes this build context is rebuilt.
// override
//
// dependOnInheritedWidgetOfExactType<T extends InheritedWidget>({Object? aspect}) -> T?
// Obtains the nearest widget of the given type T, which must be the type of a concrete InheritedWidget subclass, and registers this build context with that widget such that when that widget changes (or a new widget of that type is introduced, or the widget goes away), this build context is rebuilt so that it can obtain new values from that widget.
// override
//
// describeElement(String name, {DiagnosticsTreeStyle style = DiagnosticsTreeStyle.errorProperty}) -> DiagnosticsNode
// Returns a description of an Element from the current build context.
// override
//
// describeMissingAncestor({required Type expectedAncestorType}) -> List<DiagnosticsNode>
// Adds a description of a specific type of widget missing from the current build context's ancestry tree.
// override
//
// describeOwnershipChain(String name) -> DiagnosticsNode
// Adds a description of the ownership chain from a specific Element to the error report.
// override
//
// describeWidget(String name, {DiagnosticsTreeStyle style = DiagnosticsTreeStyle.errorProperty}) -> DiagnosticsNode
// Returns a description of the Widget associated with the current build context.
// override
//
// detachRenderObject() -> void
// Remove renderObject from the render tree.
//
// didChangeDependencies() -> void
// Called when a dependency of this element changes.
// @mustCallSuper
//
// findAncestorRenderObjectOfType<T extends RenderObject>() -> T?
// Returns the RenderObject object of the nearest ancestor RenderObjectWidget widget that is an instance of the given type T.
// override
//
// findAncestorStateOfType<T extends State<StatefulWidget>>() -> T?
// Returns the State object of the nearest ancestor StatefulWidget widget that is an instance of the given type T.
// override
//
// findAncestorWidgetOfExactType<T extends Widget>() -> T?
// Returns the nearest ancestor widget of the given type T, which must be the type of a concrete Widget subclass.
// override
//
// findRenderObject() -> RenderObject?
// The current RenderObject for the widget. If the widget is a RenderObjectWidget, this is the render object that the widget created for itself. Otherwise, it is the render object of the first descendant RenderObjectWidget.
// override
//
// findRootAncestorStateOfType<T extends State<StatefulWidget>>() -> T?
// Returns the State object of the furthest ancestor StatefulWidget widget that is an instance of the given type T.
// override
//
// forgetChild(Element child) -> void
// Remove the given child from the element's child list, in preparation for the child being reused elsewhere in the element tree.
// @mustCallSuper, @protected
// getElementForInheritedWidgetOfExactType<T extends InheritedWidget>() -> InheritedElement?
// Obtains the element corresponding to the nearest widget of the given type T, which must be the type of a concrete InheritedWidget subclass.
// override
//
// inflateWidget(Widget newWidget, Object? newSlot) -> Element
// Create an element for the given widget and add it as a child of this element in the given slot.
// @protected
//
// markNeedsBuild() -> void
// Marks the element as dirty and adds it to the global list of widgets to rebuild in the next frame.
//
// mount(Element? parent, Object? newSlot) -> void
// Add this element to the tree in the given slot of the given parent.
// @mustCallSuper
//
// performRebuild() -> void
// Called by rebuild() after the appropriate checks have been made.
// @protected
//
// reassemble() -> void
// Called whenever the application is reassembled during debugging, for example during hot reload.
// @mustCallSuper, @protected
//
// rebuild() -> void
// Called by the BuildOwner when BuildOwner.scheduleBuildFor has been called to mark this element dirty, by mount when the element is first built, and by update when the widget has changed.
//
// toDiagnosticsNode({String? name, DiagnosticsTreeStyle? style}) -> DiagnosticsNode
// Returns a debug representation of the object that is used by debugging tools and by DiagnosticsNode.toStringDeep.
//
// toStringShort() -> String
// A short, textual description of this element.
// override
//
// unmount() -> void
// Transition from the "inactive" to the "defunct" lifecycle state.
// @mustCallSuper
//
// update(covariant Widget newWidget) -> void
// Change the widget used to configure this element.
// @mustCallSuper
//
// updateChild(Element? child, Widget? newWidget, Object? newSlot) -> Element?
// Update the given child with the given new configuration.
// @protected
//
// updateSlotForChild(Element child, Object? newSlot) -> void
// Change the slot that the given child occupies in its parent.
// @protected
//
// visitAncestorElements(bool visitor(Element element)) -> void
// Walks the ancestor chain, starting with the parent of this build context's widget, invoking the argument for each ancestor. The callback is given a reference to the ancestor widget's corresponding Element object. The walk stops when it reaches the root widget or when the callback returns false. The callback must not return null.
// override
//
// visitChildElements(ElementVisitor visitor) -> void
// Wrapper around visitChildren for BuildContext.
// override
//
// visitChildren(ElementVisitor visitor) -> void
// Calls the argument for each child. Must be overridden by subclasses that support having children.

/// Implementers: [ComponentElement], [RenderObjectElement]
pub trait Element
where
    Self: AsRef<RefCell<WidgetComponent>>,
{
    // ?_into:Vec<WidgetComponent>
    fn children_at_point(
        &self,
        x: f32,
        y: f32,
        into: Option<Vec<WidgetComponent>>,
    ) -> Vec<WidgetComponent> {
        // let comp = self.as_ref().borrow();

        // assert!(!comp.destroyed, "Widget was already destroyed but is being interacted with");

        // let result = into.unwrap_or_default();

        // if comp.children.len() == 0 {
        //     return result;
        // }

        // for child in comp.children.iter() {
        //     if let Some(widget) = child.widget() {
        //         if widget.contains(x, y) && widget.visible() {
        //             result.push(child);
        //             if widget.children().len() > 0 {
        //                 return widget.children_at_point(x, y, Some(result));
        //             }
        //         }
        //     }
        // }

        // _result.sort(|a, b| {
        //     if a.depth == b.depth {
        //         return 0;
        //     }
        //
        //     if a.depth < b.depth {
        //         return -1;
        //     }
        //     return 1;
        // }); // DV

        // return result;

        todo!()
    }

    fn topmost_child_at_point(&self, x: f32, y: f32) -> Option<&Box<dyn Element>> {
        let comp = self.as_ref().borrow();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        // //if we have no children, we are the topmost child
        // if comp.children.len() == 0 {
        //     return Some(comp);
        // }

        // //if we have children, we look at each one, looking for the highest one
        // //after we have the highest one, we ask it to return it"s own highest child

        // let highest_child = comp;
        // let highest_depth = 0.0;

        // for child in comp.children.iter() {
        //     if let Some(widget) = child.widget() {
        //         if widget.visible() && widget.contains(x, y) {
        //             if widget.depth() >= highest_depth {
        //                 highest_child = *child;
        //                 highest_depth = widget.depth();
        //             } //highest_depth
        //         } //child contains point
        //     }
        // }

        // if highest_child != comp.id() && highest_child.children.len() != 0 {
        //     if let Some(widget) = highest_child.widget() {
        //         if widget.children.len() != 0 {
        //             widget.topmost_child_at_point(x, y)
        //         } else {
        //             highest_child
        //         }
        //     }
        // } else {
        //     highest_child
        // }
        todo!()
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        let comp = self.as_ref().borrow();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        let inside = Helper::in_rect(x, y, comp.x, comp.y, comp.w, comp.h);

        if comp.clip.is_none() {
            return inside;
        }

        // if inside {
        //     if let Some(clip) = comp.clip {
        //         if let Some(widget) = clip.widget() {
        //             return widget.contains(x, y);
        //         }
        //     }
        // }
        // false
        inside
    }

    fn onclipchanged(&self) {
        let comp = self.as_ref().borrow();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        // if let Some(clip) = comp.clip {
        //     if let Some(widget) = clip.widget() {
        //         comp.onclip.emit(&WidgetClipEvent {
        //             clipped: false,
        //             h: widget.h(),
        //             w: widget.w(),
        //             x: widget.x(),
        //             y: widget.y(),
        //         });
        //     }
        // }
    }

    /// The control this one is clipped by
    fn clip(&self) -> Option<Id> {
        let comp = self.as_ref().borrow();

        comp.clip
    }

    /// The control this one is clipped by
    fn set_clip(&self, other: Option<Id>) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        // if let Some(clip) = comp.clip {
        //     if let Some(widget) = clip.widget() {
        //         // widget.onbounds.remove(box || self.onclipchanged());
        //         todo!()
        //     }
        // }

        comp.clip = other;

        if let Some(clip) = comp.clip {
            // if let Some(widget) = clip.widget() {
            //     // widget.onbounds.listen(box |e| self.onclipchanged());
            //     todo!()
            // }

            // // TODO: clip children applies to children
            // for child in comp.children.iter() {
            //     if let Some(widget) = child.widget() {
            //         widget.set_clip(comp.clip);
            //     }
            // }

            // self.onclipchanged();
        } else if comp.onclip.is_some() {
            let _ = comp.onclip.get().try_send(WidgetClipEvent {
                clipped: true,
                h: 0.0,
                w: 0.0,
                x: 0.0,
                y: 0.0,
            });
        }
    }

    #[inline]
    fn set_visible_only(&self, visible: bool) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        comp.update_vis_state = false;
        comp.visible = visible;
        comp.update_vis_state = true;
    }

    /// If the control is visible
    fn visible(&self) -> bool {
        let comp = self.as_ref().borrow();

        comp.visible
    }

    /// If the control is visible
    fn vis_state(&self) -> bool {
        let comp = self.as_ref().borrow();

        comp.vis_state
    }

    /// If the control is visible
    fn set_visible(&self, visible: bool) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        comp.visible = visible;
        if comp.update_vis_state {
            comp.vis_state = visible;
        }

        if comp.onvisible.is_some() {
            let visible = comp.visible;
            let _ = comp.onvisible.get().try_send(visible);
        }

        // for child in comp.children.iter() {
        //     if let Some(widget) = child.widget() {
        //         widget.set_visible_only(comp.visible && widget.vis_state());
        //     }
        // }

        if let Some(widget) = comp.canvas.as_mut() {
            widget.focus_invalid = true;
        }
    }

    // ?_from:WidgetComponent = None
    fn find_top_parent(&self, from: Option<&WidgetComponent>) -> Option<WidgetComponent> {
        let comp = self.as_ref().borrow();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        // let target = from.unwrap_or(comp);

        // match target.parent {
        //     Some(parent) => {
        //         // // if the parent of the target is not canvas,
        //         // // keep escalating until it is
        //         // if Std.is(target.parent, Canvas) {
        //         //     return target;
        //         // } else {
        //         //     //is
        //         //     return comp.parent.find_top_parent(self);
        //         // }
        //         todo!()
        //     }
        //     None => None,
        // }
        todo!()
    }

    fn add(&self, child: &dyn Element) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if let Some(parent) = child.parent() {
            parent.remove(child.id());

            if parent.id() != self.id() {
                comp.children.push(child.id());
                child.set_parent(Some(self.id()));
                if comp.onchildadd.is_some() {
                    let _ = comp.onchildadd.get().try_send(child.id());
                }
            }

            if let Some(widget) = comp.canvas.as_ref() {
                widget.sync_depth();
            }
        }
    }

    /// The child must be sure that his parent reference is correct.
    /// The child must remove the parent reference by himself: child.set_parent(None);
    fn remove(&self, child: Id) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if let Some(index) = comp.children.iter().position(|x| x == &child) {
            comp.children.remove(index);

            if comp.onchildremove.is_some() {
                let _ = comp.onchildremove.get().try_send(child);
            }

            if let Some(widget) = comp.canvas.as_ref() {
                widget.sync_depth();
            }
        }
    }

    fn children_bounds(&self) -> ChildBounds {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if comp.children.is_empty() {
            comp.children_bounds.x = 0.0;
            comp.children_bounds.y = 0.0;
            comp.children_bounds.w = 0.0;
            comp.children_bounds.h = 0.0;
            comp.children_bounds.x_local = 0.0;
            comp.children_bounds.y_local = 0.0;

            return comp.children_bounds;
        } //no children

        let mut current_x: f32 = 0.0;
        let mut current_y: f32 = 0.0;
        let mut current_r: f32 = 0.0;
        let mut current_b: f32 = 0.0;

        let mut real_x: f32 = 0.0;
        let mut real_y: f32 = 0.0;

        // for child in comp.children.iter() {
        //     if let Some(widget) = child.widget() {
        //         current_x = widget.x_local().min(current_x);
        //         current_y = widget.y_local().min(current_y);
        //         current_r = current_r.max(widget.x_local() + widget.w());
        //         current_b = current_b.max(widget.y_local() + widget.h());

        //         real_x = widget.x().min(real_x);
        //         real_y = widget.y().min(real_y);
        //     }
        // } //child in children

        comp.children_bounds.x_local = current_x;
        comp.children_bounds.y_local = current_y;

        comp.children_bounds.x = real_x;
        comp.children_bounds.y = real_y;
        comp.children_bounds.w = current_r;
        comp.children_bounds.h = current_b;

        comp.children_bounds
    }

    fn render(&self) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if comp.renderable && comp.onrender.is_some() {
            let _ = comp.onrender.get().try_send(());
        }

        // for child in comp.children.iter() {
        //     if let Some(widget) = child.widget() {
        //         widget.render();
        //     }
        // }
    }

    fn keyup(&self, e: &mut KeyEvent) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if comp.onkeyup.is_some() {
            let _ = comp.onkeyup.get().try_send(e.clone());
        }

        // if e.bubble {
        //     if let Some(parent) = comp.parent {
        //         if let Some(widget) = parent.widget() {
        //             if let Some(canvas) = comp.canvas.as_ref() {
        //                 if canvas.id() != widget.id() && canvas.id() != self.id() {
        //                     widget.keyup(e);
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn keydown(&self, e: &mut KeyEvent) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if comp.onkeydown.is_some() {
            let _ = comp.onkeydown.get().try_send(e.clone());
        }

        // if e.bubble {
        //     if let Some(parent) = comp.parent {
        //         if let Some(widget) = parent.widget() {
        //             if let Some(canvas) = comp.canvas.as_ref() {
        //                 if canvas.id() != widget.id() && canvas.id() != self.id() {
        //                     widget.keydown(e);
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn textinput(&self, e: &mut TextEvent) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if comp.ontextinput.is_some() {
            let _ = comp.ontextinput.get().try_send(e.clone());
        }

        // if e.bubble {
        //     if let Some(parent) = comp.parent {
        //         if let Some(widget) = parent.widget() {
        //             if let Some(canvas) = comp.canvas.as_ref() {
        //                 if canvas.id() != widget.id() && canvas.id() != self.id() {
        //                     widget.textinput(e);
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn mousemove(&self, e: &mut MouseEvent) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if comp.onmousemove.is_some() {
            let _ = comp.onmousemove.get().try_send(e.clone());
        }

        // if e.bubble {
        //     if let Some(parent) = comp.parent {
        //         if let Some(widget) = parent.widget() {
        //             if let Some(canvas) = comp.canvas.as_ref() {
        //                 if canvas.id() != widget.id() && canvas.id() != self.id() {
        //                     widget.mousemove(e);
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn mouseup(&self, e: &mut MouseEvent) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if comp.onmouseup.is_some() {
            let _ = comp.onmouseup.get().try_send(e.clone());
        }

        // if e.bubble {
        //     if let Some(parent) = comp.parent {
        //         if let Some(widget) = parent.widget() {
        //             if let Some(canvas) = comp.canvas.as_ref() {
        //                 if canvas.id() != widget.id() && canvas.id() != self.id() {
        //                     widget.mouseup(e);
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn mousewheel(&self, e: &mut MouseEvent) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if comp.onmousewheel.is_some() {
            let _ = comp.onmousewheel.get().try_send(e.clone());
        }

        // if e.bubble {
        //     if let Some(parent) = comp.parent {
        //         if let Some(widget) = parent.widget() {
        //             if let Some(canvas) = comp.canvas.as_ref() {
        //                 if canvas.id() != widget.id() && canvas.id() != self.id() {
        //                     widget.mousewheel(e);
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn mousedown(&self, e: &mut MouseEvent) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if comp.onmousedown.is_some() {
            let _ = comp.onmousedown.get().try_send(e.clone());
        }

        // if e.bubble {
        //     if let Some(parent) = comp.parent {
        //         if let Some(widget) = parent.widget() {
        //             if let Some(canvas) = comp.canvas.as_ref() {
        //                 if canvas.id() != widget.id() && canvas.id() != self.id() {
        //                     widget.mousedown(e);
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn mouseenter(&self, e: &mut MouseEvent) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if comp.onmouseenter.is_some() {
            let _ = comp.onmouseenter.get().try_send(e.clone());
        }
        comp.ishovered = true;
    }

    fn mouseleave(&self, e: &mut MouseEvent) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if comp.onmouseleave.is_some() {
            let _ = comp.onmouseleave.get().try_send(e.clone());
        }
        comp.ishovered = false;
    }

    fn destroy_children(&self) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        while let Some(child) = comp.children.pop() {
            // if let Some(widget) = child.widget() {
            //     widget.destroy();
            // }
        }
    }

    fn destroy(&self) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "attempt to destroy control twice `$self` ($name)"
        );

        self.unmark();
        self.unfocus();
        self.uncapture();

        self.destroy_children();

        if let Some(clip) = comp.clip {
            // clip.widget().map(|x| x.onbounds.remove(self.onclipchanged()));
            todo!()
        }

        if let Some(parent) = comp.parent {
            // if let Some(widget) = parent.widget() {
            //     widget.remove(self.id());
            //     self.set_parent(None);
            // }
        }

        if comp.ondestroy.is_some() {
            let _ = comp.ondestroy.get().try_send(());
        }

        comp.user = None;

        // comp.oncreate.clear();
        // comp.onrender.clear();
        // comp.onbounds.clear();
        // comp.ondestroy.clear();
        // comp.onvisible.clear();
        // comp.ondepth.clear();
        // comp.onclip.clear();
        // comp.onchildadd.clear();
        // comp.onchildremove.clear();
        // comp.onmousedown.clear();
        // comp.onmouseup.clear();
        // comp.onmousemove.clear();
        // comp.onmousewheel.clear();
        // comp.onmouseleave.clear();
        // comp.onmouseenter.clear();
        // comp.onkeydown.clear();
        // comp.onkeyup.clear();
        // comp.ontextinput.clear();
        // comp.onfocused.clear();
        // comp.onmarked.clear();
        // comp.oncaptured.clear();

        comp.destroyed = true;
    }

    fn update(&self, dt: f32) {
        log::info!("Update Default Element Impl");
        let comp = self.as_ref().borrow();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );
    }

    fn focus(&self) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if let Some(canvas) = comp.canvas.as_ref() {
            if canvas.id() == self.id() {
                return;
            }
        }

        let pre = if let Some(canvas) = comp.canvas.as_ref() {
            canvas.focused == Some(self.id())
        } else {
            false
        };

        if let Some(canvas) = comp.canvas.as_mut() {
            canvas.focused = Some(self.id());
        }

        if !pre && comp.onfocused.is_some() {
            let _ = comp.onfocused.get().try_send(true);
        }
    }

    fn unfocus(&self) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if let Some(canvas) = comp.canvas.as_ref() {
            if canvas.id() == self.id() {
                return;
            }
        }

        if let Some(canvas) = comp.canvas.as_mut() {
            if let Some(focused) = canvas.focused {
                if focused == self.id() {
                    canvas.focused = None;

                    if comp.onfocused.is_some() {
                        let _ = comp.onfocused.get().try_send(false);
                    }
                }
            }
        }
    }

    fn capture(&self) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if let Some(canvas) = comp.canvas.as_ref() {
            if canvas.id() == self.id() {
                return;
            }
        }

        let pre = if let Some(canvas) = comp.canvas.as_ref() {
            if let Some(captured) = canvas.captured {
                captured == self.id()
            } else {
                false
            }
        } else {
            false
        };

        if let Some(canvas) = comp.canvas.as_mut() {
            canvas.captured = Some(self.id());
        }

        if !pre && comp.oncaptured.is_some() {
            let _ = comp.oncaptured.get().try_send(true);
        }
    }

    fn uncapture(&self) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if let Some(canvas) = comp.canvas.as_mut() {
            if canvas.id() == self.id() {
                return;
            }

            if let Some(captured) = canvas.captured {
                if captured == self.id() {
                    canvas.captured = None;

                    if comp.oncaptured.is_some() {
                        let _ = comp.oncaptured.get().try_send(false);
                    }
                }
            }
        }
    }

    fn mark(&self) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if let Some(canvas) = comp.canvas.as_ref() {
            if canvas.id() == self.id() {
                return;
            }
        }

        let pre = if let Some(ref canvas) = comp.canvas.as_ref() {
            if let Some(marked) = canvas.marked {
                marked == self.id()
            } else {
                false
            }
        } else {
            false
        };

        if let Some(ref mut canvas) = comp.canvas.as_mut() {
            canvas.marked = Some(self.id());
        }

        if !pre && comp.onmarked.is_some() {
            let _ = comp.onmarked.get().try_send(true);
        }
    }

    fn unmark(&self) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if let Some(canvas) = comp.canvas.as_mut() {
            if canvas.id() == self.id() {
                return;
            }

            if let Some(marked) = canvas.marked {
                if marked == self.id() {
                    canvas.marked = None;

                    if comp.onmarked.is_some() {
                        let _ = comp.onmarked.get().try_send(false);
                    }
                }
            }
        }
    }

    fn refresh_bounds(&self) {
        let mut comp = self.as_ref().borrow_mut();

        if comp.onbounds.is_some() {
            let _ = comp.onbounds.get().try_send(());
        }

        // for child in comp.children.iter() {
        //     if let Some(widget) = child.widget() {
        //         widget.refresh_bounds();
        //     }
        // }
    }

    // _dx: f32 =0.0, _dy: f32 =0.0, _dw: f32 =0.0, _dh: f32 =0.0
    fn bounds_changed(&self, dx: f32, dy: f32, dw: f32, dh: f32) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        if comp.updating {
            return;
        }

        // manual relayout of childs
        // we dont need it more coz we use flexbox
        // if dx != 0.0 || dy != 0.0 {
        //     for child in comp.children.iter() {
        //         if let Some(widget) = child.widget() {
        //             widget.set_pos(widget.x() + dx, widget.y() + dy);
        //         }
        //     }
        // }

        if comp.onbounds.is_some() {
            let _ = comp.onbounds.get().try_send(());
        }
    }

    //Properties

    //Spatial properties

    fn set_pos(&self, x: f32, y: f32) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        comp.updating = true;

        let dx = x - comp.x;
        let dy = y - comp.y;

        comp.x = x;
        comp.y = y;

        comp.updating = false;

        self.bounds_changed(dx, dy, 0.0, 0.0);
    }

    fn set_size(&self, w: f32, h: f32) {
        log::info!("Set Size Default Impl {}x{}", w, h);

        let (dw, dh) = {
            let mut comp = self.as_ref().borrow_mut();

            assert!(
                !comp.destroyed,
                "Widget was already destroyed but is being interacted with"
            );

            comp.updating = true;

            let dw = w - comp.w;
            let dh = h - comp.h;

            comp.w = w;
            comp.h = h;

            comp.updating = false;

            (dw, dh)
        };

        self.bounds_changed(0.0, 0.0, dw, dh);
    }

    #[inline]
    fn destroyed(&self) -> bool {
        let comp = self.as_ref().borrow();

        comp.destroyed
    }

    #[inline]
    fn right(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.x + comp.w
    }

    #[inline]
    fn bottom(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.y + comp.h
    }

    /// The x position of the control bounds, world coordinate
    fn x(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.x
    }

    /// The x position of the control bounds, world coordinate
    fn set_x(&self, x: f32) {
        let mut comp = self.as_ref().borrow_mut();

        let dx = x - self.x();

        comp.x = x;

        if !comp.ignore_spatial {
            comp.ignore_spatial = true;
            match comp.parent {
                Some(parent) => {
                    // if let Some(widget) = parent.widget() {
                    //     comp.x_local = comp.x - widget.x(); // TODO: direct or with setter
                    // }
                }
                None => {
                    comp.x_local = comp.x; // TODO: direct or with setter
                }
            }
            comp.ignore_spatial = false;
        }

        self.bounds_changed(dx, 0.0, 0.0, 0.0);
    }

    /// The y position of the control bounds, world coordinate
    fn y(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.y
    }

    /// The y position of the control bounds, world coordinate
    fn set_y(&self, y: f32) {
        let mut comp = self.as_ref().borrow_mut();

        let dy = y - comp.y;

        comp.y = y;

        if !comp.ignore_spatial {
            comp.ignore_spatial = true;
            match comp.parent {
                Some(parent) => {
                    // if let Some(widget) = parent.widget() {
                    //     comp.y_local = comp.y - widget.y(); // TODO: direct or with setter
                    // }
                }
                None => {
                    comp.y_local = comp.y; // TODO: direct or with setter
                }
            }

            comp.ignore_spatial = false;
        }

        self.bounds_changed(0.0, dy, 0.0, 0.0);
    }

    /// The minimum width
    fn w_min(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.w_min
    }

    /// The minimum width
    fn set_w_min(&self, w_min: f32) {
        let mut comp = self.as_ref().borrow_mut();

        comp.w_min = w_min;

        if comp.w < comp.w_min {
            comp.w = comp.w_min;
        }
    }

    /// The minimum height
    fn h_min(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.h_min
    }

    /// The minimum height
    fn set_h_min(&self, h_min: f32) {
        let mut comp = self.as_ref().borrow_mut();

        comp.h_min = h_min;

        if comp.h < comp.h_min {
            comp.h = comp.h_min;
        }
    }

    /// The maximum width
    fn w_max(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.w_max
    }

    /// The maximum width
    fn set_w_max(&self, w_max: f32) {
        let mut comp = self.as_ref().borrow_mut();

        comp.w_max = w_max;

        if comp.w > comp.w_max {
            comp.w = comp.w_max;
        }
    }

    /// The maximum height
    fn h_max(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.h_max
    }

    /// The maximum height
    fn set_h_max(&self, _h_max: f32) {
        let mut comp = self.as_ref().borrow_mut();

        comp.h_max = _h_max;

        if comp.h > comp.h_max {
            comp.h = comp.h_max;
        }
    }

    /// The width of the control bounds
    fn w(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.w
    }

    /// The width of the control bounds
    fn set_w(&self, w: f32) {
        // log::info!("Set Width Deault Impl {}", w);
        let dw = {
            let mut comp = self.as_ref().borrow_mut();

            let mut w = if w < comp.w_min { comp.w_min } else { w };

            w = if w > comp.w_max && comp.w_max != 0.0 {
                comp.w_max
            } else {
                w
            };

            let dw = w - comp.w;

            comp.w = w;

            dw
        };

        self.bounds_changed(0.0, 0.0, dw, 0.0);
    }

    /// The height of the control bounds
    fn h(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.h
    }

    /// The height of the control bounds
    fn set_h(&self, h: f32) {
        // log::info!("Set Height Deault Impl {}", h);
        let dh = {
            let mut comp = self.as_ref().borrow_mut();

            let mut h = if h < comp.h_min { comp.h_min } else { h };

            h = if h > comp.h_max && comp.h_max != 0.0 {
                comp.h_max
            } else {
                h
            };

            let dh = h - comp.h;

            comp.h = h;

            dh
        };

        self.bounds_changed(0.0, 0.0, 0.0, dh);
    }

    /// The x position of the control bounds, relative to its container
    fn set_x_local(&self, x: f32) {
        let mut comp = self.as_ref().borrow_mut();

        comp.x_local = x;

        if !comp.ignore_spatial {
            comp.ignore_spatial = true;
            match comp.parent {
                Some(parent) => {
                    // if let Some(widget) = parent.widget() {
                    //     comp.x = widget.x() + comp.x_local;
                    // }
                }
                None => {
                    comp.x = comp.x_local;
                }
            }

            comp.ignore_spatial = false;
        }
    }

    /// The y position of the control bounds, relative to its container
    fn set_y_local(&self, y: f32) {
        let mut comp = self.as_ref().borrow_mut();

        comp.y_local = y;

        if !comp.ignore_spatial {
            comp.ignore_spatial = true;
            match comp.parent {
                Some(parent) => {
                    // if let Some(widget) = parent.widget() {
                    //     comp.y = widget.y() + comp.y_local;
                    // }
                }
                None => {
                    comp.y = comp.y_local;
                }
            }

            comp.ignore_spatial = false;
        }
    }

    /// The x position of the control bounds, relative to its container
    fn x_local(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.x_local
    }

    /// The y position of the control bounds, relative to its container
    fn y_local(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.y_local
    }

    //Node properties

    #[inline]
    fn nodes(&self) -> i32 {
        let comp = self.as_ref().borrow();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        let mut nodes = 1;
        // for child in comp.children.iter() {
        //     if let Some(widget) = child.widget() {
        //         nodes += widget.nodes();
        //     }
        // }
        nodes
    }

    #[inline]
    fn is_focused(&self) -> bool {
        let comp = self.as_ref().borrow();
        comp.isfocused
    }

    fn set_focused(&self, focused: bool) {
        let mut comp = self.as_ref().borrow_mut();

        comp.isfocused = focused;
    }

    #[inline]
    fn is_captured(&self) -> bool {
        let comp = self.as_ref().borrow();
        comp.iscaptured
    }

    fn set_captured(&self, captured: bool) {
        let mut comp = self.as_ref().borrow_mut();
        comp.iscaptured = captured;
    }

    #[inline]
    fn is_marked(&self) -> bool {
        let comp = self.as_ref().borrow();
        comp.ismarked
    }

    fn set_marked(&self, marked: bool) {
        let mut comp = self.as_ref().borrow_mut();
        comp.ismarked = marked
    }

    //Depth properties

    #[inline]
    fn depth_offset(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.depth_offset
    }

    //the depth of this control
    #[inline]
    fn depth(&self) -> f32 {
        let comp = self.as_ref().borrow();

        comp.depth
    }

    //the depth of this control
    fn set_depth(&self, depth: f32) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        comp.depth = depth;

        if comp.ondepth.is_some() {
            let depth = comp.depth;
            let _ = comp.ondepth.get().try_send(depth);
        }
    }

    #[inline]
    fn mouse_input(&self) -> bool {
        let comp = self.as_ref().borrow();
        comp.mouse_input
    }

    #[inline]
    fn key_input(&self) -> bool {
        let comp = self.as_ref().borrow();
        comp.key_input
    }

    //Parent properties

    #[inline]
    //the parent control, None if no parent
    fn parent(&self) -> Option<&dyn Element> {
        // let comp = self.as_ref().borrow();
        // comp.parent
        todo!()
    }

    //the parent control, None if no parent
    fn set_parent(&self, p: Option<Id>) {
        let mut comp = self.as_ref().borrow_mut();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        //do stuff with old parent

        comp.parent = p;

        if let Some(parent) = comp.parent {
            // if let Some(widget) = parent.widget() {
            //     comp.ignore_spatial = true;
            //     comp.x = widget.x() + comp.x_local;
            //     comp.y = widget.y() + comp.y_local;
            //     comp.ignore_spatial = false;
            // }
        }
    }

    #[inline]
    //the parent control, None if no parent
    fn id(&self) -> Id {
        let comp = self.as_ref().borrow();
        comp.id
    }

    // NEW API
    fn node(&self) -> Option<Node> {
        None
    }

    // you should get node layout from LayoutSystem
    // and update component properties and call relayout method on childs
    fn relayout(&self, origin: Point2<f32>) {}
}

impl PartialEq for dyn Element {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
