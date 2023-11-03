#[cfg(test)]
mod tests {
    #[test]
    fn add_ints() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn add_strings() {
        let a = "Hello ".to_string();
        let b = a + "world!";
        assert_eq!(b, "Hello world!");
    }

    #[test]
    fn argon2() {
        use argon2::{
            password_hash::{
                rand_core::OsRng,
                PasswordHash, PasswordHasher, PasswordVerifier, SaltString
            },
            Argon2
        };

        let password = b"hunter42"; // Bad password; don't actually use!
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

        // Verify password against PHC string.
        //
        // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
        // `Argon2` instance.
        let parsed_hash = PasswordHash::new(&password_hash).unwrap();
        assert!(Argon2::default().verify_password(password, &parsed_hash).is_ok());
    }
}

/*
# cargo test
   Compiling backend v0.1.0 (/root/projects/flashcard/backend)
    Finished test [unoptimized + debuginfo] target(s) in 0.24s
     Running unittests src/main.rs (target/debug/deps/backend-1dfcf1f8ad3adfc6)

running 2 tests
test tests::tests::add_ints ... ok
test tests::tests::add_strings ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/