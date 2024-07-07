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
pub struct ContactEntry {
    #[serde(rename = "contactEntryId")]
    pub contact_entry_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "contactListId")]
    pub contact_list_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "officeNumber")]
    pub office_number: String,
    #[serde(rename = "mobileNumber")]
    pub mobile_number: String,
    #[serde(rename = "otherNumber")]
    pub other_number: String,
    #[serde(rename = "line")]
    pub line: i32,
    #[serde(rename = "autoDivert")]
    pub auto_divert: String,
    #[serde(rename = "lastChange")]
    pub last_change: String,
}

impl ContactEntry {
    pub fn new(contact_entry_id: String, tenant_id: String, contact_list_id: String, display_name: String, office_number: String, mobile_number: String, other_number: String, line: i32, auto_divert: String, last_change: String) -> ContactEntry {
        ContactEntry {
            contact_entry_id,
            tenant_id,
            contact_list_id,
            display_name,
            office_number,
            mobile_number,
            other_number,
            line,
            auto_divert,
            last_change,
        }
    }
}


