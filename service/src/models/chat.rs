use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::chats;
use crate::types::Id;
use crate::types::JsonWrapper;

#[derive(Insertable, Debug)]
#[diesel(table_name = chats)]
pub struct NewChat {
    pub id: Id,
    pub user_id: Id,
    pub title: String,
    pub prompt_id: Option<Id>,
    pub config: JsonWrapper<ChatConfig>,
    pub cost: f32,
    pub vendor: String,
}

#[derive(Queryable, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub id: Id,
    pub user_id: Id,
    pub title: String,
    pub prompt_id: Option<Id>,
    pub config: JsonWrapper<ChatConfig>,
    pub cost: f32,
    pub vendor: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset, Deserialize, Default)]
#[diesel(table_name = chats)]
pub struct PatchChat {
    pub id: Id,
    pub title: Option<String>,
    pub prompt_id: Option<Id>,
    pub config: Option<JsonWrapper<ChatConfig>>,
    pub cost: Option<f32>,
    pub vendor: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatConfig {
    pub backtrack: usize,
    pub params: ChatParams,
}

impl Default for ChatConfig {
    fn default() -> Self {
        Self {
            backtrack: 3,
            params: ChatParams {
                model: "gpt-3.5-turbo".to_string(),
                temperature: None,
                stop: None,
                presence_penalty: None,
                frequency_penalty: None,
            },
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatParams {
    pub model: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
}