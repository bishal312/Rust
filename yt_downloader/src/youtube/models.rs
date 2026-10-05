use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlayerResponse {
    pub playability_status: PlayabilityStatus,
    pub video_details: Option<VideoDetails>,
    pub streaming_data: Option<StreamingData>,
}

#[derive(Debug, Deserialize)]
pub struct PlayabilityStatus {
    pub status: String, // "OK", "UNPLAYABLE", "ERROR" ....
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub video_id: String,
    pub title: String,
    pub author: String,
    pub length_seconds: String, // YouTube sends numbers as strings
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    #[serde(default)]
    pub formats: Vec<Format>, // progressive: audio + video together
    #[serde(default)]
    pub adaptive_formats: Vec<Format>, // separate video only / audio only
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub itag: u32,
    pub url: Option<String>,
    pub signature_cipher: Option<String>,
    pub mime_type: String,
    pub bitrate: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub quality_label: Option<String>,
    pub content_length: Option<String>,
}

impl Format {
    pub fn is_video(&self) -> bool {
        self.mime_type.starts_with("video/")
    }

    pub fn is_audio(&self) -> bool {
        self.mime_type.starts_with("audio/")
    }

    pub fn content_length(&self) -> Option<u64> {
        self.content_length.as_ref()?.parse().ok()
    }
}
