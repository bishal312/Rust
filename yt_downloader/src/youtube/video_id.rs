use thiserror::Error;
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
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');

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

    const YOUTUBE_HOSTS: &[&str] = &[
        "youtube.com",
        "m.youtube.com",
        "music.youtube.com",
        "youtube-nocookie.com",
    ];
    let candidate: Option<String> = match host {
        // https://Youtu.be/<id>
        "youtu.be" => url
            .path_segments()
            .and_then(|mut segments| segments.next())
            .map(str::to_owned),

        h if YOUTUBE_HOSTS.contains(&h) => {
            let mut segments = url.path_segments().into_iter().flatten();
            match segments.next() {
                // https://www.youtube.com/watch?v=<id>
                Some("watch") => url
                    .query_pairs()
                    .find(|(key, _)| key == "v")
                    .map(|(_, value)| value.into_owned()),

                // /shorts/<id>, /embed/<id>, /live//<id>
                Some("shorts" | "embed" | "live" | "v") => segments.next().map(str::to_owned),

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

#[cfg(test)]
mod tests {
    use super::*;

    const ID: &str = "abc123XYZ_-";

    fn assert_parses(input: &str) {
        assert_eq!(
            parse_video_id(input).unwrap().as_str(),
            ID,
            "input: {input}"
        );
    }

    #[test] // watch urls test
    fn watch_urls() {
        assert_parses("https://www.youtube.com/watch?v=abc123XYZ_-");
        assert_parses("https://youtube.com/watch?v=abc123XYZ_-&t=42s");
        assert_parses("https://m.youtube.com/watch?feature=share&v=abc123XYZ_-");
        assert_parses("https://music.youtube.com/watch?v=abc123XYZ_-");
    }

    #[test] // short links test
    fn short_link() {
        assert_parses("https://youtu.be/abc123XYZ_-");
        assert_parses("https://youtu.be/abc123XYZ_-?si=tracking");
    }

    #[test] // shorts_embed_live
    fn shorts_embed_live() {
        assert_parses("https://www.youtube.com/shorts/abc123XYZ_-");
        assert_parses("https://www.youtube.com/embed/abc123XYZ_-");
        assert_parses("https://www.youtube.com/live/abc123XYZ_-");
    }

    #[test] // bare_id_and_missing_scheme test
    fn bare_id_and_missing_scheme() {
        assert_parses("abc123XYZ_-");
        assert_parses("  abc123XYZ_-  ");
        assert_parses("youtube.com/watch?v=abc123XYZ_-");
    }

    #[test] // nocookie_and_old_v_paths test
    fn nocookie_and_old_v_paths() {
        assert_parses("https://www.youtube-nocookie.com/embed/abc123XYZ_-");
        assert_parses("https://youtube-nocookie.com/embed/abc123XYZ_-");
        assert_parses("https://youtube.com/v/abc123XYZ_-");
        assert_parses("https://youtube.com/v/abc123XYZ_-?version=3");
    }

    #[test] // error test
    fn errors() {
        assert_eq!(
            parse_video_id("https://vimeo.com/12345678901"),
            Err(ParseError::NotYoutube)
        );
        assert_eq!(
            parse_video_id("https://www.youtube.com/watch"),
            Err(ParseError::MissingId)
        );
        assert_eq!(
            parse_video_id("https://youtu.be/"),
            Err(ParseError::MissingId)
        );
        assert!(matches!(
            parse_video_id("https://youtu.be/tooshort"),
            Err(ParseError::InvalidId(_))
        ));
        assert!(parse_video_id("hello world").is_err());
    }
}
