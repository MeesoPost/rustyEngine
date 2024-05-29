use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Quiz {
    pub id: i32,
    pub name: String,
    // Add other fields as needed (e.g., created_at)
}

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub quiz_id: i32,
    pub text: String,
    // Add other fields (e.g., type, options)
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub text: String,
    pub is_correct: bool,
}
