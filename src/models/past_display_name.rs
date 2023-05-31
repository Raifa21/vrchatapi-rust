/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PastDisplayName {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl PastDisplayName {
    pub fn new(display_name: String, updated_at: String) -> PastDisplayName {
        PastDisplayName {
            display_name,
            updated_at,
        }
    }
}


