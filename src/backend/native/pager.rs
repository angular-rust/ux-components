#![allow(unused_variables)]

// use std::mem::transmute;
use super::{ButtonGroup, Stack, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct PagerProps {
    pub pages: Vec<clutter::Actor>,
    pub current_page: Vec<clutter::Actor>,
    pub edge_previews: bool,
    pub button_box: Option<clutter::Actor>,
    pub button_group: ButtonGroup,
    // pub pages_to_buttons: GHashTable, /* ClutterActor* -> Button* */
    pub hover_timeout: u32,
}

#[derive(Clone, Debug)]
pub struct Pager {
    props: RefCell<PagerProps>,
    widget: Widget,
}

impl Pager {
    pub fn new() -> Pager {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::pager_new()) }
        unimplemented!()
    }
}

impl Default for Pager {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Pager {}
impl Is<Pager> for Pager {}

impl AsRef<Pager> for Pager {
    fn as_ref(&self) -> &Pager {
        self
    }
}

impl Is<Stack> for Pager {}

impl AsRef<Stack> for Pager {
    fn as_ref(&self) -> &Stack {
        // &self.widget
        unimplemented!()
    }
}

impl Is<Widget> for Pager {}

impl AsRef<Widget> for Pager {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<clutter::Actor> for Pager {}

impl AsRef<clutter::Actor> for Pager {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_PAGER: Option<&Pager> = None;

pub trait PagerExt: 'static {
    /// pager_get_actor_for_page:
    /// @self: a #Pager
    /// @page: a page number
    ///
    /// Returns: (transfer none): the #ClutterActor for @page
    ///
    fn get_actor_for_page(&self, page: u32) -> Option<clutter::Actor>;

    /// pager_get_current_page:
    /// @self: a #Pager
    ///
    /// Returns: the current page number
    ///
    fn get_current_page(&self) -> u32;

    /// pager_get_current_page_actor:
    /// @self: a #Pager
    ///
    /// Returns: (transfer none): the #ClutterActor on the current page
    ///
    fn get_current_page_actor(&self) -> Option<clutter::Actor>;

    /// pager_get_edge_previews:
    /// @self: a #Pager
    ///
    /// Returns: the value of the #Pager:edge-previews property
    ///
    fn get_edge_previews(&self) -> bool;

    /// pager_get_n_pages:
    /// @self: a #Pager
    ///
    /// Returns: the number of pages in this pager
    ///
    fn get_n_pages(&self) -> usize;

    /// pager_insert_page:
    /// @self: a #Pager
    /// @child: the page to insert
    /// @position: the position to insert the page. If this is negative, or is
    ///   larger than the number of pages, it will the last page
    ///
    /// Inserts a page into the #Pager at the position specified by @position.
    ///
    fn insert_page<P: Is<clutter::Actor>>(&self, child: &P, position: i32);

    /// pager_next:
    /// @self: a #Pager
    ///
    /// Move to the next page.
    ///
    fn next(&self);

    /// pager_previous:
    /// @self: a #Pager
    ///
    /// Move to the previous page.
    ///
    fn previous(&self);

    /// pager_set_current_page:
    /// @self: a #Pager
    /// @page: the page to move to
    /// @animate: whether to animate the move between pages
    ///
    /// Move to @page.
    ///
    fn set_current_page(&self, page: u32, animate: bool);

    /// pager_set_current_page_by_actor:
    /// @self: a #Pager
    /// @actor: the actor of the page to move to
    /// @animate: whether to animate the move between pages
    ///
    /// Move to the page containing @actor.
    ///
    fn set_current_page_by_actor<P: Is<clutter::Actor>>(&self, actor: &P, animate: bool);

    /// pager_set_edge_previews:
    /// @self: a #Pager
    /// @edge_previews: %true to enable edge previews
    ///
    /// Sets the #Pager:edge-previews property.
    ///
    fn set_edge_previews(&self, edge_previews: bool);

    fn get_property_page_actor(&self) -> Option<clutter::Actor>;

    // fn set_property_page_actor<P: Is<clutter::Actor> + SetValueOptional>(
    //     &self,
    //     page_actor: Option<&P>,
    // );

    fn get_property_page_num(&self) -> u32;

    fn set_property_page_num(&self, page_num: u32);

    fn connect_property_edge_previews_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_page_actor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_num_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Pager>> PagerExt for O {
    /// pager_get_actor_for_page:
    /// @self: a #Pager
    /// @page: a page number
    ///
    /// Returns: (transfer none): the #ClutterActor for @page
    ///
    fn get_actor_for_page(&self, page: u32) -> Option<clutter::Actor> {
        let pager = self.as_ref();
        // CLUTTER_ACTOR(g_list_nth_data (pager.pages, page));
        unimplemented!()
    }

    /// pager_get_current_page:
    /// @self: a #Pager
    ///
    /// Returns: the current page number
    ///
    fn get_current_page(&self) -> u32 {
        let pager = self.as_ref();

        // let pos = g_list_position(pager.pages, pager.current_page);

        unimplemented!()
    }

    /// pager_get_current_page_actor:
    /// @self: a #Pager
    ///
    /// Returns: (transfer none): the #ClutterActor on the current page
    ///
    fn get_current_page_actor(&self) -> Option<clutter::Actor> {
        let pager = self.as_ref();

        // CLUTTER_ACTOR(pager.current_page.data);
        unimplemented!()
    }

    /// pager_get_edge_previews:
    /// @self: a #Pager
    ///
    /// Returns: the value of the #Pager:edge-previews property
    ///
    fn get_edge_previews(&self) -> bool {
        let pager = self.as_ref();
        let props = pager.props.borrow();

        props.edge_previews
    }

    /// pager_get_n_pages:
    /// @self: a #Pager
    ///
    /// Returns: the number of pages in this pager
    ///
    fn get_n_pages(&self) -> usize {
        let pager = self.as_ref();
        let props = pager.props.borrow();

        props.pages.len()
    }

    /// pager_insert_page:
    /// @self: a #Pager
    /// @child: the page to insert
    /// @position: the position to insert the page. If this is negative, or is
    ///   larger than the number of pages, it will the last page
    ///
    /// Inserts a page into the #Pager at the position specified by @position.
    ///
    fn insert_page<P: Is<clutter::Actor>>(&self, child: &P, position: i32) {
        let pager = self.as_ref();

        // pager.pages = g_list_insert(pager.pages, child, position);

        // pager_add_internal_actor(self, child, "fit", true, None);
        // clutter_actor_set_child_below_sibling((ClutterActor *)self, child, None);

        // pager_add_page_button(self, child);

        // if pager.current_page == None {
        //     pager_change_page(self, pager.pages, false);
        // } else {
        //     pager_relayout_pages(self, false);
        // }
    }

    /// pager_next:
    /// @self: a #Pager
    ///
    /// Move to the next page.
    ///
    fn next(&self) {
        let pager = self.as_ref();

        // if pager.current_page.next == None {
        //     return;
        // }

        // pager_change_page(self, pager.current_page.next, true);
    }

    /// pager_previous:
    /// @self: a #Pager
    ///
    /// Move to the previous page.
    ///
    fn previous(&self) {
        let pager = self.as_ref();

        // if pager.current_page.prev == None {
        //     return;
        // }

        // pager_change_page(self, pager.current_page.prev, true);
    }

    /// pager_set_current_page:
    /// @self: a #Pager
    /// @page: the page to move to
    /// @animate: whether to animate the move between pages
    ///
    /// Move to @page.
    ///
    fn set_current_page(&self, page: u32, animate: bool) {
        let pager = self.as_ref();

        // let page_l = g_list_nth (pager.pages, page);
        // g_return_if_fail(page_l != None);
        // pager_change_page(self, page_l, animate);
    }

    /// pager_set_current_page_by_actor:
    /// @self: a #Pager
    /// @actor: the actor of the page to move to
    /// @animate: whether to animate the move between pages
    ///
    /// Move to the page containing @actor.
    ///
    fn set_current_page_by_actor<P: Is<clutter::Actor>>(&self, actor: &P, animate: bool) {
        let pager = self.as_ref();

        // let page_l = g_list_find (pager.pages, actor);
        // g_return_if_fail(page_l != None);
        // pager_change_page(self, page_l, animate);
    }

    /// pager_set_edge_previews:
    /// @self: a #Pager
    /// @edge_previews: %true to enable edge previews
    ///
    /// Sets the #Pager:edge-previews property.
    ///
    fn set_edge_previews(&self, edge_previews: bool) {
        let pager = self.as_ref();

        // if pager.edge_previews == edge_previews {
        //     return;
        // }

        // if !edge_previews {
        //     // disable any currently pending timeout
        //     if pager.hover_timeout > 0 {
        //         g_source_remove(pager.hover_timeout);
        //         pager.hover_timeout = 0;
        //     }
        // }

        // pager.edge_previews = edge_previews;
        // g_object_notify(G_OBJECT(self), "edge-previews");
    }

    fn get_property_page_actor(&self) -> Option<clutter::Actor> {
        // unsafe {
        //     let mut value = Value::from_type(<clutter::Actor as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"page-actor\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `page-actor` getter")
        // }
        unimplemented!()
    }

    // fn set_property_page_actor<P: Is<clutter::Actor> + SetValueOptional>(
    //     &self,
    //     page_actor: Option<&P>,
    // ) {
    //     // unsafe {
    //     //     gobject_sys::g_object_set_property(
    //     //         self.to_glib_none().0 as *mut gobject_sys::GObject,
    //     //         b"page-actor\0".as_ptr() as *const _,
    //     //         Value::from(page_actor).to_glib_none().0,
    //     //     );
    //     // }
    //     unimplemented!()
    // }

    fn get_property_page_num(&self) -> u32 {
        // unsafe {
        //     let mut value = Value::from_type(<u32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"page-num\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `page-num` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_page_num(&self, page_num: u32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"page-num\0".as_ptr() as *const _,
        //         Value::from(&page_num).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn connect_property_edge_previews_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_edge_previews_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Pager,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Pager>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Pager::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::edge-previews\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_edge_previews_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_page_actor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_page_actor_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Pager,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Pager>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Pager::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::page-actor\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_page_actor_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_page_num_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_page_num_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Pager,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Pager>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Pager::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::page-num\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_page_num_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Pager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pager")
    }
}
