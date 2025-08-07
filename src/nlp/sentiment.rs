use anyhow::Result;
use super::{Sentiment, SentimentPolarity, EmotionalState};

/// Sentiment analysis system
#[derive(Debug, Clone)]
pub struct SentimentAnalyzer {
    positive_words: Vec<String>,
    negative_words: Vec<String>,
    frustration_words: Vec<String>,
    urgency_words: Vec<String>,
}

impl SentimentAnalyzer {
    pub async fn new() -> Result<Self> {
        let positive_words = vec![
            "good", "great", "excellent", "perfect", "awesome", "wonderful",
            "fantastic", "amazing", "love", "like", "happy", "pleased",
            "satisfied", "thank", "thanks", "appreciate"
        ].iter().map(|s| s.to_string()).collect();

        let negative_words = vec![
            "bad", "terrible", "awful", "horrible", "hate", "dislike",
            "angry", "frustrated", "annoyed", "broken", "failed", "error",
            "problem", "issue", "wrong", "stupid", "useless"
        ].iter().map(|s| s.to_string()).collect();

        let frustration_words = vec![
            "frustrated", "annoyed", "angry", "mad", "irritated", "upset",
            "can't", "won't", "doesn't work", "not working", "broken"
        ].iter().map(|s| s.to_string()).collect();

        let urgency_words = vec![
            "urgent", "emergency", "asap", "immediately", "now", "quickly",
            "fast", "critical", "important", "deadline", "hurry"
        ].iter().map(|s| s.to_string()).collect();

        Ok(Self {
            positive_words,
            negative_words,
            frustration_words,
            urgency_words,
        })
    }

    pub async fn analyze(&self, input: &str) -> Result<Sentiment> {
        let input_lower = input.to_lowercase();
        
        let mut positive_score = 0;
        let mut negative_score = 0;
        let mut frustration_score = 0;
        let mut urgency_score = 0;

        // Count sentiment indicators
        for word in &self.positive_words {
            if input_lower.contains(word) {
                positive_score += 1;
            }
        }

        for word in &self.negative_words {
            if input_lower.contains(word) {
                negative_score += 1;
            }
        }

        for word in &self.frustration_words {
            if input_lower.contains(word) {
                frustration_score += 1;
            }
        }

        for word in &self.urgency_words {
            if input_lower.contains(word) {
                urgency_score += 1;
            }
        }

        // Determine polarity
        let polarity = if positive_score > negative_score {
            SentimentPolarity::Positive
        } else if negative_score > positive_score {
            SentimentPolarity::Negative
        } else {
            SentimentPolarity::Neutral
        };

        // Determine emotional state
        let emotional_state = if urgency_score > 0 {
            EmotionalState::Urgent
        } else if frustration_score > 0 {
            EmotionalState::Frustrated
        } else if input_lower.contains("?") || input_lower.contains("how") || input_lower.contains("what") {
            EmotionalState::Curious
        } else if positive_score > 0 {
            EmotionalState::Satisfied
        } else if negative_score > 0 {
            EmotionalState::Confused
        } else {
            EmotionalState::Neutral
        };

        // Calculate confidence
        let total_indicators = positive_score + negative_score + frustration_score + urgency_score;
        let confidence = if total_indicators > 0 {
            (total_indicators as f32 / 10.0).min(1.0)
        } else {
            0.5 // Neutral confidence when no indicators
        };

        Ok(Sentiment {
            polarity,
            confidence,
            emotional_state,
        })
    }
}
