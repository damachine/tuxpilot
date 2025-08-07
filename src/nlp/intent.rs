use anyhow::Result;
use super::{Intent, IntentCategory};
use std::collections::HashMap;

/// Intent classification system
#[derive(Debug, Clone)]
pub struct IntentClassifier {
    intent_patterns: HashMap<String, IntentPattern>,
}

/// Intent pattern for classification
#[derive(Debug, Clone)]
pub struct IntentPattern {
    pub intent_name: String,
    pub category: IntentCategory,
    pub keywords: Vec<String>,
    pub patterns: Vec<String>,
    pub confidence_threshold: f32,
}

impl IntentClassifier {
    pub async fn new() -> Result<Self> {
        let mut intent_patterns = HashMap::new();

        // System management intents
        intent_patterns.insert("check_system_status".to_string(), IntentPattern {
            intent_name: "check_system_status".to_string(),
            category: IntentCategory::SystemManagement,
            keywords: vec!["status", "system", "health", "check", "running"].iter().map(|s| s.to_string()).collect(),
            patterns: vec![
                "check system status".to_string(),
                "how is the system".to_string(),
                "system health".to_string(),
            ],
            confidence_threshold: 0.7,
        });

        // File operations intents
        intent_patterns.insert("list_files".to_string(), IntentPattern {
            intent_name: "list_files".to_string(),
            category: IntentCategory::FileOperations,
            keywords: vec!["list", "files", "directory", "ls", "show"].iter().map(|s| s.to_string()).collect(),
            patterns: vec![
                "list files".to_string(),
                "show directory".to_string(),
                "what files are here".to_string(),
            ],
            confidence_threshold: 0.8,
        });

        // Process management intents
        intent_patterns.insert("check_processes".to_string(), IntentPattern {
            intent_name: "check_processes".to_string(),
            category: IntentCategory::ProcessManagement,
            keywords: vec!["process", "running", "ps", "kill", "stop"].iter().map(|s| s.to_string()).collect(),
            patterns: vec![
                "check processes".to_string(),
                "what's running".to_string(),
                "show processes".to_string(),
            ],
            confidence_threshold: 0.7,
        });

        // Troubleshooting intents
        intent_patterns.insert("troubleshoot_issue".to_string(), IntentPattern {
            intent_name: "troubleshoot_issue".to_string(),
            category: IntentCategory::Troubleshooting,
            keywords: vec!["problem", "issue", "error", "fix", "broken", "help"].iter().map(|s| s.to_string()).collect(),
            patterns: vec![
                "something is wrong".to_string(),
                "fix this problem".to_string(),
                "help me troubleshoot".to_string(),
            ],
            confidence_threshold: 0.6,
        });

        // Information intents
        intent_patterns.insert("get_information".to_string(), IntentPattern {
            intent_name: "get_information".to_string(),
            category: IntentCategory::Information,
            keywords: vec!["what", "how", "when", "where", "info", "information"].iter().map(|s| s.to_string()).collect(),
            patterns: vec![
                "what is".to_string(),
                "how do I".to_string(),
                "tell me about".to_string(),
            ],
            confidence_threshold: 0.5,
        });

        Ok(Self { intent_patterns })
    }

    pub async fn classify(&self, input: &str) -> Result<Intent> {
        let input_lower = input.to_lowercase();
        let mut best_match: Option<(&IntentPattern, f32)> = None;

        for pattern in self.intent_patterns.values() {
            let confidence = self.calculate_confidence(&input_lower, pattern);
            
            if confidence >= pattern.confidence_threshold {
                if let Some((_, current_confidence)) = best_match {
                    if confidence > current_confidence {
                        best_match = Some((pattern, confidence));
                    }
                } else {
                    best_match = Some((pattern, confidence));
                }
            }
        }

        if let Some((pattern, confidence)) = best_match {
            Ok(Intent {
                name: pattern.intent_name.clone(),
                confidence,
                category: pattern.category.clone(),
                parameters: self.extract_parameters(&input_lower, pattern),
            })
        } else {
            // Default to unknown intent
            Ok(Intent {
                name: "unknown".to_string(),
                confidence: 0.1,
                category: IntentCategory::Unknown,
                parameters: HashMap::new(),
            })
        }
    }

    fn calculate_confidence(&self, input: &str, pattern: &IntentPattern) -> f32 {
        let mut score = 0.0;
        let mut total_keywords = pattern.keywords.len() as f32;

        // Check keyword matches
        for keyword in &pattern.keywords {
            if input.contains(keyword) {
                score += 1.0;
            }
        }

        // Check pattern matches
        for pattern_text in &pattern.patterns {
            if input.contains(pattern_text) {
                score += 2.0; // Pattern matches are worth more
                total_keywords += 2.0;
            }
        }

        if total_keywords > 0.0 {
            score / total_keywords
        } else {
            0.0
        }
    }

    fn extract_parameters(&self, input: &str, _pattern: &IntentPattern) -> HashMap<String, String> {
        let mut parameters = HashMap::new();

        // Simple parameter extraction (in real implementation, would be more sophisticated)
        if input.contains("file") {
            parameters.insert("object_type".to_string(), "file".to_string());
        }
        if input.contains("directory") {
            parameters.insert("object_type".to_string(), "directory".to_string());
        }
        if input.contains("service") {
            parameters.insert("object_type".to_string(), "service".to_string());
        }

        parameters
    }

    pub async fn update_from_feedback(&mut self, intent: &Intent, _feedback: &str, success: bool) -> Result<()> {
        // Update confidence thresholds based on feedback
        if let Some(pattern) = self.intent_patterns.get_mut(&intent.name) {
            if success {
                pattern.confidence_threshold = (pattern.confidence_threshold * 0.95).max(0.1);
            } else {
                pattern.confidence_threshold = (pattern.confidence_threshold * 1.05).min(0.9);
            }
        }
        Ok(())
    }
}
