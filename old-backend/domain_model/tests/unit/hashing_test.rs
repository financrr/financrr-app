use entity::utility::hashing;

const HASH: &str = "$argon2id$v=19$m=65536,t=3,p=1$UGFzc3dvcmRTYWx0$c5vyH53HOF7gMcDPGoKw1pNTOtrjIBkCsD9NhwFvY5E";

#[test]
fn test_password_hashing() {
    let password = "password";
    let salt = "PasswordSalt";
    let hashed_password = hashing::hash_string_with_salt(password, salt).expect("Failed to hash password");
    assert_eq!(HASH, hashed_password);
}
