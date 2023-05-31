/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RespondGroupJoinRequest {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl RespondGroupJoinRequest {
    pub fn new() -> RespondGroupJoinRequest {
        RespondGroupJoinRequest {
            action: None,
        }
    }
}


