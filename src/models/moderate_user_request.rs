/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ModerateUserRequest {
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "moderated")]
    pub moderated: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::PlayerModerationType,
}

impl ModerateUserRequest {
    pub fn new(moderated: String, r#type: crate::models::PlayerModerationType) -> ModerateUserRequest {
        ModerateUserRequest {
            moderated,
            r#type,
        }
    }
}


