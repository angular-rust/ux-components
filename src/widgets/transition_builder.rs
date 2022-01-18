use crate::widgets::Widget;

use super::BuildContext;

pub trait TransitionBuilder {
    fn build(&self, context: BuildContext, child: Option<&dyn Widget>) -> Box<dyn Widget>;
}
