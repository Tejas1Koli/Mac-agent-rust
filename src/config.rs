use std::env;


pub struct Config {
    pub model: String,
    pub ollama_host: String,
    pub temperature: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: "gemma4:e4b-mlx".to_string(),
            ollama_host: "http://localhost:11434".to_string(),
            temperature: 0.5,
        }
    }
}
 impl Config {
    pub fn from_env() -> Self {
        let mut config = Self::default();
        if let Ok(model) = env::var("MODEL") {
            config.model = model;
        }
        if let Ok(host) = env::var("OLLAMA_HOST") {
            config.ollama_host = host;
        }
        if let Ok(temp) = env::var("TEMPERATURE") {
            if let Ok(t) = temp.parse::<f32>() {
                config.temperature = t;
            }
        }
        config
    }
 }