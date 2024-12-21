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
}

impl<T: ToString> Validate for T {
    fn less_for(&self, len: usize) -> bool {
        self.to_string().len() < len
    }

    fn greater_for(&self, len: usize) -> bool {
        self.to_string().len() > len
    }

    fn is_email(&self) -> bool {
        let email_regex = Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap();
        email_regex.is_match(&self.to_string())
    }

    fn is_phone(&self) -> bool {
        let phone_regex = Regex::new(r"^(\+380|0)\d{9}$").unwrap();
        phone_regex.is_match(&self.to_string())
    }
}
