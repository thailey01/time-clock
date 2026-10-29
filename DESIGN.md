# Design

## Storage

1. One sessions file. This gives us a significantly simpler option for finding/reading rows in our data as they all exist in the same source file. Additionally, storing files in folders/files labeled by year/month/day would introduce a timezone issue, such as: user recording clock in of 12:00 AM and then moving to a timezone behind previous one. How would that day be properly recorded using year/month/day namings?

2. A separate projects file with IDs; sessions point to projects by ID. This gives us the opportunity to more easily manage changing a project name for multiple entries of clock in and can allow us to easily associate more data with projects in the future.

3. Session CSVs will be formatted with the following headers: id, start_time, end_time, project_id. Timestamps should use seconds precision for record keeping. This data will be represented by a non-negative whole number of seconds since 1970-01-01 UTC. Billing is generally rounded by minutes, and seconds will give us the precisions to know how to round. end_time is allowed to be empty (see Validation: open sessions). project_id is a foreign key reference and allowed to be empty for a session entry. Users should not be required to pick a project in order to track their time.

4. Project CSVs will be formatted with the following headers: id, name.

5. Timestamps stored in UTC; converted to local time only for display. Keeping a timezone independent timestamp is standard for stored data and helpful for when situations come up when a users local timezone may change. However, showing users the UTC timestamp isn't particularly helpful for them, so showing them a local representation of their time will be necessary.

6. If no file exists for session data while trying to read from it, one should be created. Storage is singularly responsible for reading/writing data to files and can return an empty list of sessions to its caller to indicate no sessions exist while now having a file for them. Separate functions should exist for reading and writing to adhere to separation of concerns.

## Validation

1. A second clock-in while one is open is refused, with an explanation. Creating conflicting open clock ins should not be allowed for reasons of management clarity.

2. A clock-in time should always indicate a point in time before or equivalent to a clock-out time for an entry. Time only moves forward. Setting clock-out values for points in time before clock-in values should raise flags during editing and clock-out operations. We cannot prevent direct edits to the csv, and thus, should still rely on validation checks of the data before reporting actions. If errant entries are found, the time-period that covers the errant entry should be indicated in the report. For instance, if reporting on "total time worked" for a day, and that day contains an errant entry, the user should be given the information that that day had invalid data. Same goes for weeks/months/years.

3. Open sessions are allowed. Open sessions are defined as sessions with a valid clock in timestamp, but no clock out timestamp. Open sessions allow for operations on incompleted data for purposes of retrieving which session may be "active".

4. Every session is created via validation. Valid sessions are created when all data follows the rules that govern them. Invalid sessions are kept, not dropped, as an error variant including a cause. This allows reports to flag time periods and give indications on why validation failed.

5. Session validation rules will exist in a single location inside a sessions module. This should allow us to easily maintain and access these rules from anywhere in our application that needs them.

6. When determining what id should be used for a new row, default to the next greatest integer in the file. This will ensure uniqueness and avoid collisions with new id values.

## Reporting

1. A session that moves from one day to another should be split at a users local midnight during display. This attempts to give a more accurate tracking of time worked for each day than giving all time to the starting or ending day.

2. Times are displayed in local time (see Storage: timestamps in UTC).

## Architecture

1. Program logic lives in the library; main.rs is just a thin entry point. This gives us an opportunity to expose our application logic in a single convenient location, while ensuring that the orchestrator of our application uses only what is needed.

2. Logic regarding storage operations should be isolated from application/business logic. This gives flexibility to our application in regards to operating on data without regard to where the data came from. Enforces separation of concerns.

3. Commands services are responsible for coordinating storage actions and session rules. This service will be singularly responsible for the connection between what data exists, what can be done with it, and returning results to its caller.

## Open Questions:
- What does deleting a project mean?
- Where do data files live on each OS?
- What should reports do with rows that couldn't be parsed at all (no usable start time)?
- How should project_id be linked by users
