// db.rs
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

use crate::models::{Quiz, Question, Answer};

// Request Type
#[derive(Deserialize)]
pub enum Request {
    GetQuizzes,
    GetQuestionsForQuiz { quiz_id: i32 },
    AddQuiz { name: String },
    AddQuestion { quiz_id: i32, text: String },
    AddAnswers { question_id: i32, answers: Vec<Answer> },
}

// Response Type
#[derive(Serialize)]
pub enum Response {
    Quizzes(Vec<Quiz>),
    Questions(Vec<Question>),
    Success(String),
    Error(String),
}

// Function to create tables in the database
pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS quizzes (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS questions (
            id INTEGER PRIMARY KEY,
            quiz_id INTEGER,
            text TEXT NOT NULL,
            FOREIGN KEY(quiz_id) REFERENCES quizzes(id)
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS answers (
            id INTEGER PRIMARY KEY,
            question_id INTEGER,
            text TEXT NOT NULL,
            is_correct BOOLEAN,
            FOREIGN KEY(question_id) REFERENCES questions(id)
        )",
        (),
    )?;

    Ok(())
}

// Function to fetch quizzes from the database
pub fn get_quizzes(conn: &Connection) -> Result<Vec<Quiz>> {
    let mut stmt = conn.prepare("SELECT id, name FROM quizzes")?;
    let quiz_iter = stmt.query_map([], |row| {
        Ok(Quiz {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    quiz_iter.collect()
}

// Function to fetch questions for a specific quiz
pub fn get_questions_for_quiz(conn: &Connection, quiz_id: i32) -> Result<Vec<Question>> {
    let mut stmt = conn.prepare("SELECT id, text FROM questions WHERE quiz_id = ?")?;
    let question_iter = stmt.query_map([quiz_id], |row| {
        Ok(Question {
            id: row.get(0)?,
            quiz_id, // Directly assign the quiz_id
            text: row.get(1)?,
        })
    })?;

    question_iter.collect()
}

// Function to add a new quiz to the database
pub fn add_quiz(conn: &Connection, name: String) -> Result<i32> {
    conn.execute("INSERT INTO quizzes (name) VALUES (?)", [name])?;
    conn.last_insert_rowid() as i32
}

// Function to add a new question to a quiz
pub fn add_question(conn: &Connection, quiz_id: i32, text: String) -> Result<i32> {
    conn.execute("INSERT INTO questions (quiz_id, text) VALUES (?, ?)", [quiz_id, text])?;
    conn.last_insert_rowid() as i32
}

// Function to add multiple answers to a question
pub fn add_answers(conn: &Connection, question_id: i32, answers: Vec<Answer>) -> Result<()> {
    let mut stmt = conn.prepare("INSERT INTO answers (question_id, text, is_correct) VALUES (?, ?, ?)")?;
    for answer in answers {
        stmt.execute([question_id, answer.text, answer.is_correct])?;
    }
    Ok(())
}
