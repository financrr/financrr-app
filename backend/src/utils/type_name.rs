use std::any::type_name;

pub fn type_name_only<T>() -> &'static str {
    let full_name = type_name::<T>();
    full_name.split("::").last().unwrap_or(full_name)
}
