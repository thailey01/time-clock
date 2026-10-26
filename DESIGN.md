1. Timestamps stored in UTC; converted to local time only for display. Keeping a timezone independent timestamp is standard for stored data and helpful for when situations come up when a users local timezone may change. However, showing users the UTC timestamp isn't particularly helpful for them, so showing them a local representation of their time will be necessary.

2. One sessions file. This gives us a significantly simpler option for finding/reading rows in our data as they all exist in the same source file. Additionally, storing files in folders/files labeled by year/month/day would reintroduce a timezone issue that we handled with decision 1, such as: user recording clock in of 12:00 AM and then moving to a timezone behind previous one. How would that day be properly recorded using year/month/day namings?

3. A second clock-in while one is open is refused, with an explanation. Creating conflicting open clock ins should not be allowed for reasons of management clarity.

4. A separate projects file with IDs; sessinos point to projects by ID. This gives us the opportunity to more easily manage changing a project name for multiple entries of clock in and can allow us to easily associate more data with projects in the future.

5. A session that moves from one day to another should be split at a users local midnight during display. This attempts to give a more accurate tracking of time worked for each day than giving all time to the starting or ending day.

6. A project id should be optional for clock ins. User's should not be required to pick a project in order to track their time, but it should be an option for convenience.

7. Timestamps should use seconds precision for record keeping. Billing is generally rounded by minutes, and seconds will give us the precisions to know how to round.

8. A clock-in time should always indicate a point in time before a clock-out time for an entry. Time only moves forward, so physics will never allow for a start time to occur after an end time. Setting clock-out values for points in time before clock-in values should raise flags during editing and clock-out operations. We cannot prevent direct edits to the csv, and thus, should still rely on validation checks of the data before reporting actions. If errant entries are found, the time-period that covers the errant entry should be indicated in the report. For instance, if reporting on total time worked for a day, and that day contains an errant entry, the user should be given the information that that day had invalid data. Same goes for weeks/months/years.
