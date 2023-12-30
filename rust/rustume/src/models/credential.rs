use serde::Serialize; // Make sure to add this if you're using Serde for serialization

#[derive(Serialize)]
pub struct CredentialApproval {
    approved: bool,
    admin: bool,
}

impl  CredentialApproval {
    pub fn new(valid:bool, is_admin:bool) ->  CredentialApproval{
        if valid {
            CredentialApproval{approved: false ,  admin:false}
        }
        else if is_admin {
            CredentialApproval{approved: true ,  admin:true}
        }
        else {
            CredentialApproval{approved: true ,  admin:false}
        }
    }
}