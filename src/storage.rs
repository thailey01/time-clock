use crate::Session;

use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum ProcessingError {
    ParseError(String),
    FileError(String),
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(reason) => write!(f, "{}", reason),
            Self::FileError(reason) => write!(f, "{}", reason),
        }
    }
}

impl std::error::Error for ProcessingError {}

impl ProcessingError {
    pub fn parse(reason: &str) -> Self {
        Self::ParseError(reason.to_string())
    }
    pub fn file(reason: &str) -> Self {
        Self::FileError(reason.to_string())
    }
}

#[derive(PartialEq, Debug)]
struct CSVRow {
    id: u32,
    start: u64,
    end: Option<u64>,
    project_id: Option<u32>,
}

pub type RowIssue = (usize, String);

pub type SessionsAndIssues = (Vec<Session>, Vec<RowIssue>);

pub fn read_csv_file_and_return_session_data(
    path: &Path,
) -> Result<SessionsAndIssues, ProcessingError> {
    if !path.extension().is_some_and(|ext| ext.eq("csv")) {
        return Err(ProcessingError::file("Invalid file type"));
    }
    let Ok(f) = File::open(path) else {
        return Err(ProcessingError::file("Unable to read file"));
    };
    let bf = BufReader::new(f);

    let headers = "id,start_time,end_time,project_id";
    let mut sessions: Vec<Session> = vec![];
    let mut issues: Vec<RowIssue> = vec![];
    for (idx, line) in bf.lines().enumerate() {
        let Ok(line) = line else {
            issues.push((idx + 1, line.unwrap_err().to_string()));
            continue;
        };

        if line.eq(headers) {
            continue;
        }

        let row_values = process_line(&line);

        let Ok(vals) = row_values else {
            let err = row_values.unwrap_err();

            issues.push((idx + 1, err.to_string()));
            continue;
        };

        let session = Session::new(vals.id, vals.start, vals.end, vals.project_id);

        sessions.push(session);
    }

    Ok((sessions, issues))
}

fn process_line(line: &str) -> Result<CSVRow, ProcessingError> {
    let line_parts: Vec<&str> = line.split(',').collect();

    if line_parts.len() < 4 {
        return Err(ProcessingError::parse("Malformed row"));
    }

    if line_parts[0].is_empty() {
        return Err(ProcessingError::parse("Missing row id"));
    }

    if line_parts[1].is_empty() {
        return Err(ProcessingError::parse("Missing row start_time"));
    }

    let Ok(Some(parsed_id)) = clean_u32_from_line(line_parts[0]) else {
        return Err(ProcessingError::parse("Invalid row id"));
    };

    let Ok(Some(parsed_start)) = clean_u64_from_line(line_parts[1]) else {
        return Err(ProcessingError::parse("Invalid row start_time"));
    };

    let Ok(parsed_end) = clean_u64_from_line(line_parts[2]) else {
        return Err(ProcessingError::parse("Invalid row end_time"));
    };

    let Ok(parsed_project_id) = clean_u32_from_line(line_parts[3]) else {
        return Err(ProcessingError::parse("Invalid row project_id"));
    };

    Ok(CSVRow {
        id: parsed_id,
        start: parsed_start,
        end: parsed_end,
        project_id: parsed_project_id,
    })
}

fn clean_u32_from_line(part: &str) -> Result<Option<u32>, ProcessingError> {
    if part.is_empty() {
        return Ok(None);
    }

    part.parse::<u32>()
        .map(Some)
        .map_err(|_| ProcessingError::parse("Invalid u32 value"))
}

fn clean_u64_from_line(part: &str) -> Result<Option<u64>, ProcessingError> {
    if part.is_empty() {
        return Ok(None);
    }

    part.parse::<u64>()
        .map(Some)
        .map_err(|_| ProcessingError::parse("Invalid u64 value"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test/");

    #[test]
    fn clean_line_returns_parsed_value() {
        let val_u32: u32 = 10000000;
        let str_u32 = format!("{val_u32}");

        let parsed_u32 = clean_u32_from_line(&str_u32);

        let val_u64: u64 = 10000000;
        let str_u64 = format!("{val_u64}");

        let parsed_u64 = clean_u64_from_line(&str_u64);

        assert_eq!(Ok(Some(val_u32)), parsed_u32);
        assert_eq!(Ok(Some(val_u64)), parsed_u64);
    }

    #[test]
    fn clean_u32_fails_from_non_integer() {
        let invalid = "banana";

        let result = clean_u32_from_line(invalid);

        assert_eq!(result, Err(ProcessingError::parse("Invalid u32 value")));
    }

    #[test]
    fn clean_u64_fails_from_non_integer() {
        let invalid = "banana";

        let result = clean_u64_from_line(invalid);

        assert_eq!(result, Err(ProcessingError::parse("Invalid u64 value")));
    }

    #[test]
    fn process_line_successfully() {
        let line = "1,2020,2021,3";
        let correct_row = CSVRow {
            id: 1,
            start: 2020,
            end: Some(2021),
            project_id: Some(3),
        };

        let row = process_line(line);

        assert_eq!(row, Ok(correct_row));

        let line = "1,2020,2021,";
        let correct_row = CSVRow {
            id: 1,
            start: 2020,
            end: Some(2021),
            project_id: None,
        };

        let row = process_line(line);

        assert_eq!(row, Ok(correct_row));

        let line = "1,2020,,";
        let correct_row = CSVRow {
            id: 1,
            start: 2020,
            end: None,
            project_id: None,
        };

        let row = process_line(line);

        assert_eq!(row, Ok(correct_row));
    }

    #[test]
    fn process_line_fails_from_too_few_cols() {
        let line = "1,";

        let result = process_line(line);

        assert_eq!(result, Err(ProcessingError::parse("Malformed row")));
    }

    #[test]
    fn process_line_fails_from_missing_id() {
        let line = ",2020,2021,2";

        let result = process_line(line);

        assert_eq!(result, Err(ProcessingError::parse("Missing row id")));
    }

    #[test]
    fn process_line_fails_from_missing_start_time() {
        let line = "1,,2021,2";

        let result = process_line(line);

        assert_eq!(
            result,
            Err(ProcessingError::parse("Missing row start_time"))
        );
    }

    #[test]
    fn process_line_fails_string_in_col() {
        let line = "abc,2020,2021,2";

        let result = process_line(line);

        assert_eq!(result, Err(ProcessingError::parse("Invalid row id")));

        let line = "1,abc,2021,2";

        let result = process_line(line);

        assert_eq!(
            result,
            Err(ProcessingError::parse("Invalid row start_time"))
        );

        let line = "1,2020,abc,2";

        let result = process_line(line);

        assert_eq!(result, Err(ProcessingError::parse("Invalid row end_time")));

        let line = "1,2020,2021,abc";

        let result = process_line(line);

        assert_eq!(
            result,
            Err(ProcessingError::parse("Invalid row project_id"))
        );
    }

    #[test]
    fn process_file_successfully() {
        let file_dir: String = TEST_DIR.to_string() + "test_rows.csv";
        let file_path = Path::new(&file_dir);

        let validated_session = Session::new(1, 2020, Some(2021), Some(2));
        let validated_sessions = vec![validated_session];
        let validated_issues: Vec<RowIssue> = vec![];

        let determined_sessions = read_csv_file_and_return_session_data(file_path);

        assert_eq!(
            determined_sessions,
            Ok((validated_sessions, validated_issues))
        );
    }

    #[test]
    fn process_file_with_issues() {
        let file_dir: String = TEST_DIR.to_string() + "test_rows_with_issues.csv";
        let file_path = Path::new(&file_dir);

        let validated_row_1 = Session::new(1, 2020, Some(2021), Some(2));
        let validated_row_2 = Session::new(3, 3020, Some(3021), Some(2));
        let validated_row_3 = Session::new(5, 4020, Some(4021), Some(2));
        let validated_rows = vec![validated_row_1, validated_row_2, validated_row_3];
        let issue_row_1: (usize, String) = (3, "Missing row start_time".to_string());
        let issue_row_2: (usize, String) = (5, "Invalid row end_time".to_string());
        let issue_row_3: (usize, String) = (7, "Missing row id".to_string());
        let issue_rows: Vec<(usize, String)> = vec![issue_row_1, issue_row_2, issue_row_3];

        let determined_sessions_issues = read_csv_file_and_return_session_data(file_path);

        assert_eq!(determined_sessions_issues, Ok((validated_rows, issue_rows)));
    }

    #[test]
    fn fail_to_process_nonexistant_file() {
        let file_dir: String = TEST_DIR.to_string() + "invalid.csv";
        let file_path = Path::new(&file_dir);

        let result = read_csv_file_and_return_session_data(file_path);

        assert_eq!(result, Err(ProcessingError::file("Unable to read file")));
    }

    #[test]
    fn fail_to_process_non_csv_file() {
        let file_dir: String = TEST_DIR.to_string() + "test_rows.txt";
        let file_path = Path::new(&file_dir);

        let result = read_csv_file_and_return_session_data(file_path);

        assert_eq!(result, Err(ProcessingError::file("Invalid file type")));
    }
}
