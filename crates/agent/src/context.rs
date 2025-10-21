use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AgentContext {
    pub user_preferences: HashMap<String, String>,
    pub platform_configs: HashMap<String, PlatformConfig>,
    pub content_templates: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    pub platform: String,
    pub configured: bool,
    pub preferences: HashMap<String, String>,
}

impl AgentContext {
    pub fn new() -> Self {
        Self {
            user_preferences: HashMap::new(),
            platform_configs: HashMap::new(),
            content_templates: HashMap::new(),
        }
    }

    pub fn set_user_preference(&mut self, key: String, value: String) {
        self.user_preferences.insert(key, value);
    }

    pub fn get_user_preference(&self, key: &str) -> Option<&String> {
        self.user_preferences.get(key)
    }

    pub fn configure_platform(&mut self, platform: String, config: PlatformConfig) {
        self.platform_configs.insert(platform, config);
    }

    pub fn get_platform_config(&self, platform: &str) -> Option<&PlatformConfig> {
        self.platform_configs.get(platform)
    }

    pub fn add_content_template(&mut self, name: String, template: String) {
        self.content_templates.insert(name, template);
    }

    pub fn get_content_template(&self, name: &str) -> Option<&String> {
        self.content_templates.get(name)
    }
}
