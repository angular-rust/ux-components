use crate::widgets::Widget;

use super::BuildContext;

pub trait WidgetBuilder {
    fn build(&self, context: BuildContext) -> &dyn Widget;
}
