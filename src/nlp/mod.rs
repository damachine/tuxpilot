use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod intent;
pub mod entity;
pub mod context;
pub mod conversation;
pub mod sentiment;

use crate::config::Config;

/// Natural Language Processing system
#[derive(Debug, Clone)]
pub struct NLPSystem {
    config: Config,
    intent_classifier: intent::IntentClassifier,
    entity_extractor: entity::EntityExtractor,
    context_manager: context::ContextManager,
    conversation_manager: conversation::ConversationManager,
    sentiment_analyzer: sentiment::SentimentAnalyzer,
}

/// NLP analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NLPAnalysis {
    pub input_text: String,
    pub intent: Intent,
    pub entities: Vec<Entity>,
    pub sentiment: Sentiment,
    pub confidence: f32,
    pub context: ConversationContext,
    pub suggested_actions: Vec<SuggestedAction>,
}

/// Intent classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub name: String,
    pub confidence: f32,
    pub category: IntentCategory,
    pub parameters: HashMap<String, String>,
}

/// Intent categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntentCategory {
    SystemManagement,
    FileOperations,
    ProcessManagement,
    NetworkOperations,
    SecurityOperations,
    Monitoring,
    Troubleshooting,
    Information,
    Conversation,
    Unknown,
}

/// Named entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub entity_type: EntityType,
    pub value: String,
    pub start_pos: usize,
    pub end_pos: usize,
    pub confidence: f32,
}

/// Entity types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Filename,
    Directory,
    ProcessName,
    ServiceName,
    IPAddress,
    Port,
    Username,
    PackageName,
    Command,
    SystemResource,
    TimeExpression,
    Number,
}

/// Sentiment analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sentiment {
    pub polarity: SentimentPolarity,
    pub confidence: f32,
    pub emotional_state: EmotionalState,
}

/// Sentiment polarity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SentimentPolarity {
    Positive,
    Negative,
    Neutral,
}

/// Emotional state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmotionalState {
    Frustrated,
    Confused,
    Satisfied,
    Urgent,
    Curious,
    Neutral,
}

/// Conversation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub session_id: String,
    pub previous_intents: Vec<String>,
    pub current_topic: Option<String>,
    pub user_preferences: HashMap<String, String>,
    pub system_state: HashMap<String, String>,
    pub conversation_history: Vec<ConversationTurn>,
}

/// Conversation turn
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_input: String,
    pub system_response: String,
    pub intent: String,
    pub success: bool,
}

/// Suggested action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedAction {
    pub action_id: String,
    pub action_type: ActionType,
    pub description: String,
    pub command: Option<String>,
    pub confidence: f32,
    pub risk_level: RiskLevel,
    pub estimated_time: u32,
}

/// Action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    ExecuteCommand,
    ShowInformation,
    StartService,
    StopService,
    InstallPackage,
    UpdateSystem,
    CheckStatus,
    FixIssue,
    Clarification,
}

/// Risk levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Safe,
    Low,
    Medium,
    High,
    Critical,
}

impl NLPSystem {
    pub async fn new(config: Config) -> Result<Self> {
        let intent_classifier = intent::IntentClassifier::new().await?;
        let entity_extractor = entity::EntityExtractor::new().await?;
        let context_manager = context::ContextManager::new().await?;
        let conversation_manager = conversation::ConversationManager::new().await?;
        let sentiment_analyzer = sentiment::SentimentAnalyzer::new().await?;

        Ok(Self {
            config,
            intent_classifier,
            entity_extractor,
            context_manager,
            conversation_manager,
            sentiment_analyzer,
        })
    }

    pub async fn analyze_input(&mut self, input: &str, session_id: &str) -> Result<NLPAnalysis> {
        println!("🧠 Analyzing natural language input...");

        // Classify intent
        let intent = self.intent_classifier.classify(input).await?;

        // Extract entities
        let entities = self.entity_extractor.extract(input).await?;

        // Analyze sentiment
        let sentiment = self.sentiment_analyzer.analyze(input).await?;

        // Get conversation context
        let context = self.context_manager.get_context(session_id).await?;

        // Generate suggested actions
        let suggested_actions = self.generate_suggested_actions(&intent, &entities, &context).await?;

        // Calculate overall confidence
        let confidence = self.calculate_overall_confidence(&intent, &entities, &sentiment);

        // Update conversation context
        self.context_manager.update_context(session_id, &intent, input).await?;

        let analysis = NLPAnalysis {
            input_text: input.to_string(),
            intent,
            entities,
            sentiment,
            confidence,
            context,
            suggested_actions,
        };

        println!("🧠 NLP analysis completed (confidence: {:.1}%)", confidence * 100.0);
        Ok(analysis)
    }

    async fn generate_suggested_actions(
        &self,
        intent: &Intent,
        entities: &[Entity],
        _context: &ConversationContext,
    ) -> Result<Vec<SuggestedAction>> {
        let mut actions = Vec::new();

        match intent.category {
            IntentCategory::SystemManagement => {
                actions.push(SuggestedAction {
                    action_id: uuid::Uuid::new_v4().to_string(),
                    action_type: ActionType::CheckStatus,
                    description: "Check system status".to_string(),
                    command: Some("systemctl status".to_string()),
                    confidence: 0.8,
                    risk_level: RiskLevel::Safe,
                    estimated_time: 5,
                });
            }
            IntentCategory::FileOperations => {
                if let Some(filename) = entities.iter().find(|e| matches!(e.entity_type, EntityType::Filename)) {
                    actions.push(SuggestedAction {
                        action_id: uuid::Uuid::new_v4().to_string(),
                        action_type: ActionType::ShowInformation,
                        description: format!("Show information about file: {}", filename.value),
                        command: Some(format!("ls -la {}", filename.value)),
                        confidence: 0.9,
                        risk_level: RiskLevel::Safe,
                        estimated_time: 2,
                    });
                }
            }
            IntentCategory::ProcessManagement => {
                if let Some(process) = entities.iter().find(|e| matches!(e.entity_type, EntityType::ProcessName)) {
                    actions.push(SuggestedAction {
                        action_id: uuid::Uuid::new_v4().to_string(),
                        action_type: ActionType::CheckStatus,
                        description: format!("Check process status: {}", process.value),
                        command: Some(format!("ps aux | grep {}", process.value)),
                        confidence: 0.85,
                        risk_level: RiskLevel::Safe,
                        estimated_time: 3,
                    });
                }
            }
            IntentCategory::Troubleshooting => {
                actions.push(SuggestedAction {
                    action_id: uuid::Uuid::new_v4().to_string(),
                    action_type: ActionType::FixIssue,
                    description: "Run system diagnostics".to_string(),
                    command: Some("journalctl -xe --no-pager | tail -50".to_string()),
                    confidence: 0.7,
                    risk_level: RiskLevel::Low,
                    estimated_time: 10,
                });
            }
            IntentCategory::Information => {
                actions.push(SuggestedAction {
                    action_id: uuid::Uuid::new_v4().to_string(),
                    action_type: ActionType::ShowInformation,
                    description: "Provide system information".to_string(),
                    command: Some("uname -a && uptime && df -h".to_string()),
                    confidence: 0.9,
                    risk_level: RiskLevel::Safe,
                    estimated_time: 5,
                });
            }
            _ => {
                actions.push(SuggestedAction {
                    action_id: uuid::Uuid::new_v4().to_string(),
                    action_type: ActionType::Clarification,
                    description: "Request clarification from user".to_string(),
                    command: None,
                    confidence: 0.6,
                    risk_level: RiskLevel::Safe,
                    estimated_time: 0,
                });
            }
        }

        Ok(actions)
    }

    fn calculate_overall_confidence(&self, intent: &Intent, entities: &[Entity], sentiment: &Sentiment) -> f32 {
        let intent_weight = 0.5;
        let entity_weight = 0.3;
        let sentiment_weight = 0.2;

        let entity_confidence = if entities.is_empty() {
            0.5 // Neutral if no entities
        } else {
            entities.iter().map(|e| e.confidence).sum::<f32>() / entities.len() as f32
        };

        intent.confidence * intent_weight + 
        entity_confidence * entity_weight + 
        sentiment.confidence * sentiment_weight
    }

    pub async fn generate_response(&self, analysis: &NLPAnalysis) -> Result<String> {
        let mut response = String::new();

        // Generate contextual response based on intent and sentiment
        match analysis.intent.category {
            IntentCategory::SystemManagement => {
                response.push_str("I can help you manage your system. ");
            }
            IntentCategory::FileOperations => {
                response.push_str("I'll assist you with file operations. ");
            }
            IntentCategory::ProcessManagement => {
                response.push_str("Let me help you with process management. ");
            }
            IntentCategory::Troubleshooting => {
                response.push_str("I'll help you troubleshoot the issue. ");
                
                // Add empathy based on sentiment
                match analysis.sentiment.emotional_state {
                    EmotionalState::Frustrated => {
                        response.push_str("I understand this can be frustrating. ");
                    }
                    EmotionalState::Urgent => {
                        response.push_str("I see this is urgent. Let me prioritize this. ");
                    }
                    _ => {}
                }
            }
            IntentCategory::Information => {
                response.push_str("Here's the information you requested. ");
            }
            IntentCategory::Conversation => {
                response.push_str("I'm here to help! ");
            }
            _ => {
                response.push_str("I'm not sure I understand. Could you please clarify? ");
            }
        }

        // Add suggested actions
        if !analysis.suggested_actions.is_empty() {
            response.push_str("Here are some actions I can take:\n");
            for (i, action) in analysis.suggested_actions.iter().enumerate() {
                response.push_str(&format!("{}. {}\n", i + 1, action.description));
            }
        }

        Ok(response)
    }

    pub async fn improve_from_feedback(&mut self, analysis: &NLPAnalysis, feedback: &str, success: bool) -> Result<()> {
        // Update models based on user feedback
        self.intent_classifier.update_from_feedback(&analysis.intent, feedback, success).await?;
        self.entity_extractor.update_from_feedback(&analysis.entities, feedback, success).await?;
        
        // Log conversation turn
        self.conversation_manager.log_turn(
            &analysis.context.session_id,
            &analysis.input_text,
            feedback,
            &analysis.intent.name,
            success,
        ).await?;

        println!("🧠 NLP system updated from user feedback");
        Ok(())
    }

    pub async fn get_conversation_summary(&self, session_id: &str) -> Result<String> {
        self.conversation_manager.get_session_summary(session_id).await
    }

    pub async fn detect_language(&self, text: &str) -> Result<String> {
        // Simple language detection (in real implementation, would use proper language detection)
        if text.chars().any(|c| "äöüßÄÖÜ".contains(c)) {
            Ok("de".to_string()) // German
        } else if text.chars().any(|c| "àáâãäåæçèéêëìíîïñòóôõöøùúûüý".contains(c)) {
            Ok("fr".to_string()) // French
        } else {
            Ok("en".to_string()) // Default to English
        }
    }

    pub async fn translate_response(&self, response: &str, target_language: &str) -> Result<String> {
        // Mock translation (in real implementation, would use translation service)
        match target_language {
            "de" => Ok(format!("[DE] {}", response)),
            "fr" => Ok(format!("[FR] {}", response)),
            _ => Ok(response.to_string()),
        }
    }
}
