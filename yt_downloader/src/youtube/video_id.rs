use thiserror::Error;
use tokio::sync::broadcast::error;
use url::Url;

// Validated 11-character YouTube video ID.
// The field is private, so the only way to get a VideoId is through
// validation: an invalid one can't exist.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoId(String);

impl VideoId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn new(candiate: &str) -> Result<Self, ParseError> {
        let valid = candiate.len() == 11
            && candiate
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == "-" || c == "_");

        if valid {
            Ok(VideoId(candiate.to_string()))
        } else {
            Err(ParseError::InvalidId(candiate.to_string()))
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("not a valid URL: {0}")]
    InvalidUrl(String),
    #[error("not a Youtube URL")]
    NotYoutube,
    #[error("no video ID found in the URL")]
    MissingId,
    #[error("invalid video ID: {0}")]
    InvalidId(String),
}

pub fn parse_video_id(input: &str) -> Result<VideoId, ParseError> {
    let input = input.trim();

    // A bare ID like "abc123XYZ_-"
    if !input.contains("/") && !input.contains(".") {
        return VideoId::new(input);
    }

    let with_schema = if input.contains("://") {
        input.to_string()
    } else {
        format!("http://{input}")
    };

    let url = Url::parse(&with_schema).map_err(|e| ParseError::InvalidUrl(e.to_string()))?;
    let host = url.host_str().ok_or(ParseError::NotYoutube)?;
    let host = host.strip_prefix("www.").unwrap_or(host);

    let candidate: Option<String> = match host {
        // https://Youtu.be/<id>
        "youtu.be" => url
            .path_segments()
            .and_then(|mut segments| segments.next())
            .map(str::to_owned),

        "youtube.com" | "m.youtube.com" | "music.youtube.com" => {
            let mut segments = url.path_segments().into_iter().flatten();
            match segments.next() {
                // https://www.youtube.com/watch?v=<id>
                Some("watch") => url
                    .query_pairs()
                    .find(|(key, _)| key == "v")
                    .map(|(_, value)| value.into_owned()),

                // /shorts/<id>, /embed/<id>, /live//<id>
                Some("shorts") | Some("embed") | Some("live") => segments.next().map(str::to_owned),

                _ => None,
            }
        }

        _ => return Err(ParseError::NotYoutube),
    };

    match candidate {
        Some(id) if !id.is_empty() => VideoId::new(&id),
        _ => Err(ParseError::MissingId),
    }
}
