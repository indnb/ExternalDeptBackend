use regex::Regex;

pub trait Validate {
    #[allow(dead_code)]
    fn less_for(&self, len: usize) -> bool;
    #[allow(dead_code)]
    fn greater_for(&self, len: usize) -> bool;
    #[allow(dead_code)]
    fn is_email(&self) -> bool;
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
        match Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\\.[a-z]{2,}$") {
            Ok(email_regex) => email_regex.is_match(self.as_ref()),
            Err(_) => false,
        }
    }

    fn is_phone(&self) -> bool {
        match Regex::new(r"^(\+380|0)\d{9}$") {
            Ok(phone_regex) => phone_regex.is_match(self.as_ref()),
            Err(_) => false,
        }
    }

    fn is_password(&self, max_len: usize) -> bool {
        let input = self.as_ref();
        if input.len() > max_len || input.len() <= 10 {
            return false;
        }

        let digits = match Regex::new(r"\\d") {
            Ok(digits) => digits,
            Err(_) => return false,
        };
        let symbols = match Regex::new(r"[!@#$%^&*()_+=\\\-{}\[\]|\\:;'<>,.?/~`]") {
            Ok(symbols) => symbols,
            Err(_) => return false,
        };
        let letters = match Regex::new(r"[a-zA-Z]") {
            Ok(lettres) => lettres,
            Err(_) => return false,
        };
        digits.is_match(input) && symbols.is_match(input) && letters.is_match(input)
    }
}
