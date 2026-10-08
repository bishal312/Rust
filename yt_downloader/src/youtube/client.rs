use serde_json::{Value, json};

// Everything that identifies us to youtube as a paritcular app.
pub struct ClientProfile {
    pub name: &'static str,
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
            name: "android_vr",
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

    pub fn ios() -> Self {
        ClientProfile {
            name: "ios",
            client_id: 5,
            client_version: "20.10.4",
            user_agent: "com.google.ios.youtube/20.10.4 (iPhone16,2; U; CPU iOS 18_3_2 like Mac OS X;)",
            context: json!({
                "client": {
                    "clientName": "IOS",
                    "clientVersion": "20.10.4",
                    "deviceMake": "Apple",
                    "deviceModel": "iPhone16,2",
                    "osName": "iPhone",
                    "osVersion": "18.3.2.22D82",
                    "hl": "en",
                    "gl": "US",
                }
            }),
        }
    }
}
