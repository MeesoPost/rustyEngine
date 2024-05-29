use std::sync::{Arc, Mutex};
use interprocess::os::windows::named_pipe::NamedPipeListener;
use std::io::{BufReader, BufWriter};
use rusqlite::Connection;
use crate::{db, models::Request};
use crate::db::Response;

pub fn start_api(shared_connection: Arc<Mutex<Connection>>) {
    let pipe_name = "my_quiz_pipe";
    let listener = NamedPipeListener::new(pipe_name).expect("Failed to create named pipe");

    for conn in listener.incoming() {
        let conn = conn.unwrap();
        let mut reader = BufReader::new(conn);
        let mut writer = BufWriter::new(conn);

        // Attempt to read and parse the request, handling potential errors
        let request = match serde_json::from_reader::<_, Request>(&mut reader) {
            Ok(req) => req,
            Err(err) => {
                let error_response = Response::Error("Invalid request format".to_string());
                serde_json::to_writer(&mut writer, &error_response).ok();
                continue; // Skip to the next connection
            }
        };

        // Acquire the lock on the database connection
        let mut connection = shared_connection.lock().unwrap();
        // Process the request and get the response
        let response = handle_request(&mut connection, request);
        
        // Release the lock after processing the request
        drop(connection);
        
        // Send the response back to the client, handling potential errors
        match serde_json::to_writer(&mut writer, &response) {
            Ok(_) => (),
            Err(err) => eprintln!("Error sending response: {}", err),
        }
    }
}

// Move handle_request outside the start_api function
pub fn handle_request(conn: &mut Connection, request: Request) -> Response {
    match request.action.as_str() {
        "GetQuizzes" => Response::Quizzes(db::get_quizzes(conn).unwrap_or_default()),
        "GetQuestionsForQuiz" => {
            let quiz_id = request.quiz_id.unwrap();
            Response::Questions(db::get_questions_for_quiz(conn, quiz_id).unwrap_or_default())
        },
        "AddQuiz" => {
            let name = request.name.unwrap();
            match db::add_quiz(conn, name) {
                Ok(new_quiz_id) => Response::Success(format!("Quiz added with ID: {}", new_quiz_id)),
                Err(err) => Response::Error(err.to_string()), 
            }
        },
        _ => Response::Error("Invalid action".to_string()),
    }
}
