use crate::ui::Locale;

pub trait LocaleListResolutionCallback {
    fn resolve(
        &self,
        locales: Option<Vec<Locale>>,
        supported_locales: Vec<Locale>,
    ) -> Option<Locale>;
}
