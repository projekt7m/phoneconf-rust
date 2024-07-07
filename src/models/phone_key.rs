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
pub struct PhoneKey {
    #[serde(rename = "phoneKeyId")]
    pub phone_key_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "keyGroup")]
    pub key_group: String,
    #[serde(rename = "groupMember")]
    pub group_member: String,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "line")]
    pub line: i32,
    #[serde(rename = "keyType")]
    pub key_type: crate::models::KeyType,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "locked")]
    pub locked: bool,
    #[serde(rename = "lastChange")]
    pub last_change: String,
}

impl PhoneKey {
    pub fn new(phone_key_id: String, tenant_id: String, key_group: String, group_member: String, label: String, line: i32, key_type: crate::models::KeyType, value: String, locked: bool, last_change: String) -> PhoneKey {
        PhoneKey {
            phone_key_id,
            tenant_id,
            key_group,
            group_member,
            label,
            line,
            key_type,
            value,
            locked,
            last_change,
        }
    }
}


