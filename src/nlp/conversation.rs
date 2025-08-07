use anyhow::Result;
use super::ConversationTurn;
use std::collections::HashMap;

/// Conversation management system
#[derive(Debug, Clone)]
pub struct ConversationManager {
    session_histories: HashMap<String, Vec<ConversationTurn>>,
}

impl ConversationManager {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            session_histories: HashMap::new(),
        })
    }

    pub async fn log_turn(
        &mut self,
        session_id: &str,
        user_input: &str,
        system_response: &str,
        intent: &str,
        success: bool,
    ) -> Result<()> {
        let turn = ConversationTurn {
            timestamp: chrono::Utc::now(),
            user_input: user_input.to_string(),
            system_response: system_response.to_string(),
            intent: intent.to_string(),
            success,
        };

        self.session_histories
            .entry(session_id.to_string())
            .or_insert_with(Vec::new)
            .push(turn);

        Ok(())
    }

    pub async fn get_session_summary(&self, session_id: &str) -> Result<String> {
        if let Some(history) = self.session_histories.get(session_id) {
            let mut summary = String::new();
            summary.push_str(&format!("Conversation Summary for Session: {}\n", session_id));
            summary.push_str(&format!("Total Turns: {}\n", history.len()));
            
            let successful_turns = history.iter().filter(|t| t.success).count();
            summary.push_str(&format!("Successful Turns: {}\n", successful_turns));
            
            if !history.is_empty() {
                summary.push_str(&format!("Duration: {} minutes\n", 
                    (history.last().unwrap().timestamp - history.first().unwrap().timestamp).num_minutes()));
            }

            Ok(summary)
        } else {
            Ok("No conversation history found for this session.".to_string())
        }
    }
}
