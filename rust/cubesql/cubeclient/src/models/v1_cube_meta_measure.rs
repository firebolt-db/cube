/*
 * Cube.js
 *
 * Cube.js Swagger Schema
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1CubeMetaMeasure {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "aggType", skip_serializing_if = "Option::is_none")]
    pub agg_type: Option<String>,
}

impl V1CubeMetaMeasure {
    pub fn new(name: String, r#type: String) -> V1CubeMetaMeasure {
        V1CubeMetaMeasure {
            name,
            title: None,
            description: None,
            r#type,
            agg_type: None,
        }
    }
}
