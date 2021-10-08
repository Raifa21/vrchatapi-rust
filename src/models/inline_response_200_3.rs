/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2003 {
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "nextPartNumber")]
    pub next_part_number: f32,
    #[serde(rename = "maxParts")]
    pub max_parts: f32,
    #[serde(rename = "parts")]
    pub parts: Vec<serde_json::Value>,
    /// Unknown
    #[serde(rename = "etags")]
    pub etags: Vec<serde_json::Value>,
}

impl InlineResponse2003 {
    pub fn new(upload_id: String, file_name: String, next_part_number: f32, max_parts: f32, parts: Vec<serde_json::Value>, etags: Vec<serde_json::Value>) -> InlineResponse2003 {
        InlineResponse2003 {
            upload_id,
            file_name,
            next_part_number,
            max_parts,
            parts,
            etags,
        }
    }
}


