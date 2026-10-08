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

    let best_video = formats
        .iter()
        .filter(|f| f.url.is_some() && f.is_video())
        .filter(|f| f.container() == "mp4" && f.codecs().starts_with("avc1"))
        .filter(|f| f.height.unwrap_or(0) <= max_height)
        .max_by_key(|f| (f.height.unwrap_or(0), f.bitrate.unwrap_or(0)));

    let best_audio = formats
        .iter()
        .filter(|f| f.url.is_some() && f.is_audio())
        .filter(|f| f.container() == "mp4" && f.codecs().starts_with("mp4a"))
        .max_by_key(|f| f.bitrate.unwrap_or(0));

    Some((best_video?, best_audio?))
}
