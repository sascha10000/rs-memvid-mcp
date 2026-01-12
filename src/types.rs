use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// Helper functions for serde defaults
fn default_true() -> bool {
    true
}

fn default_extraction_budget_ms() -> u64 {
    350
}

/// Frame ID type (u64 index in timeline)
pub type FrameId = u64;

/// Document metadata (flexible JSON object)
/// See memvid_core::DocMetadata for full structure including mime, bytes, hash, width, height, colors, caption, exif, audio, media fields
pub type DocMetadata = serde_json::Value;

/// Frame role in the memvid timeline
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum FrameRole {
    /// Standard document frame (default)
    #[default]
    #[schemars(description = "Standard document frame")]
    Document,
    /// Chunk of a larger document
    #[schemars(description = "Chunk of a larger document")]
    DocumentChunk,
    /// Image extracted from document (e.g., PDF page)
    #[schemars(description = "Image extracted from document (e.g., PDF page)")]
    ExtractedImage,
}

impl From<FrameRole> for memvid_core::FrameRole {
    fn from(role: FrameRole) -> Self {
        match role {
            FrameRole::Document => memvid_core::FrameRole::Document,
            FrameRole::DocumentChunk => memvid_core::FrameRole::DocumentChunk,
            FrameRole::ExtractedImage => memvid_core::FrameRole::ExtractedImage,
        }
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateParams {
    #[schemars(description = "Filepath to create the memory file at")]
    pub filepath: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PutBytesParams {
    #[schemars(description = "Raw binary data to put into memvid memory")]
    pub data: String,
    /// Options for putting data into memvid memory
    #[schemars(description = "Options for putting data into memvid memory")]
    pub options: Option<PutOptions>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PutOptions {
    #[schemars(
        description = "Unix timestamp in milliseconds since epoch for the frame. If not provided, current time is used."
    )]
    pub timestamp: Option<i64>,

    #[schemars(
        description = "Track identifier for organizing related frames (e.g., 'project-alpha', 'meeting-notes'). Optional grouping mechanism."
    )]
    pub track: Option<String>,

    #[schemars(
        description = "Content type classification (e.g., 'note', 'document', 'code', 'email'). Used for filtering and organization."
    )]
    pub kind: Option<String>,

    #[schemars(
        description = "Unique resource identifier for the frame. Auto-generated if not provided. Should be unique within the memory."
    )]
    pub uri: Option<String>,

    #[schemars(
        description = "Human-readable title for the frame. Displayed in search results and used for relevance ranking."
    )]
    pub title: Option<String>,

    #[serde(default)]
    #[schemars(
        description = "Rich metadata object containing MIME type, file size, dimensions, EXIF data, audio metadata, etc. See DocMetadata type for full structure."
    )]
    pub metadata: Option<DocMetadata>,

    #[serde(default)]
    #[schemars(
        description = "Override text used for search indexing. If not provided, text is extracted from content automatically."
    )]
    pub search_text: Option<String>,

    #[serde(default)]
    #[schemars(
        description = "List of tags for categorization (e.g., ['urgent', 'personal', 'work']). Used for filtering and faceted search."
    )]
    pub tags: Vec<String>,

    #[serde(default)]
    #[schemars(
        description = "List of labels for classification (e.g., ['invoice', 'receipt', 'contract']). Similar to tags but typically more formal."
    )]
    pub labels: Vec<String>,

    #[serde(default)]
    #[schemars(
        description = "Additional key-value metadata pairs for custom fields. Stored as strings and indexed for search."
    )]
    pub extra_metadata: BTreeMap<String, String>,

    #[serde(default)]
    #[schemars(
        description = "Generate vector embeddings for semantic search. Default: false. Enables similarity search but increases ingestion time."
    )]
    pub enable_embedding: bool,

    #[serde(default = "default_true")]
    #[schemars(
        description = "Automatically extract and assign tags from content. Default: true. Uses NLP to identify key topics and entities."
    )]
    pub auto_tag: bool,

    #[serde(default = "default_true")]
    #[schemars(
        description = "Extract and index temporal references from text. Default: true. Enables time-based queries and timeline views."
    )]
    pub extract_dates: bool,

    #[serde(default = "default_true")]
    #[schemars(
        description = "Extract subject-predicate-object triplets for knowledge graph. Default: true. Enables entity lookups and graph queries."
    )]
    pub extract_triplets: bool,

    #[serde(default)]
    #[schemars(
        description = "Frame ID of parent frame. Used for hierarchical relationships (e.g., image extracted from PDF page)."
    )]
    pub parent_id: Option<FrameId>,

    #[serde(default)]
    #[schemars(
        description = "Role of frame in the timeline. Options: 'document' (default), 'document_chunk', 'extracted_image'. Affects how frame is processed."
    )]
    pub role: FrameRole,

    #[serde(default)]
    #[schemars(
        description = "Skip storing raw binary content, only store extracted text and hash. Default: false. Saves storage space but loses original file."
    )]
    pub no_raw: bool,

    #[serde(default)]
    #[schemars(
        description = "Original file path for reference tracking. Used with no_raw to maintain link to original source file."
    )]
    pub source_path: Option<String>,

    #[serde(default)]
    #[schemars(
        description = "Skip ingestion if frame with matching hash already exists. Default: false. Returns existing frame ID instead of creating duplicate."
    )]
    pub dedup: bool,

    #[serde(default = "default_true")]
    #[schemars(
        description = "Make frame searchable immediately (<1s) with soft commit. Default: true. Full enrichment happens in background."
    )]
    pub instant_index: bool,

    #[serde(default = "default_extraction_budget_ms")]
    #[schemars(
        description = "Time budget for text extraction in milliseconds. Default: 350. Set to 0 for unlimited extraction time."
    )]
    pub extraction_budget_ms: u64,
}

impl PutOptions {
    /// Convert to memvid_core::PutOptions for use with Memvid API
    pub fn to_memvid_options(&self) -> memvid_core::PutOptions {
        memvid_core::PutOptions {
            timestamp: self.timestamp,
            track: self.track.clone(),
            kind: self.kind.clone(),
            uri: self.uri.clone(),
            title: self.title.clone(),
            metadata: self
                .metadata
                .as_ref()
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            search_text: self.search_text.clone(),
            tags: self.tags.clone(),
            labels: self.labels.clone(),
            extra_metadata: self.extra_metadata.clone(),
            enable_embedding: self.enable_embedding,
            auto_tag: self.auto_tag,
            extract_dates: self.extract_dates,
            extract_triplets: self.extract_triplets,
            parent_id: self.parent_id,
            role: self.role.into(),
            no_raw: self.no_raw,
            source_path: self.source_path.clone(),
            dedup: self.dedup,
            instant_index: self.instant_index,
            extraction_budget_ms: self.extraction_budget_ms,
        }
    }
}
