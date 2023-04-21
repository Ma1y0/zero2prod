use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_has_white_space = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|x| forbidden_characters.contains(&x));

        if is_empty_or_has_white_space || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid name", s))
        } else {
            Ok(Self(s))
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_long_name_is_valid() {
        let a = "a".repeat(256);
        assert_ok!(SubscriberName::parse(a));
    }
    
    #[test]
    fn a_name_longer_then_256_is_not_valid() {
        let a = "a".repeat(257);
        assert_err!(SubscriberName::parse(a));
    }

    #[test]
    fn empty_string_is_not_valid() {
        let a = String::from("");
        assert_err!(SubscriberName::parse(a));
    }

    #[test]
    fn names_with_black_listed_characters_are_not_valid() {
       for a in ['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
           let a = a.to_string();
           assert_err!(SubscriberName::parse(a));
       } 
    }

    #[test]
    fn a_valid_name_is_valid() {
        let a = String::from("John Smith");
        assert_ok!(SubscriberName::parse(a));
    }
}
