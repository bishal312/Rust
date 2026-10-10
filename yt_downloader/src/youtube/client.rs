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
    pub fn web() -> Self {
        ClientProfile {
            name: "web",
            client_id: 1,
            client_version: "2.20261001.00.00",
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
            context: json!({
                "client": {
                    "clientName": "WEB",
                    "clientVersion": "2.20261001.00.00",
                    "platform": "DESKTOP",
                    "hl": "en",
                    "gl": "US",
                    "browserName": "Chrome",
                    "browserVersion": "131.0.0.0",
                    "osName": "Windows",
                    "osVersion": "10.0",
                }
            }),
        }
    }

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

    pub fn visionos() -> Self {
        ClientProfile {
            name: "visionos",
            client_id: 101,
            client_version: "1.02",
            user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15",
            context: json!({
                    "client": {
                        "clientName": "VISIONOS",
                        "clientVersion": "1.02",
                        "deviceMake": "Apple",
                        "deviceModel": "RealityDevice17,1",
                        "osName": "visionOS",
                        "osVersion": "26.5.230471",
                        "hl": "en",
                        "gl": "US",
                    }
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ClientProfile;

    #[test]
    fn web_profile_uses_desktop_web_identity() {
        let profile = ClientProfile::web();
        assert_eq!(profile.client_id, 1);
        assert_eq!(profile.client_version, "2.20261001.00.00");
        assert_eq!(profile.context["client"]["clientName"], "WEB");
        assert_eq!(profile.context["client"]["platform"], "DESKTOP");
        assert!(profile.user_agent.contains("Windows NT 10.0"));
        assert!(profile.user_agent.contains("Chrome/131.0.0.0"));
    }
}
