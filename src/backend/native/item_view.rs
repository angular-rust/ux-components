#![allow(unused_variables)]

// use std::mem::transmute;
use super::{Grid, ItemFactory, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct AttributeData {
    pub name: String,
    pub col: i32,
}

#[derive(Clone, Debug)]
pub struct ItemView {
    pub model: Option<clutter::Model>,
    pub attributes: Vec<AttributeData>,
    pub item_type: glib::types::Type,
    pub factory: Option<ItemFactory>,
    pub filter_changed: u64,
    pub row_added: u64,
    pub row_changed: u64,
    pub row_removed: u64,
    pub sort_changed: u64,
    pub is_frozen: bool,
    widget: Widget,
}

impl ItemView {
    pub fn new() -> ItemView {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::item_view_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for ItemView {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for ItemView {}
impl Is<ItemView> for ItemView {}

impl AsRef<ItemView> for ItemView {
    fn as_ref(&self) -> &ItemView {
        self
    }
}

impl Is<Grid> for ItemView {}

impl AsRef<Grid> for ItemView {
    fn as_ref(&self) -> &Grid {
        // &self.widget
        unimplemented!()
    }
}

impl Is<Widget> for ItemView {}

impl AsRef<Widget> for ItemView {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<clutter::Actor> for ItemView {}

impl AsRef<clutter::Actor> for ItemView {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_ITEM_VIEW: Option<&ItemView> = None;

pub trait ItemViewExt: 'static {
    /// add_attribute:
    /// @item_view: An #ItemView
    /// @attribute: Name of the attribute
    /// @column: Column number
    ///
    /// Adds an attribute mapping between the current model and the objects from the
    /// cell renderer.
    ///
    fn add_attribute(&self, attribute: &str, column: usize);

    /// freeze:
    /// @item_view: An #ItemView
    ///
    /// Freeze the view. This means that the view will not act on changes to the
    /// model until it is thawed. Call #thaw to thaw the view
    ///
    fn freeze(&self);

    /// get_factory:
    /// @item_view: A #ItemView
    ///
    /// Gets the #ItemFactory used for creating new items.
    ///
    /// Returns: (transfer none): A #ItemFactory.
    ///
    fn get_factory(&self) -> Option<ItemFactory>;

    /// get_item_type:
    /// @item_view: An #ItemView
    ///
    /// Get the item type currently being used to create items
    ///
    /// Returns: a #GType
    ///
    fn get_item_type(&self) -> glib::types::Type;

    /// get_model:
    /// @item_view: An #ItemView
    ///
    /// Get the model currently used by the #ItemView
    ///
    /// Returns: (transfer none): the current #ClutterModel
    ///
    fn get_model(&self) -> Option<clutter::Model>;

    /// set_factory:
    /// @item_view: A #ItemView
    /// @factory: (allow-none): A #ItemFactory
    ///
    /// Sets @factory to be the factory used for creating new items
    ///
    fn set_factory(&self, factory: Option<&ItemFactory>);

    /// set_item_type:
    /// @item_view: An #ItemView
    /// @item_type: A #GType
    ///
    /// Set the item type used to create items representing each row in the model
    ///
    fn set_item_type(&self, item_type: glib::types::Type);

    /// set_model:
    /// @item_view: An #ItemView
    /// @model: A #ClutterModel
    ///
    /// Set the model used by the #ItemView
    ///
    fn set_model<P: Is<clutter::Model>>(&self, model: &P);

    /// thaw:
    /// @item_view: An #ItemView
    ///
    /// Thaw the view. This means that the view will now act on changes to the model.
    ///
    fn thaw(&self);

    fn connect_property_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_item_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<ItemView>> ItemViewExt for O {
    /// add_attribute:
    /// @item_view: An #ItemView
    /// @attribute: Name of the attribute
    /// @column: Column number
    ///
    /// Adds an attribute mapping between the current model and the objects from the
    /// cell renderer.
    ///
    fn add_attribute(&self, attribute: &str, column: usize) {
        let itemview = self.as_ref();

        // AttributeData *prop;

        // let prop = g_new (AttributeData, 1);
        // prop.name = g_strdup (_attribute);
        // prop.col = column;

        // itemview.attributes.push(prop);
        // model_changed_cb(itemview.model, item_view);
    }

    /// freeze:
    /// @item_view: An #ItemView
    ///
    /// Freeze the view. This means that the view will not act on changes to the
    /// model until it is thawed. Call #thaw to thaw the view
    ///
    fn freeze(&self) {
        let itemview = self.as_ref();
        // itemview.is_frozen = true;
    }

    /// get_factory:
    /// @item_view: A #ItemView
    ///
    /// Gets the #ItemFactory used for creating new items.
    ///
    /// Returns: (transfer none): A #ItemFactory.
    ///
    fn get_factory(&self) -> Option<ItemFactory> {
        let itemview = self.as_ref();
        itemview.factory.clone()
    }

    /// get_item_type:
    /// @item_view: An #ItemView
    ///
    /// Get the item type currently being used to create items
    ///
    /// Returns: a #GType
    ///
    fn get_item_type(&self) -> glib::types::Type {
        let itemview = self.as_ref();
        itemview.item_type
    }

    /// get_model:
    /// @item_view: An #ItemView
    ///
    /// Get the model currently used by the #ItemView
    ///
    /// Returns: (transfer none): the current #ClutterModel
    ///
    fn get_model(&self) -> Option<clutter::Model> {
        let itemview = self.as_ref();
        itemview.model.clone()
    }

    /// set_factory:
    /// @item_view: A #ItemView
    /// @factory: (allow-none): A #ItemFactory
    ///
    /// Sets @factory to be the factory used for creating new items
    ///
    fn set_factory(&self, factory: Option<&ItemFactory>) {
        let itemview = self.as_ref();

        // if itemview.factory == factory {
        //     return;
        // }

        // if itemview.factory {
        //     g_object_unref(itemview.factory);
        //     itemview.factory = None;
        // }

        // if factory {
        //     itemview.factory = g_object_ref(factory);
        // }

        // g_object_notify(G_OBJECT(item_view), "factory");
    }

    /// set_item_type:
    /// @item_view: An #ItemView
    /// @item_type: A #GType
    ///
    /// Set the item type used to create items representing each row in the model
    ///
    fn set_item_type(&self, item_type: glib::types::Type) {
        let itemview = self.as_ref();

        // itemview.item_type = item_type;

        // // update the view
        // model_changed_cb(itemview.model, itemview);
    }

    /// set_model:
    /// @item_view: An #ItemView
    /// @model: A #ClutterModel
    ///
    /// Set the model used by the #ItemView
    ///
    fn set_model<P: Is<clutter::Model>>(&self, model: &P) {
        let itemview = self.as_ref();

        // if itemview.model {
        //     g_signal_handlers_disconnect_by_func(itemview.model,
        //                                             (GCallback)model_changed_cb,
        //                                             itemview);
        //     g_signal_handlers_disconnect_by_func(itemview.model,
        //                                             (GCallback)row_changed_cb,
        //                                             itemview);
        //     g_signal_handlers_disconnect_by_func(itemview.model,
        //                                             (GCallback)row_removed_cb,
        //                                             itemview);
        //     g_object_unref (itemview.model);

        //     itemview.model = None;
        // }

        // if model {
        //     g_return_if_fail(CLUTTER_IS_MODEL(model));

        //     itemview.model = g_object_ref(model);

        //     itemview.filter_changed = g_signal_connect(itemview.model,
        //                                             "filter-changed",
        //                                             G_CALLBACK(model_changed_cb),
        //                                             itemview);

        //     itemview.row_added = g_signal_connect(itemview.model,
        //                                         "row-added",
        //                                         G_CALLBACK(row_changed_cb),
        //                                         itemview);

        //     itemview.row_changed = g_signal_connect(itemview.model,
        //                                             "row-changed",
        //                                             G_CALLBACK(row_changed_cb),
        //                                             itemview);

        //     // model_changed_cb (called from row_changed_cb) expect the row to already
        //     // have been removed, thus we need to use _after
        //     itemview.row_removed = g_signal_connect_after(itemview.model,
        //                                                 "row-removed",
        //                                                 G_CALLBACK(row_removed_cb),
        //                                                 item_view);

        //     itemview.sort_changed = g_signal_connect(itemview.model,
        //                                             "sort-changed",
        //                                             G_CALLBACK(model_changed_cb),
        //                                             item_view);

        //     // Only do this inside this block, setting the model to None should have
        //     // the effect of preserving the view; just disconnect the handlers
        //     model_changed_cb(itemview.model, itemview);
        // }
    }

    /// thaw:
    /// @item_view: An #ItemView
    ///
    /// Thaw the view. This means that the view will now act on changes to the model.
    ///
    fn thaw(&self) {
        let itemview = self.as_ref();

        // itemview.is_frozen = false;

        // // Repopulate
        // model_changed_cb(itemview.model, itemview);
    }

    fn connect_property_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_factory_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ItemView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ItemView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ItemView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::factory\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_factory_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_item_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_item_type_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ItemView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ItemView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ItemView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::item-type\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_item_type_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ItemView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ItemView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ItemView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::model\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_model_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for ItemView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ItemView")
    }
}
