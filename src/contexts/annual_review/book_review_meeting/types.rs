use crate::{contexts::annual_review, simple_types::{ClientId, ConstainedTrustOrCompanyNameString200, ConstrainedIndividualNameString100, EmailAddress, PostalAddress}};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime};

#[derive(Debug)]
pub enum ValidationError {
    InvalidName(String),
    InvalidEmail(String),
    InvalidAddress(String),
    InvalidDate(String),
    EmptyInput(String),
    InvalidInput(String)
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ValidationError::InvalidName(ref desc) => write!(f, "Invalid name: {}", desc),
            ValidationError::InvalidEmail(ref desc) => write!(f, "Invalid email: {}", desc),
            ValidationError::InvalidAddress(ref desc) => write!(f, "Invalid address: {}", desc),
            ValidationError::InvalidDate(ref desc) => write!(f, "Invalid date: {}", desc),
            ValidationError::EmptyInput(ref desc) => write!(f, "Empty Input: {}", desc),
            ValidationError::InvalidInput(ref desc) => write!(f, "Invalid Input: {}", desc)
        }
    }
}

impl std::error::Error for ValidationError {}

// Unvalidated annual review information input to the domain received as JSON or similar for deserializastion and validation

#[derive(Debug, Serialize, Deserialize)]
pub struct UnvalidatedAnnualReviewInformation {
    annual_review_due_date: String,
    last_annual_review_due_date: String,
    adviser_name: String,
    administrator_email: String,
    client_contact_information: ClientContactInformation
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientContactInformation {
    JointIndividualsElectronicContact(JointIndividualsElectronicContact),
    SingleIndividualElectronicContact(SingleIndividualElectronicContact),
    JointIndividualsPostContact(JointIndividualsPostContact),
    //SingleIndividualPostContact(SingleIndividualPostContact),
    //MultipleTrusteesElectronicContact(MultipleTrusteesElectronicContact),
    //PrimaryTrusteeElectronicContact(PrimaryTrusteeElectronicContact),
    //MultipleTrusteesPostContact(MultipleTrusteesPostContact),
    //PrimaryTrusteePostContact(PrimaryTrusteePostContact),
    //MultipleDirectorsElectronicContact(MultipleDirectorsElectronicContact),
    //PrimaryDirectorElectronicContact(PrimaryDirectorElectronicContact),
    //MultipleDirectorsPostContact(MultipleDirectorsPostContact),
    //PrimaryDirectorPostContact(PrimaryDirectorPostContact), 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JointIndividualsElectronicContact {
    primary_contact_first_name: String,
    individual_two_first_name: String,
    primary_contact_email_address: String,
    individual_two_email_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SingleIndividualElectronicContact {
    first_name: String,
    email_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JointIndividualsPostContact {
    primary_contact_first_name: String,
    individual_two_first_name: String,
    house_name: String,
    house_number: String,
    address_line_one: String,
    address_line_two: String,
    address_line_three: String,
    address_line_four: String,
    city: String,
    county: String,
    postcode: String,
    country: String
}











#[derive(Debug, Serialize)]
struct ValidatedAnnualReviewInformation {
    annual_review_due_date: ValidatedAnnualReviewDueDate,
    client_contact_information: ValidatedClientContactInformation
}

impl ValidatedAnnualReviewInformation {
    pub fn validate(
        unvalidated_annual_review_information: UnvalidatedAnnualReviewInformation
    ) -> Result<Self, ValidationError> {
        let annual_review_due_date = ValidatedAnnualReviewDueDate::validate(
            unvalidated_annual_review_information.annual_review_due_date,
            unvalidated_annual_review_information.last_annual_review_due_date
        ).map_err(|e| ValidationError::InvalidDate(e.to_string()))?;

        let client_contact_information = match unvalidated_annual_review_information.client_contact_information {
            ClientContactInformation::JointIndividualsElectronicContact(information) => {
                ValidatedClientContactInformation::validate_joint_indivduals_electronic_contact(information)
            }
            ClientContactInformation::SingleIndividualElectronicContact(information) => {
                ValidatedClientContactInformation::validate_single_indivdual_electronic_contact(information)
            }
            ClientContactInformation::JointIndividualsPostContact(information) => {
                ValidatedClientContactInformation::validated_joint_individuals_post_contact(information)
            }
        };

        match client_contact_information {
            Ok(client_contact_information) => { Ok(Self{annual_review_due_date, client_contact_information})}
            Err(error) => {Err(error)}
        }
    }
}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct ValidatedAnnualReviewDueDate(NaiveDateTime);

impl ValidatedAnnualReviewDueDate {

    pub fn validate(
        unvalidated_date_string: String,
        last_annual_review_date_string: String,
    ) -> Result<Self, ValidationError> {
        
        let datetime = NaiveDateTime::parse_from_str(&unvalidated_date_string, "%d/%m/%Y")
            .map_err(|e| ValidationError::InvalidDate(e.to_string()))?;

        // Check the date is not in the past

        // Check the annual review date is not greater than 1 year from the previous annual review completion by querying persistence

        // Check the annual review date is within the relevant tax year

        Ok(Self(datetime))

    }
}



#[derive(Debug, Serialize)]
enum ValidatedClientContactInformation {
    ValidatedJointIndividualsElectronicContact(ValidatedJointIndividualsElectronicContact),
    ValidatedSingleIndividualElectronicContact(ValidatedSingleIndividualElectronicContact),
    ValidatedJointIndividualsPostContact(ValidatedJointIndividualsPostContact),
    //SingleIndividualPostContact(SingleIndividualPostContact),
    //MultipleTrusteesElectronicContact(MultipleTrusteesElectronicContact),
    //PrimaryTrusteeElectronicContact(PrimaryTrusteeElectronicContact),
    //MultipleTrusteesPostContact(MultipleTrusteesPostContact),
    //PrimaryTrusteePostContact(PrimaryTrusteePostContact),
    //MultipleDirectorsElectronicContact(MultipleDirectorsElectronicContact),
    //PrimaryDirectorElectronicContact(PrimaryDirectorElectronicContact),
    //MultipleDirectorsPostContact(MultipleDirectorsPostContact),
    //PrimaryDirectorPostContact(PrimaryDirectorPostContact), 
}

impl ValidatedClientContactInformation {
    pub fn validate_joint_indivduals_electronic_contact(
        joint_individuals_electronic_contact: JointIndividualsElectronicContact
    ) -> Result<Self, ValidationError> {
        
        let validated_joint_individuals_electronic_contact = ValidatedJointIndividualsElectronicContact::validate(
            joint_individuals_electronic_contact.primary_contact_first_name, 
            joint_individuals_electronic_contact.individual_two_first_name, 
            joint_individuals_electronic_contact.primary_contact_email_address, 
            joint_individuals_electronic_contact.individual_two_email_address
        );

        match validated_joint_individuals_electronic_contact {
            Ok(valid) => { Ok(Self::ValidatedJointIndividualsElectronicContact(valid))}
            Err(error) => Err(error)
        }

    }

    pub fn validate_single_indivdual_electronic_contact(
        single_individual_electronic_contact: SingleIndividualElectronicContact
    ) -> Result<Self, ValidationError> {

        let validated_single_individual_electronic_contact = ValidatedSingleIndividualElectronicContact::validate(
            single_individual_electronic_contact.first_name, 
            single_individual_electronic_contact.email_address
        );

        match validated_single_individual_electronic_contact {
            Ok(valid) => { Ok(Self::ValidatedSingleIndividualElectronicContact(valid))}
            Err(error) => Err(error)
        }
    }

    pub fn validated_joint_individuals_post_contact(
        joint_individuals_post_contact: JointIndividualsPostContact
    ) -> Result<Self, ValidationError> {

        let validated_joint_individuals_post_contact = ValidatedJointIndividualsPostContact::validate(
            joint_individuals_post_contact.primary_contact_first_name, 
            joint_individuals_post_contact.individual_two_first_name, 
            joint_individuals_post_contact.house_name,
            joint_individuals_post_contact.house_number,
            joint_individuals_post_contact.address_line_one,
            joint_individuals_post_contact.address_line_two,
            joint_individuals_post_contact.address_line_three,
            joint_individuals_post_contact.address_line_four,
            joint_individuals_post_contact.city,
            joint_individuals_post_contact.county,
            joint_individuals_post_contact.postcode,
            joint_individuals_post_contact.country
        );

        match validated_joint_individuals_post_contact {
            Ok(valid) => { Ok(Self::ValidatedJointIndividualsPostContact(valid))}
            Err(error) => Err(error)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct ValidatedJointIndividualsElectronicContact {
    primary_contact_first_name: ConstrainedIndividualNameString100,
    individual_two_first_name: ConstrainedIndividualNameString100,
    primary_contact_email_address: EmailAddress,
    individual_two_email_address: EmailAddress,
}

impl ValidatedJointIndividualsElectronicContact {
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
struct ValidatedSingleIndividualElectronicContact {
    first_name: ConstrainedIndividualNameString100,
    email_address: EmailAddress,
}

impl ValidatedSingleIndividualElectronicContact {
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
struct ValidatedJointIndividualsPostContact {
    primary_contact_first_name: ConstrainedIndividualNameString100,
    individual_two_first_name: ConstrainedIndividualNameString100,
    postal_address: PostalAddress,
}

impl ValidatedJointIndividualsPostContact {
    pub fn validate(
        primary_contact_first_name: String,
        individual_two_first_name: String,
        house_name: String,
        house_number: String,
        address_line_one: String,
        address_line_two: String,
        address_line_three: String,
        address_line_four: String,
        city: String,
        county: String,
        postcode: String,
        country: String,
    ) -> Result<Self, ValidationError> {
        let primary_contact_first_name = ConstrainedIndividualNameString100::try_from(primary_contact_first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        let individual_two_first_name = ConstrainedIndividualNameString100::try_from(individual_two_first_name)
            .map_err(|e| ValidationError::InvalidEmail(e.to_string()))?;
        let postal_address = PostalAddress::new(
            if house_name.is_empty() { None } else { Some(house_name) },
            if house_number.is_empty() { None } else { Some(house_number) },
            address_line_one,
            if address_line_two.is_empty() { None } else { Some(address_line_two) },
            if address_line_three.is_empty() { None } else { Some(address_line_three)},
            if address_line_four.is_empty() { None } else { Some(address_line_four)},
            city,
            if county.is_empty() { None } else { Some(county)},
            postcode,
            if country.is_empty() { None } else { Some(country)},
        ).map_err(|e|ValidationError::InvalidInput(e.to_string()))?;

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
    trust_name: ConstainedTrustOrCompanyNameString200,
    trustees: Vec<TrusteeElectronicContact>,
}

impl MultipleTrusteesElectronicContact {
    pub fn validate(trust_name: ConstainedTrustOrCompanyNameString200, trustees: Vec<TrusteeElectronicContact>) -> Result<Self, ValidationError> {
        
        let trust_name = ConstainedTrustOrCompanyNameString200::try_from(trust_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        
        if trustees.len() < 2 {
            Err(ValidationError::EmptyInput("There must be at least two trustees.".to_string()))
        } else {
            Ok(Self { 
                trust_name,
                trustees 
            })
        }
    }

    pub fn trustees(&self) -> &Vec<TrusteeElectronicContact> {
        &self.trustees
    }
}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PrimaryTrusteeElectronicContact {
    trust_name: ConstainedTrustOrCompanyNameString200,
    primary_trustee_first_name: ConstrainedIndividualNameString100,
    primary_trustee_email_address: EmailAddress,
}

impl PrimaryTrusteeElectronicContact {
    pub fn validate(
        trust_name: String,
        primary_trustee_first_name: String,
        primary_trustee_email_address: String,
    ) -> Result<Self, ValidationError> {
        let trust_name = ConstainedTrustOrCompanyNameString200::try_from(trust_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        let primary_trustee_first_name = ConstrainedIndividualNameString100::try_from(primary_trustee_first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        let primary_trustee_email_address = EmailAddress::try_from(primary_trustee_email_address)
            .map_err(|e| ValidationError::InvalidEmail(e.to_string()))?;

        Ok(Self {
            trust_name,
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
    trust_name: ConstainedTrustOrCompanyNameString200,
    trustees: Vec<TrusteePostContact>,
}

impl MultipleTrusteesPostContact {
    pub fn validate(trust_name: String, trustees: Vec<TrusteePostContact>) -> Result<Self, ValidationError> {
        let trust_name = ConstainedTrustOrCompanyNameString200::try_from(trust_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        
        if trustees.len() < 2 {
            Err(ValidationError::EmptyInput("There must be at least two trustees.".to_string()))
        } else {
            Ok(Self { 
                trust_name,
                trustees 
            })
        }
    }

    pub fn trustees(&self) -> &Vec<TrusteePostContact> {
        &self.trustees
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PrimaryTrusteePostContact {
    trust_name: ConstainedTrustOrCompanyNameString200,
    primary_trustee_first_name: ConstrainedIndividualNameString100,
    primary_trustee_postal_address: PostalAddress,
}

impl PrimaryTrusteePostContact {
    pub fn validate(
        trust_name: String,
        primary_trustee_first_name: String,
        primary_trustee_postal_address: PostalAddress,
    ) -> Result<Self, ValidationError> {
        let trust_name = ConstainedTrustOrCompanyNameString200::try_from(trust_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        
        let primary_trustee_first_name = ConstrainedIndividualNameString100::try_from(primary_trustee_first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;

        Ok(Self {
            trust_name,
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
    company_name: ConstainedTrustOrCompanyNameString200,
    directors: Vec<DirectorContact>,
}

impl MultipleDirectorsElectronicContact {
    pub fn validate(company_name: String, directors: Vec<DirectorContact>) -> Result<Self, ValidationError> {

        let company_name = ConstainedTrustOrCompanyNameString200::try_from(company_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;

        if directors.len() < 2 {
            Err(ValidationError::EmptyInput("There must be at least two directors.".to_string()))
        } else {
            Ok(Self { 
                company_name,
                directors }
            )
        }
    }

    pub fn directors(&self) -> &Vec<DirectorContact> {
        &self.directors
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PrimaryDirectorElectronicContact {
    company_name: ConstainedTrustOrCompanyNameString200,
    primary_director_first_name: ConstrainedIndividualNameString100,
    primary_director_email_address: EmailAddress,
}

impl PrimaryDirectorElectronicContact {
    pub fn validate(
        company_name: String,
        primary_director_first_name: String,
        primary_director_email_address: String,
    ) -> Result<Self, ValidationError> {
        let company_name = ConstainedTrustOrCompanyNameString200::try_from(company_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        let primary_director_first_name = ConstrainedIndividualNameString100::try_from(primary_director_first_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        let primary_director_email_address = EmailAddress::try_from(primary_director_email_address)
            .map_err(|e| ValidationError::InvalidEmail(e.to_string()))?;

        Ok(Self {
            company_name,
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
struct DirectorPostContact {
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
struct MultipleDirectorsPostContact {
    company_name: ConstainedTrustOrCompanyNameString200,
    directors: Vec<DirectorPostContact>,
}

impl MultipleDirectorsPostContact {
    pub fn validate(company_name: String, directors: Vec<DirectorPostContact>) -> Result<Self, ValidationError> {
        let company_name = ConstainedTrustOrCompanyNameString200::try_from(company_name)
            .map_err(|e| ValidationError::InvalidName(e.to_string()))?;
        if directors.len() < 2 {
            Err(ValidationError::EmptyInput("There must be at least two directors.".to_string()))
        } else {
            Ok(Self { 
                company_name,
                directors 
            })
        }
    }

    pub fn directors(&self) -> &Vec<DirectorPostContact> {
        &self.directors
    }
}

#[derive(Debug, Serialize)]
struct PrimaryDirectorPostContact {
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