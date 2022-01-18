use once_cell::sync::OnceCell;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::BTreeMap,
    fmt::{Debug, Formatter, Result},
    rc::Rc,
};

use crate::prelude::Singleton;

use super::{WidgetRenderHolder, WidgetRenderer};

/// A rendering provider for a WidgetComponent.
/// WidgetComponents will typically ask the rendering provider
/// for a concrete control `Renderer` instance to associate
/// with a given control. Framework specific rendering providers
/// will extend this struct and return a `Renderer` instance for
/// the requested type. i.e `if (type == ::Canvas) return MyCanvasRenderer()`
pub struct WidgetRenderFactory {
    storages: RefCell<BTreeMap<TypeId, Rc<dyn Any>>>,
}

unsafe impl std::marker::Send for WidgetRenderFactory {}
unsafe impl std::marker::Sync for WidgetRenderFactory {}

impl Debug for WidgetRenderFactory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("WidgetRenderFactory")
            .field("storages", &self.storages.borrow().len())
            .finish()
    }
}

impl WidgetRenderFactory {
    fn new() -> Self {
        Self {
            storages: Default::default(),
        }
    }

    pub fn has<T: 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.storages.borrow().contains_key(&type_id)
    }

    pub fn register<T: 'static + Debug>(&self, render: Box<dyn WidgetRenderer<T>>) {
        let type_id = TypeId::of::<T>();

        log::debug!("Register Widget Renderer {:?}", render);

        self.storages
            .borrow_mut()
            .insert(type_id, Rc::new(WidgetRenderHolder(render)));
    }

    pub fn remove<T: 'static + Debug>(&self) -> Option<Rc<WidgetRenderHolder<T>>> {
        let type_id = TypeId::of::<T>();

        match self.storages.borrow_mut().remove(&type_id) {
            Some(render) => match render.downcast::<WidgetRenderHolder<T>>() {
                Ok(render) => Some(render),
                Err(_) => {
                    log::error!("Something wrong with render storage");
                    None
                }
            },
            None => None,
        }
    }

    /// Asks the Rendering service for a Renderer instance,
    /// For a given control struct type and instance.
    pub fn get<T: 'static + Debug>(&self) -> Option<Rc<WidgetRenderHolder<T>>> {
        let type_id = TypeId::of::<T>();

        match self.storages.borrow().get(&type_id) {
            Some(item) => match item.clone().downcast::<WidgetRenderHolder<T>>() {
                Ok(render) => Some(render),
                Err(_) => {
                    log::error!("Something wrong with render storage");
                    None
                }
            },
            None => {
                log::warn!("Not found render for {}", std::any::type_name::<T>());
                None
            }
        }
    }
}

impl Singleton for WidgetRenderFactory {
    fn global() -> &'static Self {
        static WIDGET_RENDER_FACTORY_INSTANCE: OnceCell<WidgetRenderFactory> = OnceCell::new();
        WIDGET_RENDER_FACTORY_INSTANCE.get_or_init(Self::new)
    }
}
