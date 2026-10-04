use serde_json::{Value, json};

// Everything that identifies us to youtube as a paritcular app.
pub struct ClientProfile {
    pub client_id: u32,
    pub client_version: &'static str,
    pub user_agent: &'static str,
    pub context: Value,
}

impl ClientProfile {
    // Values mirrored from yt-dlp's INNERTUBE_CLIENTS. They go stale:
    // When requests start failing, this is the first place to look.
    pub fn android_vr() -> Self {
        ClientProfile {
            client_id: 28,
            client_version: "1.65.10",
            user_agent: "com.google.android.apps.youtube.vr.oculus/1.65.10 (Linux; U; Android 12L; eureka-user Build/SQ3A.220605.009.A1) gzip",
            context: json!({
                "client": {
                    "clientName": "ANDROID_VR",
                    "clientVersion": "1.65.10",
                    "deviceMake": "Oculus",
                    "deviceModel": "Quest 3",
                    "androidSdkVersion": 32,
                    "osName": "Android",
                    "osVersion": "12L",
                    "hl": "en",
                    "gl": "US",
                }
            }),
        }
    }
}
