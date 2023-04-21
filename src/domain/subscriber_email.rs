use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
         if validate_email(&s) {
             Ok(Self(s))
         } else {
             Err(format!("{} is not a valid email", s))
         }
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claims::assert_err;
    //use fake::faker::internet::en::SafeEmail;
    //use fake::Fake;
   
    #[test]
    fn empty_email_is_rejected() {
        let a = String::from("");
        assert_err!(SubscriberEmail::parse(a));
    }

    #[test]
    fn email_missing_at_is_rejected() {
        let a = String::from("testgmail.com");
        assert_err!(SubscriberEmail::parse(a));
    }

    #[test]
    fn email_missing_name_is_rejected() {
        let a = String::from("@gmail.com");
        assert_err!(SubscriberEmail::parse(a));
    }
    
    // TODO: fic this?
    /*
    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
    */

}
