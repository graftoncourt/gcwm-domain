use crate::simple_types::{ConstrainedIndividualNameString100, EmailAddress, PostalAddress};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum ValidationError {
    InvalidName(String),
    InvalidEmail(String),
    InvalidAddress(String),
    EmptyInput(String)
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ValidationError::InvalidName(ref desc) => write!(f, "Invalid name: {}", desc),
            ValidationError::InvalidEmail(ref desc) => write!(f, "Invalid email: {}", desc),
            ValidationError::InvalidAddress(ref desc) => write!(f, "Invalid address: {}", desc),
            ValidationError::EmptyInput(ref desc) => write!(f, "Empty Input: {}", desc)
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct JointIndividualsElectronicContact {
    primary_contact_first_name: ConstrainedIndividualNameString100,
    individual_two_first_name: ConstrainedIndividualNameString100,
    primary_contact_email_address: EmailAddress,
    individual_two_email_address: EmailAddress,
}

impl JointIndividualsElectronicContact {
    pub fn validate(
        primary_contact_first_name: String,
        individual_two_first_name: String,
        primary_contact_email_address: String,
        individual_two_email_address: String,
    ) -> Result<Self, ValidationError> {
        let primary_contact_first_name = ConstrainedIndividualNameString100::try_from(primary_contact_first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        let individual_two_first_name = ConstrainedIndividualNameString100::try_from(individual_two_first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        let primary_contact_email_address = EmailAddress::try_from(primary_contact_email_address)
            .map_err(|e| ValidationError::InvalidEmail(e.to_string()))?;
        let individual_two_email_address = EmailAddress::try_from(individual_two_email_address)
            .map_err(|e| ValidationError::InvalidEmail(e.to_string()))?;

        Ok(Self {
            primary_contact_first_name,
            individual_two_first_name,
            primary_contact_email_address,
            individual_two_email_address,
        })
    }

    pub fn primary_contact_first_name(&self) -> &ConstrainedIndividualNameString100 {
        &self.primary_contact_first_name
    }

    pub fn individual_two_first_name(&self) -> &ConstrainedIndividualNameString100 {
        &self.individual_two_first_name
    }

    pub fn primary_contact_email_address(&self) -> &EmailAddress {
        &self.primary_contact_email_address
    }

    pub fn individual_two_email_address(&self) -> &EmailAddress {
        &self.primary_contact_email_address
    }
}

// SingleIndividualElectronicContact
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct SingleIndividualElectronicContact {
    first_name: ConstrainedIndividualNameString100,
    email_address: EmailAddress,
}

impl SingleIndividualElectronicContact {
    pub fn validate(first_name: String, email_address: String) -> Result<Self, ValidationError> {
        let first_name = ConstrainedIndividualNameString100::try_from(first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        let email_address = EmailAddress::try_from(email_address)
            .map_err(|e| ValidationError::InvalidEmail(e.to_string()))?;

        Ok(Self { first_name, email_address })
    }

    pub fn first_name(&self) -> &ConstrainedIndividualNameString100 {
        &self.first_name
    }

    pub fn email_address(&self) -> &EmailAddress {
        &self.email_address
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct JointIndividualsPostContact {
    primary_contact_first_name: ConstrainedIndividualNameString100,
    individual_two_first_name: ConstrainedIndividualNameString100,
    postal_address: PostalAddress,
}

impl JointIndividualsPostContact {
    pub fn validate(
        primary_contact_first_name: String,
        individual_two_first_name: String,
        postal_address: PostalAddress,
    ) -> Result<Self, ValidationError> {
        let primary_contact_first_name = ConstrainedIndividualNameString100::try_from(primary_contact_first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        let individual_two_first_name = ConstrainedIndividualNameString100::try_from(individual_two_first_name)
            .map_err(|e| ValidationError::InvalidEmail(e.to_string()))?;

        Ok(Self {
            primary_contact_first_name,
            individual_two_first_name,
            postal_address,
        })
    }

    pub fn primary_contact_first_name(&self) -> &ConstrainedIndividualNameString100 {
        &self.primary_contact_first_name
    }

    pub fn individual_two_first_name(&self) -> &ConstrainedIndividualNameString100 {
        &self.individual_two_first_name
    }

    pub fn postal_address(&self) -> &PostalAddress {
        &self.postal_address
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct SingleIndividualPostContact {
    contact_first_name: ConstrainedIndividualNameString100,
    postal_address: PostalAddress,
}

impl SingleIndividualPostContact {
    pub fn validate(
        contact_first_name: String,
        postal_address: PostalAddress,
    ) -> Result<Self, ValidationError> {
        let contact_first_name = ConstrainedIndividualNameString100::try_from(contact_first_name)
            .map_err(|e| ValidationError::InvalidAddress(e.to_string()))?;

        Ok(Self {
            contact_first_name,
            postal_address,
        })
    }

    pub fn contact_first_name(&self) -> &ConstrainedIndividualNameString100 {
        &self.contact_first_name
    }

    pub fn postal_address(&self) -> &PostalAddress {
        &self.postal_address
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TrusteeElectronicContact {
    first_name: ConstrainedIndividualNameString100,
    email_address: EmailAddress,
}

impl TrusteeElectronicContact {
    pub fn validate(
        first_name: String,
        email_address: String,
    ) -> Result<Self, ValidationError> {
        let first_name = ConstrainedIndividualNameString100::try_from(first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        let email_address = EmailAddress::try_from(email_address)
            .map_err(|e| ValidationError::InvalidEmail(e.to_string()))?;

        Ok(Self {
            first_name,
            email_address,
        })
    }

    pub fn first_name(&self) -> &ConstrainedIndividualNameString100 {
        &self.first_name
    }

    pub fn email_address(&self) -> &EmailAddress {
        &self.email_address
    }
    
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct MultipleTrusteesElectronicContact {
    trustees: Vec<TrusteeElectronicContact>,
}

impl MultipleTrusteesElectronicContact {
    pub fn validate(trustees: Vec<TrusteeElectronicContact>) -> Result<Self, ValidationError> {
        if trustees.len() < 2 {
            Err(ValidationError::EmptyInput("There must be at least two trustees.".to_string()))
        } else {
            Ok(Self { trustees })
        }
    }

    pub fn trustees(&self) -> &Vec<TrusteeElectronicContact> {
        &self.trustees
    }
}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PrimaryTrusteeElectronicContact {
    primary_trustee_first_name: ConstrainedIndividualNameString100,
    primary_trustee_email_address: EmailAddress,
}

impl PrimaryTrusteeElectronicContact {
    pub fn validate(
        primary_trustee_first_name: String,
        primary_trustee_email_address: String,
    ) -> Result<Self, ValidationError> {
        let primary_trustee_first_name = ConstrainedIndividualNameString100::try_from(primary_trustee_first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        let primary_trustee_email_address = EmailAddress::try_from(primary_trustee_email_address)
            .map_err(|e| ValidationError::InvalidEmail(e.to_string()))?;

        Ok(Self {
            primary_trustee_first_name,
            primary_trustee_email_address,
        })
    }

    pub fn primary_trustee_first_name(&self) -> &ConstrainedIndividualNameString100 {
        &self.primary_trustee_first_name
    }

    pub fn primary_trustee_email_address(&self) -> &EmailAddress {
        &self.primary_trustee_email_address
    }
}
    

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrusteePostContact {
    first_name: ConstrainedIndividualNameString100,
    postal_address: PostalAddress,
}

impl TrusteePostContact {
    pub fn validate(
        first_name: String,
        postal_address: PostalAddress,
    ) -> Result<Self, ValidationError> {
        let first_name = ConstrainedIndividualNameString100::try_from(first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;

        Ok(Self {
            first_name,
            postal_address,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct MultipleTrusteesPostContact {
    trustees: Vec<TrusteePostContact>,
}

impl MultipleTrusteesPostContact {
    pub fn validate(trustees: Vec<TrusteePostContact>) -> Result<Self, ValidationError> {
        if trustees.len() < 2 {
            Err(ValidationError::EmptyInput("There must be at least two trustees.".to_string()))
        } else {
            Ok(Self { trustees })
        }
    }

    pub fn trustees(&self) -> &Vec<TrusteePostContact> {
        &self.trustees
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PrimaryTrusteePostContact {
    primary_trustee_first_name: ConstrainedIndividualNameString100,
    primary_trustee_postal_address: PostalAddress,
}

impl PrimaryTrusteePostContact {
    pub fn validate(
        primary_trustee_first_name: String,
        primary_trustee_postal_address: PostalAddress,
    ) -> Result<Self, ValidationError> {
        let primary_trustee_first_name = ConstrainedIndividualNameString100::try_from(primary_trustee_first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;

        Ok(Self {
            primary_trustee_first_name,
            primary_trustee_postal_address,
        })
    }

    pub fn primary_trustee_first_name(&self) -> &ConstrainedIndividualNameString100 {
        &self.primary_trustee_first_name
    }

    pub fn primary_trustee_postal_address(&self) -> &PostalAddress {
        &self.primary_trustee_postal_address
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct DirectorContact {
    first_name: ConstrainedIndividualNameString100,
    email_address: EmailAddress,
}

impl DirectorContact {
    pub fn validate(
        first_name: String,
        email_address: String,
    ) -> Result<Self, ValidationError> {
        let first_name = ConstrainedIndividualNameString100::try_from(first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        let email_address = EmailAddress::try_from(email_address)
            .map_err(|e| ValidationError::InvalidEmail(e.to_string()))?;

        Ok(Self {
            first_name,
            email_address,
        })
    }

    pub fn first_name(&self) -> &ConstrainedIndividualNameString100 {
        &self.first_name
    }

    pub fn email_address(&self) -> &EmailAddress {
        &self.email_address
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct MultipleDirectorsElectronicContact {
    directors: Vec<DirectorContact>,
}

impl MultipleDirectorsElectronicContact {
    pub fn validate(directors: Vec<DirectorContact>) -> Result<Self, ValidationError> {
        if directors.len() < 2 {
            Err(ValidationError::EmptyInput("There must be at least two directors.".to_string()))
        } else {
            Ok(Self { directors })
        }
    }

    pub fn directors(&self) -> &Vec<DirectorContact> {
        &self.directors
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PrimaryDirectorElectronicContact {
    primary_director_first_name: ConstrainedIndividualNameString100,
    primary_director_email_address: EmailAddress,
}

impl PrimaryDirectorElectronicContact {
    pub fn validate(
        primary_director_first_name: String,
        primary_director_email_address: String,
    ) -> Result<Self, ValidationError> {
        let primary_director_first_name = ConstrainedIndividualNameString100::try_from(primary_director_first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        let primary_director_email_address = EmailAddress::try_from(primary_director_email_address)
            .map_err(|e| ValidationError::InvalidEmail(e.to_string()))?;

        Ok(Self {
            primary_director_first_name,
            primary_director_email_address,
        })
    }

    pub fn primary_director_first_name(&self) -> &ConstrainedIndividualNameString100 {
        &self.primary_director_first_name
    }

    pub fn primary_director_email_address(&self) -> &EmailAddress {
        &self.primary_director_email_address
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DirectorPostContact {
    first_name: ConstrainedIndividualNameString100,
    postal_address: PostalAddress,
}

impl DirectorPostContact {
    pub fn validate(
        first_name: String,
        postal_address: PostalAddress,
    ) -> Result<Self, ValidationError> {
        let first_name = ConstrainedIndividualNameString100::try_from(first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;

        Ok(Self {
            first_name,
            postal_address,
        })
    }

    pub fn first_name(&self) -> &ConstrainedIndividualNameString100 {
        &self.first_name
    }

    pub fn postal_address(&self) -> &PostalAddress {
        &self.postal_address
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultipleDirectorsPostContact {
    directors: Vec<DirectorPostContact>,
}

impl MultipleDirectorsPostContact {
    pub fn validate(directors: Vec<DirectorPostContact>) -> Result<Self, ValidationError> {
        if directors.len() < 2 {
            Err(ValidationError::EmptyInput("There must be at least two directors.".to_string()))
        } else {
            Ok(Self { directors })
        }
    }

    pub fn directors(&self) -> &Vec<DirectorPostContact> {
        &self.directors
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimaryDirectorPostContact {
    primary_director_first_name: ConstrainedIndividualNameString100,
    primary_director_postal_address: PostalAddress,
}

impl PrimaryDirectorPostContact {
    pub fn validate(
        primary_director_first_name: String,
        primary_director_postal_address: PostalAddress,
    ) -> Result<Self, ValidationError> {
        let primary_director_first_name = ConstrainedIndividualNameString100::try_from(primary_director_first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;

        Ok( Self {
            primary_director_first_name,
            primary_director_postal_address,
        })
    }

    pub fn primary_director_first_name(&self) -> &ConstrainedIndividualNameString100 {
        &self.primary_director_first_name
    }

    pub fn primary_director_postal_address(&self) -> &PostalAddress {
        &self.primary_director_postal_address
    }
}

enum ClientContactInformation {
    JointIndividualsElectronicContact(JointIndividualsElectronicContact),
    SingleIndividualElectronicContact(SingleIndividualElectronicContact),
    JointIndividualsPostContact(JointIndividualsPostContact),
    SingleIndividualPostContact(SingleIndividualPostContact),
    MultipleTrusteesElectronicContact(MultipleTrusteesElectronicContact),
    PrimaryTrusteeElectronicContact(PrimaryTrusteeElectronicContact),
    MultipleTrusteesPostContact(MultipleTrusteesPostContact),
    PrimaryTrusteePostContact(PrimaryTrusteePostContact),
    MultipleDirectorsElectronicContact(MultipleDirectorsElectronicContact),
    PrimaryDirectorElectronicContact(PrimaryDirectorElectronicContact),
    MultipleDirectorsPostContact(MultipleDirectorsPostContact),
    PrimaryDirectorPostContact(PrimaryDirectorPostContact), 
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::simple_types::{ConstrainedIndividualNameString100, EmailAddress, PostalAddress};
    use regex::Regex;
    use std::convert::TryFrom;

    #[test]
    fn test_joint_individuals_electronic_contact_validation() {
        let primary_name = "John".to_string();
        let individual_two_name = "Jane".to_string();
        let primary_email = "john@example.com".to_string();
        let individual_two_email = "jane@example.com".to_string();

        let result = JointIndividualsElectronicContact::validate(
            primary_name, 
            individual_two_name, 
            primary_email, 
            individual_two_email
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_joint_individuals_electronic_contact_invalid_email() {
        let primary_name = "John".to_string();
        let individual_two_name = "Jane".to_string();
        let primary_email = "invalid-email".to_string();
        let individual_two_email = "jane@example.com".to_string();

        let result = JointIndividualsElectronicContact::validate(
            primary_name, 
            individual_two_name, 
            primary_email, 
            individual_two_email
        );
        assert!(result.is_err());
        if let Err(ValidationError::InvalidEmail(_)) = result {} else { panic!("Expected InvalidEmail error"); }
    }

    #[test]
    fn test_single_individual_electronic_contact_validation() {
        let name = "John".to_string();
        let email = "john@example.com".to_string();

        let result = SingleIndividualElectronicContact::validate(name, email);
        assert!(result.is_ok());
    }

    #[test]
    fn test_single_individual_electronic_contact_invalid_name() {
        let name = "".to_string();
        let email = "john@example.com".to_string();

        let result = SingleIndividualElectronicContact::validate(name, email);
        assert!(result.is_err());
        if let Err(ValidationError::InvalidName(_)) = result {} else { panic!("Expected InvalidName error"); }
    }

    #[test]
    fn test_postal_address_validation() {
        let address = PostalAddress::new(
            Some("House Name".to_string()),
            Some(123),
            "Address Line One".to_string(),
            Some("Address Line Two".to_string()),
            Some("Address Line Three".to_string()),
            Some("Address Line Four".to_string()),
            "City".to_string(),
            "A1 1AA".to_string(),
        );
        assert!(address.is_ok());
    }

    #[test]
    fn test_postal_address_invalid_postcode() {
        let address = PostalAddress::new(
            Some("House Name".to_string()),
            Some(123),
            "Address Line One".to_string(),
            Some("Address Line Two".to_string()),
            Some("Address Line Three".to_string()),
            Some("Address Line Four".to_string()),
            "City".to_string(),
            "Invalid Postcode".to_string(),
        );
        assert!(address.is_err());
        if let Err(ref err) = address {
            assert_eq!(err, "Invalid UK postcode format.");
        } else {
            panic!("Expected Invalid UK postcode format error");
        }
    }

    #[test]
    fn test_joint_individuals_post_contact_validation() {
        let primary_name = "John".to_string();
        let individual_two_name = "Jane".to_string();
        let postal_address = PostalAddress::new(
            Some("House Name".to_string()),
            Some(123),
            "Address Line One".to_string(),
            Some("Address Line Two".to_string()),
            Some("Address Line Three".to_string()),
            Some("Address Line Four".to_string()),
            "City".to_string(),
            "A1 1AA".to_string(),
        ).unwrap();

        let result = JointIndividualsPostContact::validate(
            primary_name, 
            individual_two_name, 
            postal_address
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_single_individual_post_contact_validation() {
        let name = "John".to_string();
        let postal_address = PostalAddress::new(
            Some("House Name".to_string()),
            Some(123),
            "Address Line One".to_string(),
            Some("Address Line Two".to_string()),
            Some("Address Line Three".to_string()),
            Some("Address Line Four".to_string()),
            "City".to_string(),
            "A1 1AA".to_string(),
        ).unwrap();

        let result = SingleIndividualPostContact::validate(name, postal_address);
        assert!(result.is_ok());
    }

    #[test]
    fn test_trustee_electronic_contact_validation() {
        let name = "John".to_string();
        let email = "john@example.com".to_string();

        let result = TrusteeElectronicContact::validate(name, email);
        assert!(result.is_ok());
    }

    #[test]
    fn test_trustee_electronic_contact_invalid_email() {
        let name = "John".to_string();
        let email = "invalid-email".to_string();

        let result = TrusteeElectronicContact::validate(name, email);
        assert!(result.is_err());
        if let Err(ValidationError::InvalidEmail(_)) = result {} else { panic!("Expected InvalidEmail error"); }
    }

    #[test]
    fn test_multiple_trustees_electronic_contact_validation() {
        let trustee1 = TrusteeElectronicContact {
            first_name: ConstrainedIndividualNameString100::try_from("John".to_string()).unwrap(),
            email_address: EmailAddress::try_from("john@example.com".to_string()).unwrap(),
        };

        let trustee2 = TrusteeElectronicContact {
            first_name: ConstrainedIndividualNameString100::try_from("Jane".to_string()).unwrap(),
            email_address: EmailAddress::try_from("jane@example.com".to_string()).unwrap(),
        };

        let result = MultipleTrusteesElectronicContact::validate(vec![trustee1.clone(), trustee2.clone()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trustees(), &vec![trustee1, trustee2]);
    }

    #[test]
    fn test_multiple_trustees_electronic_contact_insufficient_trustees() {
        let trustee1 = TrusteeElectronicContact {
            first_name: ConstrainedIndividualNameString100::try_from("John".to_string()).unwrap(),
            email_address: EmailAddress::try_from("john@example.com".to_string()).unwrap(),
        };

        let result = MultipleTrusteesElectronicContact::validate(vec![trustee1.clone()]);
        assert!(result.is_err());
        if let Err(ValidationError::EmptyInput(ref msg)) = result {
            assert_eq!(msg, "There must be at least two trustees.");
        } else {
            panic!("Expected EmptyInput error");
        }
    }

    #[test]
    fn test_multiple_trustees_electronic_contact_empty() {
        let result = MultipleTrusteesElectronicContact::validate(vec![]);
        assert!(result.is_err());
        if let Err(ValidationError::EmptyInput(ref msg)) = result {
            assert_eq!(msg, "There must be at least two trustees.");
        } else {
            panic!("Expected EmptyInput error");
        }
    }

    #[test]
    fn test_constrained_individual_name_string_100_valid() {
        let name = ConstrainedIndividualNameString100::try_from("John".to_string());
        assert!(name.is_ok());
    }

    #[test]
    fn test_constrained_individual_name_string_100_empty() {
        let name = ConstrainedIndividualNameString100::try_from("".to_string());
        assert!(name.is_err());
        if let Err(err) = name {
            assert_eq!(err, "Constrained string 100 characters must have at least one character.");
        } else {
            panic!("Expected error for empty string");
        }
    }

    #[test]
    fn test_constrained_individual_name_string_100_too_long() {
        let long_name = "a".repeat(101);
        let name = ConstrainedIndividualNameString100::try_from(long_name);
        assert!(name.is_err());
        if let Err(err) = name {
            assert_eq!(err, "Constrained string 100 must not have more than 100 characters.");
        } else {
            panic!("Expected error for too long string");
        }
    }

    #[test]
    fn test_constrained_individual_name_string_100_with_numbers() {
        let name = ConstrainedIndividualNameString100::try_from("John123".to_string());
        assert!(name.is_err());
        if let Err(err) = name {
            assert_eq!(err, "Constrained name string 100 must not have any numbers in it.");
        } else {
            panic!("Expected error for name with numbers");
        }
    }

    #[test]
    fn test_constrained_individual_name_string_100_with_special_chars() {
        let name = ConstrainedIndividualNameString100::try_from("John!@#".to_string());
        assert!(name.is_err());
        if let Err(err) = name {
            assert_eq!(err, "Constrained name string 100 must not have any unusual special characters such as ! £ $ % ^ & * () {} \\ / _ + in it.");
        } else {
            panic!("Expected error for name with special characters");
        }
    }

    #[test]
    fn test_email_address_valid() {
        let email = EmailAddress::try_from("john@example.com".to_string());
        assert!(email.is_ok());
    }

    #[test]
    fn test_email_address_invalid() {
        let email = EmailAddress::try_from("invalid-email".to_string());
        assert!(email.is_err());
        if let Err(err) = email {
            assert_eq!(err, "Invalid email address format.");
        } else {
            panic!("Expected error for invalid email format");
        }
    }

    #[test]
    fn test_primary_trustee_electronic_contact_validation() {
        let name = "John".to_string();
        let email = "john@example.com".to_string();

        let result = PrimaryTrusteeElectronicContact::validate(name, email);
        assert!(result.is_ok());
    }

    #[test]
    fn test_primary_trustee_electronic_contact_invalid_email() {
        let name = "John".to_string();
        let email = "invalid-email".to_string();

        let result = PrimaryTrusteeElectronicContact::validate(name, email);
        assert!(result.is_err());
        if let Err(ValidationError::InvalidEmail(_)) = result {} else { panic!("Expected InvalidEmail error"); }
    }

    #[test]
    fn test_primary_trustee_post_contact_validation() {
        let name = "John".to_string();
        let postal_address = PostalAddress::new(
            Some("House Name".to_string()),
            Some(123),
            "Address Line One".to_string(),
            Some("Address Line Two".to_string()),
            Some("Address Line Three".to_string()),
            Some("Address Line Four".to_string()),
            "City".to_string(),
            "A1 1AA".to_string(),
        ).unwrap();

        let result = PrimaryTrusteePostContact::validate(name, postal_address);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_trustees_post_contact_validation() {
        let trustee1 = TrusteePostContact {
            first_name: ConstrainedIndividualNameString100::try_from("John".to_string()).unwrap(),
            postal_address: PostalAddress::new(
                Some("House Name".to_string()),
                Some(123),
                "Address Line One".to_string(),
                Some("Address Line Two".to_string()),
                Some("Address Line Three".to_string()),
                Some("Address Line Four".to_string()),
                "City".to_string(),
                "A1 1AA".to_string(),
            ).unwrap(),
        };

        let trustee2 = TrusteePostContact {
            first_name: ConstrainedIndividualNameString100::try_from("Jane".to_string()).unwrap(),
            postal_address: PostalAddress::new(
                Some("House Name".to_string()),
                Some(124),
                "Address Line One".to_string(),
                Some("Address Line Two".to_string()),
                Some("Address Line Three".to_string()),
                Some("Address Line Four".to_string()),
                "City".to_string(),
                "A1 1AB".to_string(),
            ).unwrap(),
        };

        let result = MultipleTrusteesPostContact::validate(vec![trustee1.clone(), trustee2.clone()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trustees(), &vec![trustee1, trustee2]);
    }

    #[test]
    fn test_multiple_trustees_post_contact_insufficient_trustees() {
        let trustee1 = TrusteePostContact {
            first_name: ConstrainedIndividualNameString100::try_from("John".to_string()).unwrap(),
            postal_address: PostalAddress::new(
                Some("House Name".to_string()),
                Some(123),
                "Address Line One".to_string(),
                Some("Address Line Two".to_string()),
                Some("Address Line Three".to_string()),
                Some("Address Line Four".to_string()),
                "City".to_string(),
                "A1 1AA".to_string(),
            ).unwrap(),
        };

        let result = MultipleTrusteesPostContact::validate(vec![trustee1.clone()]);
        assert!(result.is_err());
        if let Err(ValidationError::EmptyInput(ref msg)) = result {
            assert_eq!(msg, "There must be at least two trustees.");
        } else {
            panic!("Expected EmptyInput error");
        }
    }

    #[test]
    fn test_multiple_trustees_post_contact_empty() {
        let result = MultipleTrusteesPostContact::validate(vec![]);
        assert!(result.is_err());
        if let Err(ValidationError::EmptyInput(ref msg)) = result {
            assert_eq!(msg, "There must be at least two trustees.");
        } else {
            panic!("Expected EmptyInput error");
        }
    }

    #[test]
    fn test_multiple_directors_electronic_contact_validation() {
        let director1 = DirectorContact {
            first_name: ConstrainedIndividualNameString100::try_from("John".to_string()).unwrap(),
            email_address: EmailAddress::try_from("john@example.com".to_string()).unwrap(),
        };

        let director2 = DirectorContact {
            first_name: ConstrainedIndividualNameString100::try_from("Jane".to_string()).unwrap(),
            email_address: EmailAddress::try_from("jane@example.com".to_string()).unwrap(),
        };

        let result = MultipleDirectorsElectronicContact::validate(vec![director1.clone(), director2.clone()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().directors(), &vec![director1, director2]);
    }

    #[test]
    fn test_multiple_directors_electronic_contact_insufficient_directors() {
        let director1 = DirectorContact {
            first_name: ConstrainedIndividualNameString100::try_from("John".to_string()).unwrap(),
            email_address: EmailAddress::try_from("john@example.com".to_string()).unwrap(),
        };

        let result = MultipleDirectorsElectronicContact::validate(vec![director1.clone()]);
        assert!(result.is_err());
        if let Err(ValidationError::EmptyInput(ref msg)) = result {
            assert_eq!(msg, "There must be at least two directors.");
        } else {
            panic!("Expected EmptyInput error");
        }
    }

    #[test]
    fn test_multiple_directors_electronic_contact_empty() {
        let result = MultipleDirectorsElectronicContact::validate(vec![]);
        assert!(result.is_err());
        if let Err(ValidationError::EmptyInput(ref msg)) = result {
            assert_eq!(msg, "There must be at least two directors.");
        } else {
            panic!("Expected EmptyInput error");
        }
    }

    #[test]
    fn test_multiple_directors_post_contact_validation() {
        let director1 = DirectorPostContact {
            first_name: ConstrainedIndividualNameString100::try_from("John".to_string()).unwrap(),
            postal_address: PostalAddress::new(
                Some("House Name".to_string()),
                Some(123),
                "Address Line One".to_string(),
                Some("Address Line Two".to_string()),
                Some("Address Line Three".to_string()),
                Some("Address Line Four".to_string()),
                "City".to_string(),
                "A1 1AA".to_string(),
            ).unwrap(),
        };

        let director2 = DirectorPostContact {
            first_name: ConstrainedIndividualNameString100::try_from("Jane".to_string()).unwrap(),
            postal_address: PostalAddress::new(
                Some("House Name".to_string()),
                Some(124),
                "Address Line One".to_string(),
                Some("Address Line Two".to_string()),
                Some("Address Line Three".to_string()),
                Some("Address Line Four".to_string()),
                "City".to_string(),
                "A1 1AB".to_string(),
            ).unwrap(),
        };

        let result = MultipleDirectorsPostContact::validate(vec![director1.clone(), director2.clone()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().directors(), &vec![director1, director2]);
    }

    #[test]
    fn test_multiple_directors_post_contact_insufficient_directors() {
        let director1 = DirectorPostContact {
            first_name: ConstrainedIndividualNameString100::try_from("John".to_string()).unwrap(),
            postal_address: PostalAddress::new(
                Some("House Name".to_string()),
                Some(123),
                "Address Line One".to_string(),
                Some("Address Line Two".to_string()),
                Some("Address Line Three".to_string()),
                Some("Address Line Four".to_string()),
                "City".to_string(),
                "A1 1AA".to_string(),
            ).unwrap(),
        };

        let result = MultipleDirectorsPostContact::validate(vec![director1.clone()]);
        assert!(result.is_err());
        if let Err(ValidationError::EmptyInput(ref msg)) = result {
            assert_eq!(msg, "There must be at least two directors.");
        } else {
            panic!("Expected EmptyInput error");
        }
    }

    #[test]
    fn test_multiple_directors_post_contact_empty() {
        let result = MultipleDirectorsPostContact::validate(vec![]);
        assert!(result.is_err());
        if let Err(ValidationError::EmptyInput(ref msg)) = result {
            assert_eq!(msg, "There must be at least two directors.");
        } else {
            panic!("Expected EmptyInput error");
        }
    }

    #[test]
    fn test_primary_director_post_contact_validation() {
        let name = "John".to_string();
        let postal_address = PostalAddress::new(
            Some("House Name".to_string()),
            Some(123),
            "Address Line One".to_string(),
            Some("Address Line Two".to_string()),
            Some("Address Line Three".to_string()),
            Some("Address Line Four".to_string()),
            "City".to_string(),
            "A1 1AA".to_string(),
        ).unwrap();

        let result = PrimaryDirectorPostContact::validate(name, postal_address);
        assert!(result.is_ok());
    }
}