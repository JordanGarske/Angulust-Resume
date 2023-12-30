use serde::Serialize; // Make sure to add this if you're using Serde for serialization

#[derive(Serialize)]
pub struct CredentialApproval {
    approved: bool,
    admin: bool,
    can_write_reference:bool
}

impl  CredentialApproval {
    pub fn new(valid:bool, is_admin:bool,can_write:bool ) ->  CredentialApproval{
        if !valid {
            CredentialApproval{approved: false ,  admin:false, can_write_reference: false}
        }
        else if is_admin {
            CredentialApproval{approved: true ,  admin:true, can_write_reference: can_write}
        }
        else {
            CredentialApproval{approved: true ,  admin:false, can_write_reference: can_write}
        }
    }
}