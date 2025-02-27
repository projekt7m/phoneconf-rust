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
pub struct NewPhone {
    #[serde(rename = "mac")]
    pub mac: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "phoneType")]
    pub phone_type: String,
    #[serde(rename = "dsskeyLength")]
    pub dsskey_length: i32,
    #[serde(rename = "ringType")]
    pub ring_type: String,
    #[serde(rename = "blockOutNumber")]
    pub block_out_number: Vec<String>,
    #[serde(rename = "udpListenPort")]
    pub udp_listen_port: i32,
    #[serde(rename = "tlsListenPort")]
    pub tls_listen_port: i32,
    #[serde(rename = "userPassword", skip_serializing_if = "Option::is_none")]
    pub user_password: Option<String>,
    #[serde(rename = "adminPassword", skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,
    #[serde(rename = "contactListId", skip_serializing_if = "Option::is_none")]
    pub contact_list_id: Option<String>,
}

impl NewPhone {
    pub fn new(mac: String, name: String, phone_type: String, dsskey_length: i32, ring_type: String, block_out_number: Vec<String>, udp_listen_port: i32, tls_listen_port: i32) -> NewPhone {
        NewPhone {
            mac,
            name,
            phone_type,
            dsskey_length,
            ring_type,
            block_out_number,
            udp_listen_port,
            tls_listen_port,
            user_password: None,
            admin_password: None,
            contact_list_id: None,
        }
    }
}


