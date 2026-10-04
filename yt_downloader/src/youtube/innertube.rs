use anyhow::{Context, Result};
use serde_json::{Value, json};

use super::{client::ClientProfile, video_id::VideoId};

const PLAYER_URL: &str = "https://www.youtube.com/youtubei/v1/player?prettyPrint=false";

// Returns the raw JSON first. looking real responses
// before deciding what our sturcts should contain.

pub async fn fetch_player_raw(
    http: &reqwest::Client,
    video_id: &VideoId,
    client: &ClientProfile,
) -> Result<Value> {
    let body = json!({
        "context": client.context,
        "videoId": video_id.as_str(),
        "contentCheckOk": true,
        "racyCheckOk": true,
    });

    http.post(PLAYER_URL)
        .header("User-Agent", client.user_agent)
        .header("X-Youtube-Client-Name", client.client_id.to_string())
        .header("X-YouTube-Clinet-Version", client.client_version)
        .json(&body)
        .send()
        .await
        .context("request to /player failed")?
        .error_for_status()
        .context("YouTube answered with an error status")?
        .json::<Value>()
        .await
        .context("response was not valid JSON")
}
