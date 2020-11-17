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
    error_kind:ErrorKind,
    context: String,
    status_code: u32,
}

enum ErrorKind {
    BadRequest,
    Unauthorized,
    DatabaseError(String),
    InternalServerError(String),
}

fn main() {
    let error = Error {
        error_kind: ErrorKind::BadRequest,
        context: String::from("Must provide an id"),
        status_code: 400,
    };

    let unauthorized = Error {
        error_kind: ErrorKind::Unauthorized,
        context: String::from("You cannot access this data"),
        status_code: 401,
    };

    let database_error = Error {
        error_kind: ErrorKind::DatabaseError(String::from("null of undefined")),
        context: String::from("Database error"),
        status_code: 500,
    };

    let internal_error = Error {
        error_kind: ErrorKind::InternalServerError(String::from("null of undefined")),
        context: String::from("Internal Server Error"),
        status_code: 500,
    };
}
