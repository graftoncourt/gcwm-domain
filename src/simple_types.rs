use regex::Regex;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConstrainedIndividualNameString100(String);

impl ConstrainedIndividualNameString100 {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ConstrainedIndividualNameString100 {
    type Error = &'static str;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        let regex_any_numbers = Regex::new(r"[\d]").unwrap();
        let regex_unusual_characters = Regex::new(r"[!£$%^&*(){}\\/+]").unwrap();
        
        if name.is_empty() {
            Err("Constrained string 100 characters must have at least one character.")
        } else if name.len() > 100 {
            Err("Constrained string 100 must not have more than 100 characters.")
        } else if regex_any_numbers.is_match(&name) {
            Err("Constrained name string 100 must not have any numbers in it.")
        } else if regex_unusual_characters.is_match(&name) {
            Err("Constrained name string 100 must not have any unusual special characters such as ! £ $ % ^ & * () {} \\ / _ + in it.")
        } else {
            Ok(Self(name))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for EmailAddress {
    type Error = &'static str;

    fn try_from(email_address: String) -> Result<Self, Self::Error> {
        let regex_email = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9-]+(\.[a-zA-Z]{2,})$").unwrap();

        if !regex_email.is_match(&email_address) {
            Err("Invalid email address format.")
        } else {
            Ok(Self(email_address))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostalAddress {
    house_name: Option<String>,
    house_number: Option<i32>,
    address_line_one: String,
    address_line_two: Option<String>,
    address_line_three: Option<String>,
    address_line_four: Option<String>,
    city: String,
    postcode: String,
}

impl PostalAddress {
    pub fn new(
        house_name: Option<String>,
        house_number: Option<i32>,
        address_line_one: String,
        address_line_two: Option<String>,
        address_line_three: Option<String>,
        address_line_four: Option<String>,
        city: String,
        postcode: String,
    ) -> Result<Self, String> {
        let regex_postcode = Regex::new(r"^[A-Z]{1,2}\d[A-Z\d]? ?\d[A-Z]{2}$").unwrap();

        if address_line_one.is_empty() || city.is_empty() || postcode.is_empty() {
            Err("Address line 1, city, and postcode must not be empty.".to_string())
        } else if !regex_postcode.is_match(&postcode) {
            Err("Invalid UK postcode format.".to_string())
        } else {
            Ok(Self {
                house_name,
                house_number,
                address_line_one,
                address_line_two,
                address_line_three,
                address_line_four,
                city,
                postcode,
            })
        }
    }

    pub fn house_name(&self) -> Option<&String> {
        self.house_name.as_ref()
    }

    pub fn house_number(&self) -> Option<&i32> {
        self.house_number.as_ref()
    }

    pub fn address_line_one(&self) -> &str {
        &self.address_line_one
    }

    pub fn address_line_two(&self) -> Option<&String> {
        self.address_line_two.as_ref()
    }

    pub fn address_line_three(&self) -> Option<&String> {
        self.address_line_three.as_ref()
    }

    pub fn address_line_four(&self) -> Option<&String> {
        self.address_line_four.as_ref()
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn postcode(&self) -> &str {
        &self.postcode
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::test_utils::shared::{INDIVIDUAL_FIRST_NAME};
    
    use super::*;

    #[test]
    fn test_valid_name() {
        let name = "JohnDoe".to_string();
        let result = ConstrainedIndividualNameString100::try_from(name);
        assert!(result.is_ok());
        let constrained_name = result.unwrap();
        assert_eq!(constrained_name.value(), "JohnDoe");
    }

    #[test]
    fn test_empty_name() {
        let name = "".to_string();
        let result = ConstrainedIndividualNameString100::try_from(name);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Constrained string 100 characters must have at least one character.");
    }

    #[test]
    fn test_name_with_numbers() {
        let name = "JohnDoe123".to_string();
        let result = ConstrainedIndividualNameString100::try_from(name);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Constrained name string 100 must not have any numbers in it.");
    }

    #[test]
    fn test_name_with_special_characters() {
        let name = "JohnDoe!@#".to_string();
        let result = ConstrainedIndividualNameString100::try_from(name);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Constrained name string 100 must not have any unusual special characters such as ! £ $ % ^ & * () {} \\ / _ + in it.");
    }

    #[test]
    fn test_name_too_long() {
        let name = "a".repeat(101);
        let result = ConstrainedIndividualNameString100::try_from(name);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Constrained string 100 must not have more than 100 characters.");
    }

    #[test]
    fn test_name_with_allowed_special_characters() {
        let name = "John-Doe Example.Name".to_string();
        let result = ConstrainedIndividualNameString100::try_from(name);
        assert!(result.is_ok());
        let constrained_name = result.unwrap();
        assert_eq!(constrained_name.value(), "John-Doe Example.Name");
    }

    #[test]
    fn test_valid_email() {
        let email = "example@example.com".to_string();
        let result = EmailAddress::try_from(email);
        assert!(result.is_ok());
        let email_address = result.unwrap();
        assert_eq!(email_address.value(), "example@example.com");
    }

    #[test]
    fn test_invalid_email_no_at() {
        let email = "example.com".to_string();
        let result = EmailAddress::try_from(email);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid email address format.");
    }

    #[test]
    fn test_invalid_email_no_domain() {
        let email = "example@".to_string();
        let result = EmailAddress::try_from(email);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid email address format.");
    }

    #[test]
    fn test_invalid_email_special_chars() {
        let email = "example@exa!mple.com".to_string();
        let result = EmailAddress::try_from(email);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid email address format.");
    }

    #[test]
    fn test_invalid_email_double_dot() {
        let email = "example@example..com".to_string();
        let result = EmailAddress::try_from(email);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid email address format.");
    }

    

}