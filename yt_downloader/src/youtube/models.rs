use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub playability_status: PlayabilityStatus,
    pub video_details: Option<VideoDetails>,
    pub streaming_data: Option<StreamingData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayabilityStatus {
    pub status: String, // "OK", "UNPLAYABLE", "ERROR" ....
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub video_id: String,
    pub title: String,
    pub author: String,
    pub length_seconds: String, // YouTube sends numbers as strings
    pub thumbnail: Option<Thumbnails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    #[serde(default)]
    pub formats: Vec<Format>, // progressive: audio + video together
    #[serde(default)]
    pub adaptive_formats: Vec<Format>, // separate video only / audio only
    pub hls_manifest_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
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

    pub fn container(&self) -> &str {
        self.mime_type
            .split(";")
            .next()
            .unwrap_or("")
            .split("/")
            .nth(1)
            .unwrap_or("")
    }

    // video/mp4; codecs=\"avc1.640028\"" -> "avc1.640028"

    pub fn codecs(&self) -> &str {
        self.mime_type
            .split("codecs=\"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .unwrap_or("")
    }

    pub fn is_h264_mp4(&self) -> bool {
        self.is_video() && self.container() == "mp4" && self.codecs().starts_with("avc1")
    }

    pub fn is_aac_mp4(&self) -> bool {
        self.is_audio() && self.container() == "mp4" && self.codecs().starts_with("mp4a")
    }
}

#[cfg(test)]
mod tests {
    use super::PlayerResponse;

    #[test]
    fn parses_youtube_camel_case_player_response() {
        let response = serde_json::from_value::<PlayerResponse>(serde_json::json!({
            "playabilityStatus": {"status": "OK"},
            "videoDetails": {
                "videoId": "abc123XYZ_-",
                "title": "Example",
                "author": "Creator",
                "lengthSeconds": "120"
            },
            "streamingData": {
                "formats": [{"itag": 18, "mimeType": "video/mp4"}]
            }
        }))
        .expect("YouTube camelCase fields should deserialize");

        assert_eq!(response.playability_status.status, "OK");
        assert_eq!(
            response.video_details.as_ref().unwrap().video_id,
            "abc123XYZ_-"
        );
        assert_eq!(
            response.streaming_data.as_ref().unwrap().formats[0].itag,
            18
        );
        assert_eq!(
            response.streaming_data.unwrap().formats[0].mime_type,
            "video/mp4"
        );
    }
}
