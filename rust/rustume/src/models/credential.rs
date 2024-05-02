use serde::{Serialize, Deserialize}; // Make sure to add this if you're using Serde for serialization,

#[derive(Serialize, Deserialize)]
pub struct CredentialApproval {
    pub approved: bool,
    pub admin: bool,
    pub can_write_reference: Option<i32>,
}

impl  CredentialApproval {
    pub fn new(valid:bool, is_admin:bool, can_resume: Option<i32>, ) ->  CredentialApproval{
        if !valid {
            CredentialApproval{approved: false ,  admin:false, can_write_reference: None}
        }
        else if is_admin {
            CredentialApproval{approved: true ,  admin:true, can_write_reference: can_resume}
        }
        else {
            CredentialApproval{approved: true ,  admin:false, can_write_reference: can_resume}
        }
    }
}