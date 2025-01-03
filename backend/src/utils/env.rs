use dotenvy::Error;

pub fn load_env_file() {
    match dotenvy::dotenv() {
        Ok(_) => (),
        Err(Error::LineParse(str, line)) => eprintln!("Failed to parse line {} in .env file: {}", line, str),
        Err(Error::Io(_)) => println!("No .env file found"),
        Err(Error::EnvVar(e)) => eprintln!("Failed to load .env file: {}", e),
        Err(e) => eprintln!("Unknown error while loading .env file: {}", e),
    }
}
