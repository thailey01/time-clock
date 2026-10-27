pub enum SessionErrorType {
    InvalidTimeBounds,
}

pub struct SessionData {
    id: u32,
    start: u64,
    end: Option<u64>,
    project_id: Option<u32>,
}

pub enum Session {
    Success(SessionData),
    Error(SessionData, SessionErrorType),
}

impl SessionData {
    pub fn validation(self) -> Session {
        let bad_time_bounds = self.end.is_some_and(|end| end < self.start);

        if bad_time_bounds {
            return Session::Error(self, SessionErrorType::InvalidTimeBounds);
        }

        Session::Success(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_including_all_data() {
        let session_data = SessionData {
            id: 1,
            start: 19091009,
            end: Some(19091010),
            project_id: Some(2),
        };
        let session: Session = session_data.validation();

        assert!(matches!(session, Session::Success(_)));
    }

    #[test]
    fn error_end_before_start() {
        let session_data = SessionData {
            id: 1,
            start: 19091009,
            end: Some(19091008),
            project_id: Some(2),
        };
        let session: Session = session_data.validation();

        assert!(matches!(
            session,
            Session::Error(_, SessionErrorType::InvalidTimeBounds)
        ));
    }

    #[test]
    fn sucess_missing_end() {
        let session_data = SessionData {
            id: 1,
            start: 19091009,
            end: None,
            project_id: Some(2),
        };
        let session: Session = session_data.validation();

        assert!(matches!(session, Session::Success(_)));
    }

    #[test]
    fn success_same_start_end() {
        let session_data = SessionData {
            id: 1,
            start: 19091009,
            end: Some(19091009),
            project_id: Some(2),
        };
        let session: Session = session_data.validation();

        assert!(matches!(session, Session::Success(_)));
    }
}
