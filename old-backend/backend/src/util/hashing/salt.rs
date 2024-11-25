use rand::distributions::Alphanumeric;
use rand::Rng;

pub(crate) fn generate_salt(length: u32) -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(length as usize).map(char::from).collect()
}
