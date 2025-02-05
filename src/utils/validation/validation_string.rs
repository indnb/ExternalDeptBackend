use regex::Regex;

lazy_static::lazy_static! {
    pub static ref EMAIL_REGEX: Regex =
        Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap();
    pub static ref NICKNAME_REGEX: Regex =
        Regex::new(r"^[a-zA-Z0-9](?:[a-zA-Z0-9_]{3,30}[a-zA-Z0-9])?$").unwrap();
    pub static ref PHONE_REGEX: Regex = Regex::new(r"^(\+380|0)\d{9}$").unwrap();
    pub static ref HAS_DIGIT: Regex = Regex::new(r"\d").unwrap();
    pub static ref HAS_SYMBOL: Regex = Regex::new(r"[!@#$%^&*()_+=\-{}\[\]|\\:;'<>,.?/~`]").unwrap();
    pub static ref HAS_LOWERCASE: Regex = Regex::new(r"[a-z]").unwrap();
    pub static ref HAS_UPPERCASE: Regex = Regex::new(r"[A-Z]").unwrap();
}

pub trait Validate {
    #[allow(dead_code)]
    fn less_for(&self, len: usize) -> bool;

    #[allow(dead_code)]
    fn greater_for(&self, len: usize) -> bool;

    #[allow(dead_code)]
    fn is_email(&self) -> bool;

    #[allow(dead_code)]
    fn is_nickname_tg(&self) -> bool;

    #[allow(dead_code)]
    fn is_phone(&self) -> bool;

    #[allow(dead_code)]
    fn is_password(&self, max_len: usize) -> bool;
}

impl<T: AsRef<str>> Validate for T {
    fn less_for(&self, len: usize) -> bool {
        self.as_ref().len() < len
    }

    fn greater_for(&self, len: usize) -> bool {
        self.as_ref().len() > len
    }

    fn is_email(&self) -> bool {
        EMAIL_REGEX.is_match(self.as_ref())
    }

    fn is_nickname_tg(&self) -> bool {
        NICKNAME_REGEX.is_match(self.as_ref())
    }

    fn is_phone(&self) -> bool {
        PHONE_REGEX.is_match(self.as_ref())
    }

    fn is_password(&self, max_len: usize) -> bool {
        let input = self.as_ref();

        if input.len() > max_len || input.len() < 10 {
            return false;
        }

        HAS_DIGIT.is_match(input)
            && HAS_SYMBOL.is_match(input)
            && HAS_LOWERCASE.is_match(input)
            && HAS_UPPERCASE.is_match(input)
    }
}
