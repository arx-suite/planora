pub mod env_utils {
    use std::env;
    use url::Url;

    #[inline]
    pub fn get_env(key: &str) -> String {
        if let Ok(value) = env::var(key) {
            value
        } else {
            panic!("missing required environment variable: `{key}`");
        }
    }

    #[inline]
    pub fn get_env_optional(key: &str) -> Option<String> {
        env::var(key).ok()
    }

    #[inline]
    pub fn get_env_port(key: &str) -> u16 {
        if let Ok(value) = get_env(key).parse::<u16>() {
            value
        } else {
            panic!("`{key}`: must be a valid number");
        }
    }

    #[inline]
    pub fn get_env_port_optional(key: &str) -> Option<u16> {
        get_env_optional(key).and_then(|value| value.parse::<u16>().ok())
    }

    #[inline]
    pub fn get_env_url(key: &str) -> String {
        let value = get_env(key);

        if let Ok(url) = Url::parse(&value) {
            return url.to_string();
        } else {
            panic!("`{key}`: must be a valid URL");
        }
    }

    #[inline]
    pub fn get_env_url_optional(key: &str) -> Option<String> {
        get_env_optional(key).and_then(|value| match Url::parse(&value) {
            Ok(url) => Some(url.to_string()),
            Err(_) => None,
        })
    }
}
