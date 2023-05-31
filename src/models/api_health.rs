/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiHealth {
    #[serde(rename = "ok")]
    pub ok: bool,
    #[serde(rename = "serverName")]
    pub server_name: String,
    #[serde(rename = "buildVersionTag")]
    pub build_version_tag: String,
}

impl ApiHealth {
    pub fn new(ok: bool, server_name: String, build_version_tag: String) -> ApiHealth {
        ApiHealth {
            ok,
            server_name,
            build_version_tag,
        }
    }
}


