use anyhow::Result;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMessage {
    pub id: String,
    pub content: String,
    pub sender: String, // "user" or "assistant"
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct SessionMemory {
    pub messages: VecDeque<MemoryMessage>,
    pub max_messages: usize,
    pub session_id: String,
}

impl SessionMemory {
    pub fn new(session_id: String, max_messages: usize) -> Self {
        Self {
            messages: VecDeque::new(),
            max_messages,
            session_id,
        }
    }

    pub fn add_message(&mut self, message: MemoryMessage) {
        self.messages.push_back(message);
        
        // Keep only the last max_messages
        while self.messages.len() > self.max_messages {
            self.messages.pop_front();
        }
    }

    pub fn get_recent_messages(&self, count: usize) -> Vec<&MemoryMessage> {
        self.messages
            .iter()
            .rev()
            .take(count)
            .rev()
            .collect()
    }

    pub fn get_all_messages(&self) -> Vec<&MemoryMessage> {
        self.messages.iter().collect()
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

#[derive(Debug, Clone)]
pub struct MemoryManager {
    sessions: DashMap<String, SessionMemory>,
    max_sessions: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            sessions: DashMap::new(),
            max_sessions: 100, // Keep last 100 sessions
        }
    }

    pub async fn store_message(
        &self,
        session_id: &str,
        content: &str,
        sender: &str,
    ) -> Result<()> {
        let message = MemoryMessage {
            id: uuid::Uuid::new_v4().to_string(),
            content: content.to_string(),
            sender: sender.to_string(),
            timestamp: chrono::Utc::now(),
            metadata: serde_json::json!({}),
        };

        // Get or create session memory
        let mut session_memory = self.sessions
            .entry(session_id.to_string())
            .or_insert_with(|| SessionMemory::new(session_id.to_string(), 50));

        session_memory.add_message(message);

        // Clean up old sessions if we exceed max_sessions
        if self.sessions.len() > self.max_sessions {
            self.cleanup_old_sessions().await;
        }

        Ok(())
    }

    pub async fn get_session_messages(&self, session_id: &str) -> Vec<MemoryMessage> {
        if let Some(session) = self.sessions.get(session_id) {
            session.get_all_messages().into_iter().cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub async fn get_recent_context(&self, session_id: &str, count: usize) -> Vec<MemoryMessage> {
        if let Some(session) = self.sessions.get(session_id) {
            session.get_recent_messages(count).into_iter().cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub async fn clear_session(&self, session_id: &str) -> Result<()> {
        if let Some(mut session) = self.sessions.get_mut(session_id) {
            session.clear();
        }
        Ok(())
    }

    pub async fn clear_all_sessions(&self) -> Result<()> {
        self.sessions.clear();
        Ok(())
    }

    async fn cleanup_old_sessions(&self) {
        // Simple cleanup: remove sessions with no recent activity
        // In a real implementation, you might want more sophisticated cleanup
        let mut to_remove = Vec::new();
        
        for entry in self.sessions.iter() {
            let session_id = entry.key();
            let session = entry.value();
            
            // Remove sessions with no messages
            if session.messages.is_empty() {
                to_remove.push(session_id.clone());
            }
        }
        
        for session_id in to_remove {
            self.sessions.remove(&session_id);
        }
    }

    pub fn get_session_count(&self) -> usize {
        self.sessions.len()
    }

    pub fn get_total_message_count(&self) -> usize {
        self.sessions.iter().map(|entry| entry.value().messages.len()).sum()
    }
}
