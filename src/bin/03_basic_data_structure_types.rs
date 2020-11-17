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
  status_code: i32
}

enum ErrorKind {
  BadRequest,
  Unauthorized,
  DatabaseError(String),
  InternalServerError(String)
}

fn display_error(error: Error) {
  match error.error_kind {
    ErrorKind::BadRequest => {
      println!("It's a BadRequest error ! Context: {}, StatusCode: {}", error.context, error.status_code)
    }
    ErrorKind::InternalServerError(description) => {
      println!("It's an InternalServerError ! Description: {}", description)
    }
    _ => println!("It's another error !"),
  }
}

fn main() {
  let bd_error = Error {
    error_kind: ErrorKind::BadRequest,
    context: String::from("The context"),
    status_code: 400
  };
  let internal_error = Error {
    error_kind: ErrorKind::InternalServerError(String::from("Explosion")),
    context: String::from("The context"),
    status_code: 500
  };

  display_error(bd_error);
  display_error(internal_error);
}
