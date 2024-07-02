use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClientId(String);

impl ClientId {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ClientId {
    type Error = &'static str;

    fn try_from(client_id_string: String) -> Result<Self, Self::Error> {
        let regex_uuid = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
        
        if !regex_uuid.is_match(&client_id_string) {
            Err("Client id does not include valid uuid string")
        } else {
            Ok(Self(client_id_string))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConstrainedAddressString100(String);

impl ConstrainedAddressString100 {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ConstrainedAddressString100 {
    type Error = &'static str;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let regex_unusual_characters = Regex::new(r"[!£$%^*{}\\/_]").unwrap();

        if string.len() > 100 {
            Err("Constrained address string 100 must not have more than 100 characters.")
        }  else if regex_unusual_characters.is_match(&string) {
            Err("Constrained address string 100 must not have any unusual special characters such as ! £ $ % ^ * {} \\ / _ in it.")
        } else {
            Ok(Self(string))
        }
    }
}

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
struct Postcode(String);

impl Postcode {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Postcode {

    fn new(postcode: String) -> Result<Self, String> {
        let regex_postcode = Regex::new(r"^[A-Z]{1,2}\d[A-Z\d]? ?\d[A-Z]{2}$").unwrap();

        if postcode.is_empty() {
            Err("Postcode must not be empty.".to_string())
        } else if !regex_postcode.is_match(&postcode) {
            Err("Invalid UK postcode format.".to_string())
        } else {
            Ok(Self(postcode))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PhysicalAddress{
    house_name: Option<ConstrainedAddressString100>,
    house_number: Option<i32>,
    address_line_one: ConstrainedAddressString100,
    address_line_two: Option<ConstrainedAddressString100>,
    address_line_three: Option<ConstrainedAddressString100>,
    address_line_four: Option<ConstrainedAddressString100>,
    city: ConstrainedAddressString100,
    county: Option<ConstrainedAddressString100>,
    postcode: Postcode,
    country: Option<ConstrainedAddressString100>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostalAddress(PhysicalAddress);

impl PostalAddress {
    pub fn new(
        house_name: Option<String>,
        house_number: Option<String>,
        address_line_one: String,
        address_line_two: Option<String>,
        address_line_three: Option<String>,
        address_line_four: Option<String>,
        city: String,
        county: Option<String>,
        postcode: String,
        country: Option<String>,
    ) -> Result<Self, String> {

        let house_name = house_name.map(|hn| ConstrainedAddressString100::try_from(hn)).transpose().map_err(|e| e.to_string())?;
        let house_number = house_number.map(|hn| hn.parse::<i32>()).transpose().map_err(|e| e.to_string())?;
        let address_line_one = ConstrainedAddressString100::try_from(address_line_one).map_err(|e| e.to_string())?;
        let address_line_two = address_line_two.map(|al| ConstrainedAddressString100::try_from(al)).transpose().map_err(|e| e.to_string())?;
        let address_line_three = address_line_three.map(|al| ConstrainedAddressString100::try_from(al)).transpose().map_err(|e| e.to_string())?;
        let address_line_four = address_line_four.map(|al| ConstrainedAddressString100::try_from(al)).transpose().map_err(|e| e.to_string())?;
        let city = ConstrainedAddressString100::try_from(city).map_err(|e| e.to_string())?;
        let county = county.map(|c| ConstrainedAddressString100::try_from(c)).transpose().map_err(|e| e.to_string())?;
        let postcode = Postcode::new(postcode).map_err(|e| e.to_string())?;
        let country = country.map(|c| ConstrainedAddressString100::try_from(c)).transpose().map_err(|e| e.to_string())?;

        Ok(Self(PhysicalAddress {
            house_name,
            house_number,
            address_line_one,
            address_line_two,
            address_line_three,
            address_line_four,
            city,
            county,
            postcode,
            country,
        }))

    }

    pub fn house_name(&self) -> Option<&ConstrainedAddressString100> {
        self.0.house_name.as_ref()
    }

    pub fn house_number(&self) -> Option<&i32> {
        self.0.house_number.as_ref()
    }

    pub fn address_line_one(&self) -> &ConstrainedAddressString100 {
        &self.0.address_line_one
    }

    pub fn address_line_two(&self) -> Option<&ConstrainedAddressString100> {
        self.0.address_line_two.as_ref()
    }

    pub fn address_line_three(&self) -> Option<&ConstrainedAddressString100> {
        self.0.address_line_three.as_ref()
    }

    pub fn address_line_four(&self) -> Option<&ConstrainedAddressString100> {
        self.0.address_line_four.as_ref()
    }

    pub fn city(&self) -> &ConstrainedAddressString100 {
        &self.0.city
    }

    pub fn postcode(&self) -> &Postcode {
        &self.0.postcode
    }

    pub fn country(&self) -> Option<&ConstrainedAddressString100> {
        self.0.country.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConstainedTrustOrCompanyNameString200(String);

impl ConstainedTrustOrCompanyNameString200 {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ConstainedTrustOrCompanyNameString200 {
    type Error = &'static str;

    fn try_from(trust_or_company_name: String) -> Result<Self, Self::Error> {
        let regex_unusual_trust_or_comany_name_characters = Regex::new(r"[!£$%^*(){}\\/]").unwrap();
        
        if trust_or_company_name.is_empty() {
            Err("Constrained string 100 characters must have at least one character.")
        } else if trust_or_company_name.len() > 200 {
            Err("Constrained string 100 must not have more than 100 characters.")
        } else if regex_unusual_trust_or_comany_name_characters.is_match(&trust_or_company_name) {
            Err("Constrained trust or company name string 100 must not have any unusual special characters such as ! £ $ % ^ * () {} \\ / _ in it.")
        } else {
            Ok(Self(trust_or_company_name))
        }
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

    #[test]
    fn test_constrained_trust_or_company_name_string_200_valid() {
        let name = ConstainedTrustOrCompanyNameString200::try_from("Valid Trust or Company Name".to_string());
        assert!(name.is_ok());
    }

    #[test]
    fn test_constrained_trust_or_company_name_string_200_empty() {
        let name = ConstainedTrustOrCompanyNameString200::try_from("".to_string());
        assert!(name.is_err());
        if let Err(err) = name {
            assert_eq!(err, "Constrained string 100 characters must have at least one character.");
        } else {
            panic!("Expected error for empty string");
        }
    }

    #[test]
    fn test_constrained_trust_or_company_name_string_200_too_long() {
        let long_name = "a".repeat(201);
        let name = ConstainedTrustOrCompanyNameString200::try_from(long_name);
        assert!(name.is_err());
        if let Err(err) = name {
            assert_eq!(err, "Constrained string 100 must not have more than 100 characters.");
        } else {
            panic!("Expected error for too long string");
        }
    }

    #[test]
    fn test_constrained_trust_or_company_name_string_200_with_unusual_chars() {
        let name = ConstainedTrustOrCompanyNameString200::try_from("Invalid!Name".to_string());
        assert!(name.is_err());
        if let Err(err) = name {
            assert_eq!(err, "Constrained trust or company name string 100 must not have any unusual special characters such as ! £ $ % ^ * () {} \\ / _ in it.");
        } else {
            panic!("Expected error for name with unusual characters");
        }
    }

    #[test]
    fn test_constrained_trust_or_company_name_string_200_with_allowed_chars() {
        let name = ConstainedTrustOrCompanyNameString200::try_from("Valid Name with - and ' and . and &".to_string());
        assert!(name.is_ok());
    }

    #[test]
    fn test_constrained_trust_or_company_name_string_200_value() {
        let name = ConstainedTrustOrCompanyNameString200::try_from("Valid Trust or Company Name".to_string()).unwrap();
        assert_eq!(name.value(), "Valid Trust or Company Name");
    }

}