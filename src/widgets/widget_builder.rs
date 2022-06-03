use crate::widgets::Widget;

use super::BuildContext;

// Box<dyn Fn(BuildContext, AsyncSnapshot<T>) -> Box<dyn Widget>>

pub type WidgetBuilder = Box<dyn Fn(BuildContext) -> Box<dyn Widget>>;

// pub trait WidgetBuilder {
//     fn build(&self, context: Option<BuildContext>) -> &dyn Widget;
// }

// #[derive(Default)]
// pub struct NullWidgetBuilder;

// impl WidgetBuilder for NullWidgetBuilder {
//     fn build(&self, context: Option<BuildContext>) -> &dyn Widget {
//         todo!()
//     }
// }