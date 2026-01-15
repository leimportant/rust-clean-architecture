use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

pub struct PasswordService;

impl PasswordService {
    pub fn hash(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default(); // Argon2id default (AMAN)

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| "failed to hash password".into())
    }

    pub fn verify(password: &str, hash: &str) -> bool {
        let parsed = PasswordHash::new(hash).ok();
        match parsed {
            Some(parsed) => Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok(),
            None => false,
        }
    }
}
