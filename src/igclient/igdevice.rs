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
            device_id: "android-b479836dc7fffd8c".to_string(),
            capabilities: "3brTvw==".to_string(),
        }
    }
}
