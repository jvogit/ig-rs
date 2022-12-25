use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IGAndroidDevice {
    pub user_agent: String,
    pub device_id: String,
    pub capabilities: String,
}

impl IGAndroidDevice {
    pub fn new(seed: &str) -> Self {
        let IG_VERSION = "148.0.0.33.121";
        let DEVICE_AGENT = "23/6.0.1; 640dpi; 1440x2560; samsung; SM-G935F; hero2lte; samsungexynos8890";
        let user_agent = format!("Instagram {IG_VERSION} Android ({DEVICE_AGENT}; en_US)");
        
        IGAndroidDevice { 
            user_agent: user_agent.to_string(), 
            device_id: IGAndroidDevice::gen_android_device_id(seed),
            capabilities: "3brTvw==".to_string(),
        }
    }

    fn gen_android_device_id(seed: &str) -> String {
        let trunc_digest = &format!("{:x}", md5::compute(seed))[..16];

        format!("android-{trunc_digest}")
    }
}
