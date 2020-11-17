// Seen in this chapter:
// - structs
// - tuple-structs
// - unit-structs
// - enums

// We're going to create a struct that contains error information.

// The struct is going to contain:
// - An enum field: `ErrorKind`
// - A string field: `context`
// - A number: `status_code`

// The enum is going to contain the following values:
// - BadRequest
// - Unauthorized
// - DatabaseError*
// - InternalServerError*
// Values annotated with a * must contain data that describe the error in itself

enum ErrorKind {
    BadRequest,
    Unauthorized,
    DatabaseError(String),
    InternalServerError(String),
}

struct Error {
    kind: ErrorKind,
    context: String,
    status_code: i32,
}

fn main() {
    let bad_request = Error {
        kind: ErrorKind::BadRequest,
        context: String::from("Vous savez pas remplir un formulaire"),
        status_code: 400,
    };

    let unauthorized = Error {
        kind: ErrorKind::Unauthorized,
        context: String::from("Vous devez passer votre token d'authentification"),
        status_code: 401,
    };

    let db_error = Error {
        kind: ErrorKind::DatabaseError(String::from("Unique constraint error")),
        context: String::from("Erreur d'accès à la base de données"),
        status_code: 502,
    };

    let server_error = Error {
        kind: ErrorKind::InternalServerError(String::from("Le webmestre ne sait pas coder")),
        context: String::from("Erreur lors de la communication avec l'api"),
        status_code: 500,
    };

    for error in [bad_request, unauthorized, db_error, server_error].iter() {
        match &error.kind {
            ErrorKind::BadRequest | ErrorKind::Unauthorized => println!("{} - {}", error.status_code, error.context),
            ErrorKind::DatabaseError(msg) | ErrorKind::InternalServerError(msg) => println!("{} - {}: {}", error.status_code, error.context, msg),
        }
    }
}
