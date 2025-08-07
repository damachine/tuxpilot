use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::{json, Value};

use crate::config::{Config, AiProvider};
use crate::error_diagnosis::ErrorDiagnostic;

#[derive(Clone)]
pub struct AiClient {
    client: Client,
    config: Config,
    use_local: bool,
}

impl AiClient {
    pub async fn new(config: &Config, use_local: bool) -> Result<Self> {
        // Check for test mode timeout
        let timeout_secs = if let Ok(timeout_str) = std::env::var("TUXPILOT_AI_TIMEOUT") {
            timeout_str.parse().unwrap_or(30)
        } else {
            30
        };

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build()?;
        
        Ok(Self {
            client,
            config: config.clone(),
            use_local,
        })
    }

    pub async fn process_query(&self, query: &str) -> Result<String> {
        let system_prompt = self.get_system_prompt();
        let user_prompt = format!("User query: {}", query);
        
        self.send_request(&system_prompt, &user_prompt).await
    }

    pub async fn analyze_diagnostic(&self, diagnostic: &ErrorDiagnostic) -> Result<String> {
        let system_prompt = self.get_diagnostic_system_prompt();
        let user_prompt = format!(
            "Error Analysis Request:\n\
             Error: {}\n\
             Context: {}\n\
             System Info: {}\n\
             Recent Logs: {}",
            diagnostic.error_message,
            diagnostic.context,
            diagnostic.system_info,
            diagnostic.relevant_logs.join("\n")
        );
        
        self.send_request(&system_prompt, &user_prompt).await
    }

    pub async fn get_command_help(&self, command: &str) -> Result<String> {
        let system_prompt = "You are a Linux command expert. Provide clear, practical help for Linux commands.";
        let user_prompt = format!("Explain the '{}' command, its common usage, and provide practical examples.", command);
        
        self.send_request(system_prompt, &user_prompt).await
    }

    pub async fn analyze_system_status(&self, status: &str) -> Result<String> {
        let system_prompt = "You are a Linux system administrator. Analyze system status and provide insights.";
        let user_prompt = format!("Analyze this system status and provide recommendations:\n{}", status);
        
        self.send_request(system_prompt, &user_prompt).await
    }

    pub async fn get_package_advice(&self, operation: &str, package: Option<&str>, suggestion: &str) -> Result<String> {
        let system_prompt = "You are a Linux package management expert. Provide safe, accurate package management advice.";
        let user_prompt = format!(
            "Package operation: {}\nPackage: {:?}\nSuggested command: {}\nProvide advice and explanation.",
            operation, package, suggestion
        );
        
        self.send_request(system_prompt, &user_prompt).await
    }

    pub async fn get_service_advice(&self, service: &str, action: Option<&str>, info: &str) -> Result<String> {
        let system_prompt = "You are a Linux service management expert. Provide clear guidance for service operations.";
        let user_prompt = format!(
            "Service: {}\nAction: {:?}\nService Info: {}\nProvide guidance and next steps.",
            service, action, info
        );
        
        self.send_request(system_prompt, &user_prompt).await
    }

    async fn send_request(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        if self.use_local {
            self.send_local_request(system_prompt, user_prompt).await
        } else {
            match self.config.ai.provider {
                AiProvider::OpenAI => self.send_openai_request(system_prompt, user_prompt).await,
                AiProvider::Anthropic => self.send_anthropic_request(system_prompt, user_prompt).await,
                AiProvider::Local => self.send_local_request(system_prompt, user_prompt).await,
                AiProvider::Ollama => self.send_ollama_request(system_prompt, user_prompt).await,
            }
        }
    }

    async fn send_openai_request(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let openai_config = self.config.ai.openai.as_ref()
            .context("OpenAI configuration not found")?;

        let base_url = openai_config.base_url.as_deref()
            .unwrap_or("https://api.openai.com/v1");

        let payload = json!({
            "model": openai_config.model,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_prompt}
            ],
            "temperature": 0.7,
            "max_tokens": 1000
        });

        let response = self.client
            .post(&format!("{}/chat/completions", base_url))
            .header("Authorization", format!("Bearer {}", openai_config.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .context("Failed to send request to OpenAI")?;

        let response_json: Value = response.json().await
            .context("Failed to parse OpenAI response")?;

        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .context("Invalid response format from OpenAI")?;

        Ok(content.to_string())
    }

    async fn send_anthropic_request(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let anthropic_config = self.config.ai.anthropic.as_ref()
            .context("Anthropic configuration not found")?;

        let payload = json!({
            "model": anthropic_config.model,
            "max_tokens": 1000,
            "system": system_prompt,
            "messages": [
                {"role": "user", "content": user_prompt}
            ]
        });

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &anthropic_config.api_key)
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .json(&payload)
            .send()
            .await
            .context("Failed to send request to Anthropic")?;

        let response_json: Value = response.json().await
            .context("Failed to parse Anthropic response")?;

        let content = response_json["content"][0]["text"]
            .as_str()
            .context("Invalid response format from Anthropic")?;

        Ok(content.to_string())
    }

    async fn send_local_request(&self, _system_prompt: &str, _user_prompt: &str) -> Result<String> {
        // Placeholder for local AI implementation
        // This would integrate with candle-core or similar for local inference
        Ok("Local AI model not yet implemented. Please use cloud providers for now.".to_string())
    }

    async fn send_ollama_request(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let ollama_config = self.config.ai.ollama.as_ref()
            .context("Ollama configuration not found")?;

        let combined_prompt = format!("{}\n\nUser: {}", system_prompt, user_prompt);

        let payload = json!({
            "model": ollama_config.model,
            "prompt": combined_prompt,
            "stream": false,
            "options": {
                "temperature": ollama_config.temperature,
                "num_ctx": ollama_config.context_size
            }
        });

        let client = self.client.clone();
        let url = format!("{}/api/generate", ollama_config.base_url);

        let response = tokio::time::timeout(
            std::time::Duration::from_secs(ollama_config.timeout_seconds),
            client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
        )
        .await
        .context("Ollama request timed out")?
        .context("Failed to send request to Ollama")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Ollama API error {}: {}. Is Ollama running? Try: ollama serve",
                status, error_text
            ));
        }

        let response_json: Value = response.json().await
            .context("Failed to parse Ollama response")?;

        let content = response_json["response"]
            .as_str()
            .context("Invalid response format from Ollama")?;

        Ok(content.to_string())
    }

    fn get_system_prompt(&self) -> String {
        let ai_info = match self.config.ai.provider {
            AiProvider::Ollama => "Du läufst lokal mit Ollama - keine Cloud-Verbindung nötig!",
            _ => "Du läufst mit Cloud-AI",
        };

        format!(
            "Du bist TuxPilot, ein AI-Assistent spezialisiert auf Linux-Systemadministration und Fehlerbehebung. \
             {} Du läufst auf einem Linux-System mit {} Paketmanager und {} Service-Manager. \
             Gib praktische, genaue und sichere Ratschläge. Erkläre Befehle immer bevor du sie vorschlägst. \
             Fokussiere auf Arch Linux Best Practices wenn anwendbar. Antworte auf Deutsch.",
            ai_info,
            format!("{:?}", self.config.system.package_manager),
            format!("{:?}", self.config.system.service_manager)
        )
    }

    fn get_diagnostic_system_prompt(&self) -> String {
        "You are a Linux system diagnostic expert. Analyze errors, logs, and system information to provide \
         clear explanations of problems and step-by-step solutions. Always prioritize system safety and \
         provide multiple solution options when possible.".to_string()
    }
}
