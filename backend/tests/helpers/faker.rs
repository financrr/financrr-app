use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn generate_random_email() -> String {
    let local_part: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("{}@financrr.test", local_part)
}
