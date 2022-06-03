use crate::ui::Locale;

pub trait LocaleResolutionCallback {
    fn resolve(&self, locale: Option<Locale>, supported_locales: Vec<Locale>) -> Option<Locale>;
}
