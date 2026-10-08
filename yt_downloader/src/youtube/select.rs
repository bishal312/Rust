use crate::youtube::models::Format;

pub fn pick_formats(formats: &[Format], max_height: u32) -> Option<(&Format, &Format)> {
    // finding the best video format
    // self exercise
    // let bestVideo = formats
    //     .iter()
    //     .filter(|f| f.container() == Some("mp4") && f.codecs().unwrap_or("").contains("avc1"))
    //     .filter(|f| f.height().unwrap_or(0) <= max_height)
    //     .max_by_key(|f| f.height().unwrap_or(0));

    // // finding best audio
    // let bestAudio = formats
    //     .iter()
    //     .filter(|f| f.container() == Some("mp4") && f.codecs().unwrap_or("").contains("mp4a"))
    //     .max_by_key(|f| f.bitrate().unwrap_or(0));

    // Some((bestVideo?, bestAudio?))

    // correction
    // let best_video = formats
    //     .iter()
    //     .filter(|f| f.url.is_some() && f.is_video())
    //     .filter(|f| f.container() == "mp4" && f.codecs().starts_with("avc1"))
    //     .filter(|f| f.height.unwrap_or(0) <= max_height)
    //     .max_by_key(|f| (f.height.unwrap_or(0), f.bitrate.unwrap_or(0)));

    // let best_audio = formats
    //     .iter()
    //     .filter(|f| f.url.is_some() && f.is_audio())
    //     .filter(|f| f.container() == "mp4" && f.codecs().starts_with("mp4a"))
    //     .max_by_key(|f| f.bitrate.unwrap_or(0));

    // using imported is_h264_mp4() and is_aac_mp4() fn from format in models.rs
    let best_video = formats
        .iter()
        .filter(|f| f.url.is_some() && f.is_h264_mp4())
        .filter(|f| f.height.unwrap_or(0) <= max_height)
        .max_by_key(|f| (f.height.unwrap_or(0), f.bitrate.unwrap_or(0)));

    let best_audio = formats
        .iter()
        .filter(|f| f.url.is_some() && f.is_aac_mp4())
        .max_by_key(|f| f.bitrate.unwrap_or(0));

    Some((best_video?, best_audio?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fmt(itag: u32, mime: &str, height: Option<u32>, bitrate: u64) -> Format {
        Format {
            itag,
            url: Some("https://example.invalid/".to_string()),
            signature_cipher: None,
            mime_type: mime.to_string(),
            bitrate: Some(bitrate),
            width: None,
            height,
            quality_label: None,
            content_length: None,
        }
    }

    const H264: &str = r#"video/mp4; codecs="avc1.640028""#;
    const VP9: &str = r#"video/webm; codecs="vp09.00.50.08""#;
    const AAC: &str = r#"audio/mp4; codecs="mp4a.40.2""#;

    #[test]
    fn picks_tallest_h264_and_ignores_other_codecs() {
        let formats = vec![
            fmt(1, H264, Some(720), 2_000_000),
            fmt(2, H264, Some(1080), 4_000_000),
            fmt(3, VP9, Some(2160), 17_000_000),
            fmt(4, AAC, None, 128_000),
        ];
        let (video, audio) = pick_formats(&formats, 4320).unwrap();
        assert_eq!(video.itag, 2);
        assert_eq!(audio.itag, 4);
    }

    #[test]
    fn respects_max_height() {
        let formats = vec![
            fmt(1, H264, Some(720), 2_000_000),
            fmt(2, H264, Some(1080), 4_000_000),
            fmt(4, AAC, None, 128_000),
        ];
        let (video, _) = pick_formats(&formats, 720).unwrap();
        assert_eq!(video.itag, 1);
    }

    #[test]
    fn same_height_prefers_higher_bitrate() {
        // write this one yourself
        let formats = vec![
            fmt(1, H264, Some(720), 2_000_000),
            fmt(2, H264, Some(1080), 4_000_000),
            fmt(3, H264, Some(1080), 8_000_000),
            fmt(4, AAC, None, 128_000),
            fmt(5, AAC, None, 164_000),
        ];
        let (video, audio) = pick_formats(&formats, 1080).unwrap();

        assert_eq!(video.itag, 3);
        assert_eq!(audio.itag, 5);
    }

    #[test]
    fn returns_none_without_audio() {
        let formats = vec![fmt(1, H264, Some(720), 2_000_000)];
        assert!(pick_formats(&formats, 1080).is_none());
    }

    #[test]
    fn skips_formats_without_url() {
        let mut unavailable_video = fmt(2, H264, Some(1080), 4_000_000);
        unavailable_video.url = None;
        let formats = vec![
            fmt(1, H264, Some(720), 2_000_000),
            unavailable_video,
            fmt(4, AAC, None, 128_000),
        ];

        let (video, _) = pick_formats(&formats, 1080).unwrap();
        assert_eq!(video.itag, 1);
    }
}
