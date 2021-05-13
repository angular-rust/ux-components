#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, Widget};
use glib::signal::SignalHandlerId;
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct NotebookProps {
    pub current_page: Option<Actor>,
    pub children: Vec<Actor>,
}

#[derive(Clone, Debug)]
pub struct Notebook {
    props: RefCell<NotebookProps>,
    widget: Widget,
}

impl Notebook {
    pub fn new() -> Notebook {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::notebook_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Notebook {}
impl Is<Notebook> for Notebook {}

impl AsRef<Notebook> for Notebook {
    fn as_ref(&self) -> &Notebook {
        self
    }
}

impl Is<Widget> for Notebook {}

impl AsRef<Widget> for Notebook {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<Actor> for Notebook {}

impl AsRef<Actor> for Notebook {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait NotebookExt: 'static {
    /// notebook_get_current_page:
    /// @notebook: A #Notebook
    ///
    /// Get the current page
    ///
    /// Returns: (transfer none): the current page
    ///
    fn get_current_page(&self) -> Option<Actor>;

    /// notebook_next_page:
    /// @notebook: A #Notebook
    ///
    /// Change the current page to next one.
    ///
    fn next_page(&self);

    /// notebook_previous_page:
    /// @notebook: A #Notebook
    ///
    /// Change the current page to previous one.
    ///
    fn previous_page(&self);

    fn set_current_page<P: Is<Actor>>(&self, page: &P);

    fn connect_property_current_page_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: Is<Notebook>> NotebookExt for O {
    /// notebook_get_current_page:
    /// @notebook: A #Notebook
    ///
    /// Get the current page
    ///
    /// Returns: (transfer none): the current page
    ///
    fn get_current_page(&self) -> Option<Actor> {
        let notebook = self.as_ref();
        let props = notebook.props.borrow();

        props.current_page.clone()
    }

    /// notebook_next_page:
    /// @notebook: A #Notebook
    ///
    /// Change the current page to next one.
    ///
    fn next_page(&self) {
        let notebook = self.as_ref();

        // let item = g_list_find(notebook.children, notebook.current_page);
        // if !item {
        //     g_warning("Current page not found in child list");
        //     return;
        // }

        // if item->next {
        //     notebook_set_current_page(notebook,
        //                                 (ClutterActor *)item.next.data);
        // } else {
        //     notebook_set_current_page(notebook,
        //                                 (ClutterActor *)notebook.children.data);
        // }
    }

    /// notebook_previous_page:
    /// @notebook: A #Notebook
    ///
    /// Change the current page to previous one.
    ///
    fn previous_page(&self) {
        let notebook = self.as_ref();

        // let item = g_list_find(notebook.children, notebook.current_page);
        // if !item {
        //     g_warning("Current page not found in child list");
        //     return;
        // }

        // if item.prev {
        //     notebook_set_current_page(notebook,
        //                                 (ClutterActor *)item.prev.data);
        // } else {
        //     notebook_set_current_page(notebook,
        //                                 (ClutterActor *)g_list_last(item).data);
        // }
    }

    fn set_current_page<P: Is<Actor>>(&self, page: &P) {
        let notebook = self.as_ref();
        let page = page.as_ref();
        let props = notebook.props.borrow();

        // if page == props.current_page {
        //     return;
        // }

        // props.current_page = page;

        // // ensure the correct child is visible
        // notebook_update_children(book);
        // g_object_notify(G_OBJECT(book), "current-page");
    }

    fn connect_property_current_page_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_current_page_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Notebook,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Notebook>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Notebook::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::current-page\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_current_page_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Notebook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Notebook")
    }
}
