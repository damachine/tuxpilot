use anyhow::Result;
use super::{Entity, EntityType};
use regex::Regex;

/// Named entity extraction system
#[derive(Debug, Clone)]
pub struct EntityExtractor {
    entity_patterns: Vec<EntityPattern>,
}

/// Entity extraction pattern
#[derive(Debug, Clone)]
pub struct EntityPattern {
    pub entity_type: EntityType,
    pub regex: Regex,
    pub confidence: f32,
}

impl EntityExtractor {
    pub async fn new() -> Result<Self> {
        let entity_patterns = vec![
            // IP Address pattern
            EntityPattern {
                entity_type: EntityType::IPAddress,
                regex: Regex::new(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b")?,
                confidence: 0.95,
            },
            // Port number pattern
            EntityPattern {
                entity_type: EntityType::Port,
                regex: Regex::new(r"\bport\s+(\d{1,5})\b")?,
                confidence: 0.9,
            },
            // Filename pattern
            EntityPattern {
                entity_type: EntityType::Filename,
                regex: Regex::new(r"\b[\w\-\.]+\.(txt|log|conf|cfg|json|xml|yml|yaml)\b")?,
                confidence: 0.8,
            },
            // Directory pattern
            EntityPattern {
                entity_type: EntityType::Directory,
                regex: Regex::new(r"\b(/[\w\-\.]+)+/?")?,
                confidence: 0.7,
            },
            // Service name pattern
            EntityPattern {
                entity_type: EntityType::ServiceName,
                regex: Regex::new(r"\b(nginx|apache|mysql|postgresql|docker|ssh|systemd)\b")?,
                confidence: 0.85,
            },
            // Username pattern
            EntityPattern {
                entity_type: EntityType::Username,
                regex: Regex::new(r"\buser\s+(\w+)\b")?,
                confidence: 0.8,
            },
            // Package name pattern
            EntityPattern {
                entity_type: EntityType::PackageName,
                regex: Regex::new(r"\bpackage\s+([\w\-\.]+)\b")?,
                confidence: 0.8,
            },
            // Number pattern
            EntityPattern {
                entity_type: EntityType::Number,
                regex: Regex::new(r"\b\d+\b")?,
                confidence: 0.6,
            },
        ];

        Ok(Self { entity_patterns })
    }

    pub async fn extract(&self, input: &str) -> Result<Vec<Entity>> {
        let mut entities = Vec::new();

        for pattern in &self.entity_patterns {
            for mat in pattern.regex.find_iter(input) {
                let entity = Entity {
                    entity_type: pattern.entity_type.clone(),
                    value: mat.as_str().to_string(),
                    start_pos: mat.start(),
                    end_pos: mat.end(),
                    confidence: pattern.confidence,
                };
                entities.push(entity);
            }
        }

        // Remove overlapping entities (keep highest confidence)
        entities.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        let mut filtered_entities = Vec::new();
        
        for entity in entities {
            let overlaps = filtered_entities.iter().any(|existing: &Entity| {
                entity.start_pos < existing.end_pos && entity.end_pos > existing.start_pos
            });
            
            if !overlaps {
                filtered_entities.push(entity);
            }
        }

        Ok(filtered_entities)
    }

    pub async fn update_from_feedback(&mut self, _entities: &[Entity], _feedback: &str, _success: bool) -> Result<()> {
        // Update entity extraction patterns based on feedback
        // In real implementation, would adjust confidence scores and patterns
        Ok(())
    }
}
