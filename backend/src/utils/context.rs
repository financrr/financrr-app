use loco_rs::app::AppContext;

pub trait AdditionalAppContextMethods {
    fn is_mailer_enabled(&self) -> bool;
}

impl AdditionalAppContextMethods for AppContext {
    fn is_mailer_enabled(&self) -> bool {
        self.mailer.is_some()
    }
}
