pub struct Password {}

impl Password {
    /// Hash a password using bcrypt
    pub fn hash(password: impl AsRef<[u8]>) -> String {
        bcrypt::hash(password.as_ref(), bcrypt::DEFAULT_COST).unwrap_or_default()
    }

    pub fn md5(password: impl AsRef<[u8]>) -> String {
        let digest = md5::compute(password.as_ref());
        format!("{:x}", digest)
    }

    /// Hash a password using md5 and then hash using bcrypt
    pub fn md5_and_hash(password: impl AsRef<[u8]>) -> String {
        let md5_password = Password::md5(password);
        Password::hash(md5_password)
    }

    pub fn verify(raw_password: &str, hash: &str) -> bool {
        if raw_password.eq(hash) {
            return true;
        }
        bcrypt::verify(raw_password, hash).unwrap_or(false)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        let s = Password::md5("123456");
        println!("{}", s);
        assert_eq!(s, "e10adc3949ba59abbe56e057f20f883e");
        println!("{}",s.len());
        
        let s = Password::md5_and_hash("123456");
        println!("{}", s);
        
    }

    #[test]
    fn test_verify() {
        let password = "123456";
        let raw_password = "123456";

        assert!(Password::verify(password, raw_password));

        let hash = Password::hash(password);
        assert!(Password::verify(password, &hash));
        let hash = Password::md5_and_hash(password);
        println!("{}", hash);
        let md5= Password::md5(password);
        println!("{}", md5);
        assert!(Password::verify(&md5, &hash));
    }
}
