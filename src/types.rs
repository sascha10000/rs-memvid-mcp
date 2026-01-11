#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateParams {
    #[schemars(description = "Filepath to create the memory file at")]
    pub filepath: String,
}
