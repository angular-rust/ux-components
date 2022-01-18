use super::BuildContext;

pub trait GenerateAppTitle {
    fn on_generate_title(&self, context: BuildContext) -> String;
}
