#[macro_export]
macro_rules! lifecycle_event {
    (
        $(#[$meta:meta])*
        pub(crate) struct $name:ident {
            $(pub(crate) $field:ident: $type:ty,)*
        }
    ) => {
        paste::paste! {
            #[allow(non_upper_case_globals)]
            static [< $name EventBus >]: std::sync::OnceLock<$crate::event::EventBus<$name>> = std::sync::OnceLock::new();
        }

        $(#[$meta])*
        pub(crate) struct $name {
            $(pub(crate) $field: $type,)*
        }

        impl $name {
            pub(crate) fn new($($field: $type,)*) -> Self {
                Self {
                    $($field,)*
                }
            }
        }

        impl $crate::event::GenericEvent for $name {
            paste::paste! {
                fn get_event_bus() -> &'static $crate::event::EventBus<Self> {
                    [< $name EventBus >].get_or_init($crate::event::EventBus::new)
                }
            }
        }
    };
}
