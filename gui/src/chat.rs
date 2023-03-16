use std::{path::Path, sync::Arc};

use askai_api::{OpenAIApi, Role, StreamContent, Topic};
use futures::{lock::Mutex, StreamExt};
use tokio::fs;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::result::Result;

pub struct Chat {
    pub id: Uuid,
    pub title: String,
    pub topic: Arc<Mutex<Topic>>,
}

impl Chat {
    pub fn new(topic: Option<String>, title: &str) -> Self {
        let topic = Topic::new(topic);
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            topic: Arc::new(Mutex::new(topic)),
        }
    }

    pub async fn send_message(
        &self,
        sender: Sender<StreamContent>,
        message: &str,
        api: OpenAIApi,
    ) -> Uuid {
        let mut topic = self.topic.lock().await;
        let message_id = topic.add_user_message(message);

        let topic = self.topic.clone();
        tokio::spawn(async move {
            let mut topic = topic.lock().await;
            match topic.send(&api).await {
                Ok(mut stream) => {
                    while let Some(content) = stream.next().await {
                        sender.send(content).await.expect("send message");
                    }
                }
                Err(err) => {
                    sender
                        .send(StreamContent::Error(err))
                        .await
                        .expect("send error");
                }
            };
        });

        message_id
    }

    pub async fn resend_message(
        &self,
        sender: Sender<StreamContent>,
        message_id: Uuid,
        api: OpenAIApi,
    ) -> Result<()> {
        let topic = self.topic.clone();

        tokio::spawn(async move {
            let mut topic = topic.lock().await;
            match topic.resend(&api, message_id).await {
                Ok(mut stream) => {
                    while let Some(content) = stream.next().await {
                        sender.send(content).await.expect("send message");
                    }
                }
                Err(err) => {
                    sender
                        .send(StreamContent::Error(err))
                        .await
                        .expect("send error");
                }
            };
        });

        Ok(())
    }

    pub async fn reset(&self) {
        self.topic.lock().await.reset();
    }

    pub fn from_topic(id: Uuid, title: &str, topic: Topic) -> Self {
        Self {
            id,
            title: title.to_string(),
            topic: Arc::new(Mutex::new(topic)),
        }
    }

    pub async fn topic_json_string(&self) -> String {
        self.topic.lock().await.to_json_string()
    }

    pub async fn save_as_markdown(&self, path: &Path) -> Result<()> {
        let markdown = self.to_markdown().await?;

        fs::write(path, markdown).await?;

        Ok(())
    }

    async fn to_markdown(&self) -> Result<String> {
        let mut markdown = String::new();

        if !self.title.is_empty() {
            markdown.push_str(&format!("# {}\n\n", self.title));
        }

        let messages = self.topic.lock().await.messages();

        for message in messages {
            match message.role {
                Role::User => {
                    markdown.push_str(&format!("## {}\n", message.content));
                }
                Role::Assistant => {
                    markdown.push_str(&format!("{}\n", message.content));
                }
                _ => {}
            }
        }

        Ok(markdown)
    }
}
