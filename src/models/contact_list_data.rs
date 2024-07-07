/*
 * Phone Backend
 *
 * ## API for managing phones connected to P7M cloud PBX  The purpose of this API is mainly to manage the configuration of phones that use P7M auto provisioning.
 *
 * The version of the OpenAPI document: 0.3.0
 * Contact: tech@p7m.de
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContactListData {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::ContactList>,
}

impl ContactListData {
    pub fn new(data: Vec<crate::models::ContactList>) -> ContactListData {
        ContactListData {
            data,
        }
    }
}


