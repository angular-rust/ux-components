#![allow(unused_variables)]

// use std::mem::transmute;
use super::{BoxLayout, ItemFactory, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct AttributeData {
    pub name: String,
    pub col: usize,
}

#[derive(Clone, Debug)]
pub struct ListViewProps {
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
}

#[derive(Clone, Debug)]
pub struct ListView {
    props: RefCell<ListViewProps>,
    widget: Widget,
}

impl ListView {
    pub fn new() -> ListView {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::listview_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for ListView {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for ListView {}
impl Is<ListView> for ListView {}

impl AsRef<ListView> for ListView {
    fn as_ref(&self) -> &ListView {
        self
    }
}

impl Is<BoxLayout> for ListView {}

impl AsRef<BoxLayout> for ListView {
    fn as_ref(&self) -> &BoxLayout {
        // &self.widget
        unimplemented!()
    }
}

impl Is<Widget> for ListView {}

impl AsRef<Widget> for ListView {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<clutter::Actor> for ListView {}

impl AsRef<clutter::Actor> for ListView {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_LIST_VIEW: Option<&ListView> = None;

pub trait ListViewExt: 'static {
    /// add_attribute:
    /// @listview: An #ListView
    /// @attribute: Name of the attribute
    /// @column: Column number
    ///
    /// Adds an attribute mapping between the current model and the objects from the
    /// cell renderer.
    ///
    fn add_attribute(&self, attribute: &str, column: u32);

    /// freeze:
    /// @listview: An #ListView
    ///
    /// Freeze the view. This means that the view will not act on changes to the
    /// model until it is thawed. Call #thaw to thaw the view.
    ///
    fn freeze(&self);

    /// get_factory:
    /// @listview: A #ListView
    ///
    /// Gets the #ItemFactory used for creating new list items.
    ///
    /// Returns: (transfer none): A #ItemFactory.
    ///
    fn get_factory(&self) -> Option<ItemFactory>;

    /// get_item_type:
    /// @listview: An #ListView
    ///
    /// Get the item type currently being used to create items
    ///
    /// Returns: a #GType
    ///
    fn get_item_type(&self) -> glib::types::Type;

    /// get_model:
    /// @listview: An #ListView
    ///
    /// Get the model currently used by the #ListView
    ///
    /// Returns: (transfer none): the current #ClutterModel
    ///
    fn get_model(&self) -> Option<clutter::Model>;

    /// set_factory:
    /// @listview: A #ListView
    /// @factory: (allow-none): A #ItemFactory
    ///
    /// Sets @factory to be the factory used for creating new list items
    ///
    fn set_factory(&self, factory: Option<&ItemFactory>);

    /// set_item_type:
    /// @listview: An #ListView
    /// @item_type: A #GType
    ///
    /// Set the item type used to create items representing each row in the
    /// model
    ///
    fn set_item_type(&self, item_type: glib::types::Type);

    /// set_model:
    /// @listview: An #ListView
    /// @model: A #ClutterModel
    ///
    /// Set the model used by the #ListView
    ///
    fn set_model<P: Is<clutter::Model>>(&self, model: &P);

    /// thaw:
    /// @listview: An #ListView
    ///
    /// Thaw the view. This means that the view will now act on changes to the
    /// model.
    ///
    fn thaw(&self);

    fn connect_property_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_item_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<ListView>> ListViewExt for O {
    /// add_attribute:
    /// @listview: An #ListView
    /// @attribute: Name of the attribute
    /// @column: Column number
    ///
    /// Adds an attribute mapping between the current model and the objects from the
    /// cell renderer.
    ///
    fn add_attribute(&self, attribute: &str, column: u32) {
        let listview = self.as_ref();

        // let prop = g_new (AttributeData, 1);
        // prop.name = g_strdup (_attribute);
        // prop.col = column;

        // listview.attributes.push(prop);
        // model_changed_cb(listview.model, listview);
    }

    /// freeze:
    /// @listview: An #ListView
    ///
    /// Freeze the view. This means that the view will not act on changes to the
    /// model until it is thawed. Call #thaw to thaw the view.
    ///
    fn freeze(&self) {
        let listview = self.as_ref();
        // listview.is_frozen = true;
    }

    /// get_factory:
    /// @listview: A #ListView
    ///
    /// Gets the #ItemFactory used for creating new list items.
    ///
    /// Returns: (transfer none): A #ItemFactory.
    ///
    fn get_factory(&self) -> Option<ItemFactory> {
        let listview = self.as_ref();
        let props = listview.props.borrow();

        props.factory.clone()
    }

    /// get_item_type:
    /// @listview: An #ListView
    ///
    /// Get the item type currently being used to create items
    ///
    /// Returns: a #GType
    ///
    fn get_item_type(&self) -> glib::types::Type {
        let listview = self.as_ref();
        let props = listview.props.borrow();

        props.item_type
    }

    /// get_model:
    /// @listview: An #ListView
    ///
    /// Get the model currently used by the #ListView
    ///
    /// Returns: (transfer none): the current #ClutterModel
    ///
    fn get_model(&self) -> Option<clutter::Model> {
        let listview = self.as_ref();
        let props = listview.props.borrow();

        props.model.clone()
    }

    /// set_factory:
    /// @listview: A #ListView
    /// @factory: (allow-none): A #ItemFactory
    ///
    /// Sets @factory to be the factory used for creating new list items
    ///
    fn set_factory(&self, factory: Option<&ItemFactory>) {
        let listview = self.as_ref();
        let mut props = listview.props.borrow_mut();

        // if listview.factory == factory {
        //     return;
        // }

        if props.factory.is_some() {
            // g_object_unref(listview.factory);
            props.factory = None;
        }

        if factory.is_some() {
            // props.factory = g_object_ref(factory);
        }

        // g_object_notify(G_OBJECT(listview), "factory");
    }

    /// set_item_type:
    /// @listview: An #ListView
    /// @item_type: A #GType
    ///
    /// Set the item type used to create items representing each row in the
    /// model
    ///
    fn set_item_type(&self, item_type: glib::types::Type) {
        let listview = self.as_ref();
        let mut props = listview.props.borrow_mut();

        props.item_type = item_type;

        // // update the view
        // model_changed_cb(listview.model, listview);
    }

    /// set_model:
    /// @listview: An #ListView
    /// @model: A #ClutterModel
    ///
    /// Set the model used by the #ListView
    ///
    fn set_model<P: Is<clutter::Model>>(&self, model: &P) {
        let listview = self.as_ref();
        let mut props = listview.props.borrow_mut();

        if props.model.is_some() {
            // g_signal_handlers_disconnect_by_func(listview.model,
            //                                     (GCallback) model_changed_cb,
            //                                     listview);
            // g_signal_handlers_disconnect_by_func(listview.model,
            //                                     (GCallback) row_changed_cb,
            //                                     listview);
            // g_signal_handlers_disconnect_by_func(listview.model,
            //                                     (GCallback) row_removed_cb,
            //                                     listview);
            // g_object_unref(listview.model);

            props.model = None;
        }

        // if model {
        //     g_return_if_fail(CLUTTER_IS_MODEL(model));

        //     props.model = g_object_ref (model);

        //     props.filter_changed = g_signal_connect(listview.model,
        //                                         "filter-changed",
        //                                         G_CALLBACK(model_changed_cb),
        //                                         listview);

        //     props.row_added = g_signal_connec (listview.model,
        //                                     "row-added",
        //                                     G_CALLBAC (row_changed_cb),
        //                                     listview);

        //     props.row_changed = g_signal_connect (listview.model,
        //                                         "row-changed",
        //                                         G_CALLBACK(row_changed_cb),
        //                                         listview);

        //     // model_changed_cb (called from row_changed_cb) expect the row to already
        //     // have been removed, thus we need to use _after
        //     props.row_removed = g_signal_connect_after(listview.model,
        //                                             "row-removed",
        //                                             G_CALLBACK(row_removed_cb),
        //                                             listview);

        //     props.sort_changed = g_signal_connect(listview.model,
        //                                         "sort-changed",
        //                                         G_CALLBACK(model_changed_cb),
        //                                         listview);

        //     // Only do this inside this block, setting the model to None should have
        //     // the effect of preserving the view; just disconnect the handlers
        //     model_changed_cb(listview.model, listview);
        // }
    }

    /// thaw:
    /// @listview: An #ListView
    ///
    /// Thaw the view. This means that the view will now act on changes to the
    /// model.
    ///
    fn thaw(&self) {
        let listview = self.as_ref();
        let mut props = listview.props.borrow_mut();

        props.is_frozen = false;

        // // Repopulate
        // model_changed_cb(listview.model, listview);
    }

    fn connect_property_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_factory_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ListView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ListView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ListView::from_glib_borrow(this).unsafe_cast_ref())
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
        //     this: *mut ffi::ListView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ListView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ListView::from_glib_borrow(this).unsafe_cast_ref())
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
        //     this: *mut ffi::ListView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ListView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ListView::from_glib_borrow(this).unsafe_cast_ref())
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

impl fmt::Display for ListView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListView")
    }
}
