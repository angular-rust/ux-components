#![allow(unused_imports)]

#[macro_use]
extern crate glib;

#[macro_use]
extern crate bitflags;

mod backend;
pub use backend::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod prelude {

    // #[derive(Clone, Debug, Default)]
    // pub struct Widget {
    //     pub width: f64,
    //     pub height: f64,
    // }

    // impl UxComponent for Widget {}
    // impl Is<Widget> for Widget {}

    // impl AsRef<Widget> for Widget {
    //     fn as_ref(&self) -> &Widget {
    //         unimplemented!()
    //     }
    // }

    // #[derive(Clone, Debug)]
    // pub struct ButtonComponent();
    // impl UxComponent for ButtonComponent {
    //     // fn get_props(&self) -> &Self {
    //     //     self
    //     // }
    // }
    // impl Is<Widget> for ButtonComponent {}

    // impl AsRef<Widget> for ButtonComponent {
    //     fn as_ref(&self) -> &Widget {
    //         unimplemented!()
    //     }
    // }

    // /* MoreTraits or supertrait +*/

    // pub trait WidgetExt: 'static {
    //     fn get_disabled(&self) -> bool;
    // }

    // impl<O: Is<Widget> + 'static> WidgetExt for O {
    //     fn get_disabled(&self) -> bool {
    //         let b = self.as_ref();
    //         println!("{}", b.height);
    //         true
    //     }
    // }

    // fn magic() {
    //     let c = ButtonComponent();
    //     c.get_disabled(); // <<< WHAT A FUCK? ITS A MAGGGIIIICCCC!!!!
    //     let a = c.as_ref(); // << SUPER MAGIC !!!!!!!!!!!!!!!!!!!!!!
    // }

    // #[derive(Clone, Debug)]
    // pub struct WindowComponent();
    // impl UxComponent for WindowComponent {}
    // impl Is<Widget> for WindowComponent {}
    // impl AsRef<Widget> for WindowComponent {
    //     fn as_ref(&self) -> &Widget {
    //         unimplemented!()
    //     }
    // }

    // #[derive(Default)]
    // struct TestAppliaction {
    //     windows: Vec<Box<dyn Is<Widget>>>,
    // }

    // impl TestAppliaction {
    //     fn new() -> Self {
    //         Self {
    //             windows: Vec::new(),
    //         }
    //     }

    //     fn add(&mut self, value: Box<dyn Is<Widget>>) {
    //         self.windows.push(value)
    //     }
    // }

    // fn check1<P: Is<Widget>>(t: P) {
    //     let a = Widget::default();
    //     let b = WindowComponent();
    //     let c = ButtonComponent();

    //     c.get_disabled(); // WHATT A FUCK
    //     let mut app = TestAppliaction::new();
    //     app.add(Box::new(a));
    //     app.add(Box::new(b));
    //     app.add(Box::new(c));
    // }

    pub trait UxComponent: std::fmt::Debug + Clone + 'static {
        // fn get_props(&self) -> &Self;
    }
    pub trait Is<T: UxComponent>: AsRef<T> + 'static {}
}

impl prelude::UxComponent for clutter::Actor {}