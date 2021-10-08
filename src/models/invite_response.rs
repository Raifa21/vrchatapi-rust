/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InviteResponse {
    #[serde(rename = "responseSlot")]
    pub response_slot: i32,
}

impl InviteResponse {
    pub fn new(response_slot: i32) -> InviteResponse {
        InviteResponse {
            response_slot,
        }
    }
}


