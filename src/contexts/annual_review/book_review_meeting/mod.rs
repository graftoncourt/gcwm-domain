/// workflow "Book Review Meeting" = 
///     input:
///         UnvalidatedAnnualReviewInformation
///         
///         UnvalidatedLastAnnualReviewDate
///         UnvalidatedClientAnnualReviewMeetingInformation
///     Output:
///         SendElectronicAnnualReviewInvite
///         SendPostAnnualReviewInvite









/// Workflow: Book Review Meeting
/// Triggered by: 
///     2 month prior to annual review due date scheduler - 
/// Primary input: 
///     Annual Review Due Date
///     Last Annual Review Date
///     Client Contact Information
///     
/// 
/// Step 1
/// do ValidatedAnnualReviewDueDate
///     if AnnualReviewDueDate is invalid then:
///         send error message
/// 
/// do ValidatedClientContactInformation
///     if ClientContact is invalid then:
///         send error message
/// 
/// Step 2
/// return:
///     AnnualReviewWorkflowTriggered Event
///     AnnualReviewScheduleMeetingMethod Event
 

/// ClientContactInformation
///     JointIndividualsElectronicContact
///     SingleIndividualElectronicContact
///     JointIndividualsPostContact
///     SingleIndividualPostContact
///     MultipleTrusteesElectronicContact
///     PrimaryTrusteeElectronicContact
///     MultipleTrusteesPostContact
///     PrimaryTrusteePostContact
///     MutlipleDirectorsElectronicContact
///     PrimaryDirectorElectronicContact
///     MutlipleDirectorsPostContact
///     PrimaryDirectorPostContact    

/// AnnualReviewDueDate
///     





///
/// 
/// 
/// Other input: 
///     Clients preferred contact method
///     Advisers calender
/// Output events: 
///     'Review Meeting Booked' Event
///     'Review Meeting Declined In Tax Year' Event
///     'Review Meeting Declined' Event    
/// Side effects:
///     An acknowledgement of the outcome is sent
///     via the clients preferred contact method

mod types;
