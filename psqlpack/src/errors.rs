use std::path::PathBuf;

pub use error_chain::ChainedError;
pub use lalrpop_util::ParseError;

use lexer;
use connection::{ConnectionError, ConnectionErrorKind};

error_chain! {
    types {
        PsqlpackError, PsqlpackErrorKind, PsqlpackResultExt, PsqlpackResult;
    }
    links {
        Connection(ConnectionError, ConnectionErrorKind);
    }
    errors {
        ProjectReadError(path: PathBuf) {
            description("Couldn't read project file")
            display("Couldn't read project file: {}", path.as_path().display())
        }
        ProjectParseError(path: PathBuf) {
            description("Couldn't parse project file")
            display("Couldn't parse project file: {}", path.as_path().display())
        }
        InvalidScriptPath(path: String) {
            description("Invalid script path in project file")
            display("Invalid script path in project file: {}", path)
        }
        PublishProfileReadError(path: PathBuf) {
            description("Couldn't read publish profile file")
            display("Couldn't read publish profile file: {}", path.as_path().display())
        }
        PublishProfileParseError(path: PathBuf) {
            description("Couldn't parse publish profile file")
            display("Couldn't parse publish profile file: {}", path.as_path().display())
        }
        PackageReadError(path: PathBuf) {
            description("Couldn't read package file")
            display("Couldn't read package file: {}", path.as_path().display())
        }
        PackageUnarchiveError(path: PathBuf) {
            description("Couldn't unarchive package file")
            display("Couldn't unarchive package file: {}", path.as_path().display())
        }
        PackageInternalReadError(file_name: String) {
            description("Couldn't read part of the package file")
            display("Couldn't read part of the package file: {}", file_name)
        }
        IOError(file: String, message: String) {
            description("IO error when reading a file")
            display("IO error when reading {}: {}", file, message)
        }
        SyntaxError(file: String, line: String, line_number: i32, start_pos: i32, end_pos: i32) {
            description("SQL syntax error encountered")
            display(
                "SQL syntax error encountered in {} on line {}:\n  {}\n  {}{}",
                file, line_number, line, " ".repeat(*start_pos as usize), "^".repeat((end_pos - start_pos) as usize))
        }
        ParseError(file: String, errors: Vec<ParseError<(), lexer::Token, ()>>) {
            description("Parser error")
            display("Parser errors in {}:\n{}", file, ParseErrorFormatter(errors))
        }
        GenerationError(message: String) {
            description("Error generating package")
            display("Error generating package: {}", message)
        }
        FormatError(file: String, message: String) {
            description("Format error when reading a file")
            display("Format error when reading {}: {}", file, message)
        }
        DatabaseError(message: String) {
            description("Database error")
            display("Database error: {}", message)
        }
        ProjectError(message: String) {
            description("Project format error")
            display("Project format error: {}", message)
        }
        MultipleErrors(errors: Vec<PsqlpackError>) {
            description("Multiple errors")
            display("Multiple errors:\n{}", MultipleErrorFormatter(errors))
        }
    }
}

use std::fmt::{Display, Formatter, Result};

struct ParseErrorFormatter<'fmt>(&'fmt Vec<ParseError<(), lexer::Token, ()>>);

impl<'fmt> Display for ParseErrorFormatter<'fmt> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (i, error) in self.0.iter().enumerate() {
            write!(f, "{}: ", i)?;
            match *error {
                ParseError::InvalidToken { .. } => {
                    write!(f, "Invalid token")?
                }
                ParseError::UnrecognizedToken {
                    ref token,
                    ref expected,
                } => {
                    match *token {
                        Some(ref x) => writeln!(f, "Unexpected {:?}", x.1),
                        _ => writeln!(f, "Unexpected end of file"),
                    }?;
                    write!(f, "   Expected one of:\n   {}", expected.join(", "))?
                }
                ParseError::ExtraToken { ref token } => {
                    write!(f, "Extra token detected: {:?}", token)?
                }
                ParseError::User { ref error } => {
                    write!(f, "{:?}", error)?
                }
            }
        }
        Ok(())
    }
}

struct MultipleErrorFormatter<'fmt>(&'fmt Vec<PsqlpackError>);

impl<'fmt> Display for MultipleErrorFormatter<'fmt> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (i, error) in self.0.iter().enumerate() {
            write!(f, "--- Error {} ---\n{}", i, error.display())?;
        }
        Ok(())
    }
}
