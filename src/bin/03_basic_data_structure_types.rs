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

struct _Error {}

enum _ErrorKind {}

fn main() {}
