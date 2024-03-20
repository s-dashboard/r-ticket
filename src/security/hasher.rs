use argon2::{Argon2, PasswordHash, PasswordVerifier};
// use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
// use password_hash::Salt;

// https://github.com/RustCrypto/password-hashes
// pub fn get_hashed(clear_text: String) -> String {
//     let salt_and_pepper_str = STANDARD_NO_PAD.encode(b"rpRHpiEeymMrXdIMU6W0VpO");
//     let password: &[u8] = clear_text.as_bytes();
//     let salt: Salt<'_> = Salt::from_b64(&salt_and_pepper_str).unwrap();
//     let result = PasswordHasher::hash_password(&Argon2::default(), password, salt);

//     return result.unwrap().serialize().as_str().to_string(); 
// }

pub fn verify_hash(clear_text: String, hash_string:String) -> Result<(), password_hash::Error> {
    let password_hash = PasswordHash::new(&hash_string);

    // Trait objects for algorithms to support
    let algs: &[&dyn PasswordVerifier] = &[&Argon2::default()];
    password_hash.unwrap().verify_password(algs, clear_text)
}