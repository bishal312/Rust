use serde::Serialize;
use std::cmp::Reverse;

use super::models::{Format, PlayerResponse};

#[derive(Debug, Serialize)]
pub struct InfoResponse {
    pub Video_id: String,
    pub title: String,
    pub author: String,
    pub duration_seconds: u64,
    pub thumbnail_url: Option<String>,
    pub client: String,
    pub formats: Vec<FormatSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormatSummary {
    pub itag: u32,
    pub kind: &'static str, // video or audio
    pub container: String,
    pub codec: String,
    pub quality: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub bitrate: Option<u64>,
    pub size_mb: Option<f64>,
    // can our future mp4 muxer handle this stream valuue in bool
    pub mp4_muxable: bool,
}

impl FormatSummary {
    fn from_format(f: &Format) -> Option<Self> {
        let kind = if f.is_video() {
            "video"
        } else if f.is_audio() {
            "audio"
        } else {
            return None;
        };

        if f.url.is_none() {
            return None;
        }

        let container = f.container().to_string();
        let codec = f.codecs().to_string();
        let mp4_muxable =
            container == "mp4" && (codec.starts_with("avc1") || codec.starts_with("mp4a"));

        Some(FormatSummary {
            itag: f.itag,
            kind,
            container,
            codec,
            quality: f.quality_label.clone(),
            width: f.width,
            height: f.height,
            bitrate: f.bitrate,
            size_mb: f
                .content_length()
                .map(|b| (b as f64 / 1_048_576.0 * 10.0).round() / 10.0),
            mp4_muxable,
        })
    }
}

impl InfoResponse {
    pub fn from_player(player: &PlayerResponse, client: &str) -> Option<Self> {
        let details = player.video_details.as_ref()?;
        let streaming = player.streaming_data.as_ref()?;

        let mut formats: Vec<FormatSummary> = streaming
            .formats
            .iter()
            .chain(streaming.adaptive_formats.iter())
            .filter_map(FormatSummary::from_format)
            .collect();

        // video first (tallest, then highest bitrate), then audio (highest bitrate)
        formats.sort_by_key(|f| {
            (
                f.kind != "video",
                Reverse(f.height.unwrap_or(0)),
                Reverse(f.bitrate.unwrap_or(0)),
            )
        });

        Some(InfoResponse {
            Video_id: details.video_id.clone(),
            title: details.title.clone(),
            author: details.author.clone(),
            duration_seconds: details.length_seconds.parse().unwrap_or(0),
            thumbnail_url: details
                .thumbnail
                .as_ref()
                .and_then(|t| t.thumbnails.last())
                .map(|t| t.url.clone()),
            client: client.to_string(),
            formats,
        })
    }
}
