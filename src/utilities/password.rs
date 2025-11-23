use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};

pub fn hash_password(password: String) -> Result<Vec<u8>, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt);

    match password_hash {
        Ok(v) => Ok(v.serialize().as_bytes().to_vec()),
        Err(_) => Err("Unable to hash password!".to_string())
    }
}

pub fn verify_password(password: String, hashed_password: Vec<u8>) -> bool {
    let argon2 = Argon2::default();
    let string= String::from_utf8(hashed_password).expect("Malformed password found in database.");
    let parsed_hash = PasswordHash::new(string.as_str()).expect("Could not parse password hash.");
    argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
}