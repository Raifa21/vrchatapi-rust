/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */

/// UserState : * \"online\" User is online in VRChat * \"active\" User is online, but not in VRChat * \"offline\" User is offline  Always offline when returned through `getCurrentUser` (/auth/user).

/// * \"online\" User is online in VRChat * \"active\" User is online, but not in VRChat * \"offline\" User is offline  Always offline when returned through `getCurrentUser` (/auth/user).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserState {
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "online")]
    Online,

}

impl ToString for UserState {
    fn to_string(&self) -> String {
        match self {
            Self::Offline => String::from("offline"),
            Self::Active => String::from("active"),
            Self::Online => String::from("online"),
        }
    }
}




