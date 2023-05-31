/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

/// LimitedUnityPackage : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LimitedUnityPackage {
    /// This can be `standalonewindows` or `android`, but can also pretty much be any random Unity verison such as `2019.2.4-801-Release` or `2019.2.2-772-Release` or even `unknownplatform`.
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "unityVersion")]
    pub unity_version: String,
}

impl LimitedUnityPackage {
    /// 
    pub fn new(platform: String, unity_version: String) -> LimitedUnityPackage {
        LimitedUnityPackage {
            platform,
            unity_version,
        }
    }
}


