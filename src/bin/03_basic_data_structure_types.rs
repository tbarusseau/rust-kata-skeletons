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

struct Error {
    error_kind: ErrorKind,
    context: String,
    status_code: i32,
}
enum ErrorKind {
    BadRequest,
    Unauthorized,
    DatabaseError(i32, String),
    InternalServerError(i32, String),
}

fn display_error(error: &Error) {
    match &error.error_kind {
        ErrorKind::BadRequest => {
            println!("A BadRequest Error has been raised, context is : {} with status code : {} ", error.context, error.status_code);
        }
        ErrorKind::Unauthorized => {
            println!("An Unauthorized Error has been raised, context is : {} with status code : {} ", error.context, error.status_code);
        }
        ErrorKind::DatabaseError(code, msg) => {
            println!("A DatabaseError has been raised, message is : {} with code : {} ", msg, code);
        }
        ErrorKind::InternalServerError(code, msg) => {
            println!("A InternalServerError has been raised, message is : {} with code : {} ", msg, code);
        }
    }
}

fn main() {
    let e0 = Error {
        error_kind: ErrorKind::BadRequest,
        context: String::from("some context causing the error"),
        status_code: 400,
    };

    let e1 = Error {
        error_kind: ErrorKind::Unauthorized,
        context: String::from("some context causing the error"),
        status_code: 401,
    };

    let e2 = Error {
        error_kind: ErrorKind::DatabaseError(502, "Resource Not Found".to_string()),
        context: String::from("some context causing the error"),
        status_code: 500,
    };

    let e3 = Error {
        error_kind: ErrorKind::InternalServerError(503, "Service Down".to_string()),
        context: String::from("some context causing the error"),
        status_code: 500,
    };
    [e0, e1, e2, e3].iter().for_each(|e| display_error(e));
}
