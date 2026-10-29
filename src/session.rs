#[derive(PartialEq, Debug)]
pub enum SessionErrorType {
    InvalidTimeBounds,
}

#[derive(PartialEq, Debug)]
pub struct SessionData {
    id: u32,
    start: u64,
    end: Option<u64>,
    project_id: Option<u32>,
}

#[derive(PartialEq, Debug)]
pub enum Session {
    Success(SessionData),
    Error(SessionData, SessionErrorType),
}

impl Session {
    pub fn new(id: u32, start: u64, end: Option<u64>, project_id: Option<u32>) -> Session {
        let data = SessionData {
            id,
            start,
            end,
            project_id,
        };

        Session::validation(data)
    }

    fn validation(data: SessionData) -> Session {
        let bad_time_bounds = data.end.is_some_and(|end| end < data.start);

        if bad_time_bounds {
            return Session::Error(data, SessionErrorType::InvalidTimeBounds);
        }

        Session::Success(data)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn success_including_all_data() {
        let session: Session = Session::new(1, 19091009, Some(19091010), Some(2));

        assert_matches!(session, Session::Success(_));
    }

    #[test]
    fn error_end_before_start() {
        let session: Session = Session::new(1, 19091009, Some(19091008), Some(2));

        assert_matches!(
            session,
            Session::Error(_, SessionErrorType::InvalidTimeBounds)
        );
    }

    #[test]
    fn sucess_missing_end() {
        let session: Session = Session::new(1, 19091009, None, Some(2));

        assert_matches!(session, Session::Success(_));
    }

    #[test]
    fn success_same_start_end() {
        let session: Session = Session::new(1, 19091009, Some(19091009), Some(2));

        assert_matches!(session, Session::Success(_));
    }
}
