pub mod env {
    use std::env;

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
}
