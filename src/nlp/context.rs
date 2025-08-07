use anyhow::Result;
use super::{ConversationContext, ConversationTurn, Intent};
use std::collections::HashMap;

/// Context management system
#[derive(Debug, Clone)]
pub struct ContextManager {
    sessions: HashMap<String, ConversationContext>,
}

impl ContextManager {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            sessions: HashMap::new(),
        })
    }

    pub async fn get_context(&mut self, session_id: &str) -> Result<ConversationContext> {
        if let Some(context) = self.sessions.get(session_id) {
            Ok(context.clone())
        } else {
            // Create new session context
            let context = ConversationContext {
                session_id: session_id.to_string(),
                previous_intents: Vec::new(),
                current_topic: None,
                user_preferences: HashMap::new(),
                system_state: HashMap::new(),
                conversation_history: Vec::new(),
            };
            self.sessions.insert(session_id.to_string(), context.clone());
            Ok(context)
        }
    }

    pub async fn update_context(&mut self, session_id: &str, intent: &Intent, user_input: &str) -> Result<()> {
        if let Some(context) = self.sessions.get_mut(session_id) {
            // Update previous intents
            context.previous_intents.push(intent.name.clone());
            if context.previous_intents.len() > 10 {
                context.previous_intents.remove(0);
            }

            // Update current topic based on intent category
            context.current_topic = Some(format!("{:?}", intent.category));

            // Add to conversation history
            context.conversation_history.push(ConversationTurn {
                timestamp: chrono::Utc::now(),
                user_input: user_input.to_string(),
                system_response: "Processing...".to_string(),
                intent: intent.name.clone(),
                success: true,
            });

            // Keep only last 50 turns
            if context.conversation_history.len() > 50 {
                context.conversation_history.remove(0);
            }
        }
        Ok(())
    }

    pub async fn set_user_preference(&mut self, session_id: &str, key: &str, value: &str) -> Result<()> {
        if let Some(context) = self.sessions.get_mut(session_id) {
            context.user_preferences.insert(key.to_string(), value.to_string());
        }
        Ok(())
    }

    pub async fn get_user_preference(&self, session_id: &str, key: &str) -> Option<String> {
        self.sessions.get(session_id)
            .and_then(|context| context.user_preferences.get(key))
            .cloned()
    }
}
