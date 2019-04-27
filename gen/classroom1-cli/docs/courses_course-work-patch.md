Updates one or more fields of a course work.

See google.classroom.v1.CourseWork for details
of which fields may be updated and who may change them.

This request must be made by the Developer Console project of the
[OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
create the corresponding course work item.

This method returns the following error codes:

* `PERMISSION_DENIED` if the requesting developer project did not create
the corresponding course work, if the user is not permitted to make the
requested modification to the student submission, or for
access errors.
* `INVALID_ARGUMENT` if the request is malformed.
* `FAILED_PRECONDITION` if the requested course work has already been
deleted.
* `NOT_FOUND` if the requested course, course work, or student submission
does not exist.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/classroom.coursework.students* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/classroom.coursework.students*.
You can set the scope for this method like this: `classroom1 --scope <scope> courses course-work-patch ...`
# Required Scalar Arguments
* **&lt;course-id&gt;** *(string)*
    - Identifier of the course.
        This identifier can be either the Classroom-assigned identifier or an
        alias.
* **&lt;id&gt;** *(string)*
    - Identifier of the course work.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CourseWork:
  alternate-link: string
  assignee-mode: string
  assignment:
    student-work-folder:
      alternate-link: string
      id: string
      title: string
  associated-with-developer: boolean
  course-id: string
  creation-time: string
  creator-user-id: string
  description: string
  due-date:
    day: integer
    month: integer
    year: integer
  due-time:
    hours: integer
    minutes: integer
    nanos: integer
    seconds: integer
  id: string
  individual-students-options:
    student-ids: [string]
  max-points: number
  multiple-choice-question:
    choices: [string]
  scheduled-time: string
  state: string
  submission-modification-mode: string
  title: string
  update-time: string
  work-type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    alternate-link=et`
    - Absolute link to this course work in the Classroom web UI.
        This is only populated if `state` is `PUBLISHED`.
        
        Read-only.
* `assignee-mode=consetetur`
    - Assignee mode of the coursework.
        If unspecified, the default value is `ALL_STUDENTS`.
* `assignment.student-work-folder    alternate-link=ut`
    - URL that can be used to access the Drive folder.
        
        Read-only.
* `id=ea`
    - Drive API resource ID.
* `title=sed`
    - Title of the Drive folder.
        
        Read-only.


* `...    associated-with-developer=true`
    - Whether this course work item is associated with the Developer Console
        project making the request.
        
        See google.classroom.Work.CreateCourseWork for more
        details.
        
        Read-only.
* `course-id=dolor`
    - Identifier of the course.
        
        Read-only.
* `creation-time=dolor`
    - Timestamp when this course work was created.
        
        Read-only.
* `creator-user-id=et`
    - Identifier for the user that created the coursework.
        
        Read-only.
* `description=consetetur`
    - Optional description of this course work.
        If set, the description must be a valid UTF-8 string containing no more
        than 30,000 characters.
* `due-date    day=49`
    - Day of month. Must be from 1 to 31 and valid for the year and month, or 0
        if specifying a year by itself or a year and month where the day is not
        significant.
* `month=74`
    - Month of year. Must be from 1 to 12, or 0 if specifying a year without a
        month and day.
* `year=45`
    - Year of date. Must be from 1 to 9999, or 0 if specifying a date without
        a year.

* `..due-time    hours=90`
    - Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
        to allow the value &#34;24:00:00&#34; for scenarios like business closing time.
* `minutes=81`
    - Minutes of hour of day. Must be from 0 to 59.
* `nanos=49`
    - Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
* `seconds=75`
    - Seconds of minutes of the time. Must normally be from 0 to 59. An API may
        allow the value 60 if it allows leap-seconds.

* `..    id=diam`
    - Classroom-assigned identifier of this course work, unique per course.
        
        Read-only.
* `individual-students-options    student-ids=rebum.`
    - Identifiers for the students that have access to the
        coursework/announcement.
    - Each invocation of this argument appends the given value to the array.

* `..    max-points=0.558379842329`
    - Maximum grade for this course work.
        If zero or unspecified, this assignment is considered ungraded.
        This must be a non-negative integer value.
* `multiple-choice-question    choices=sadipscing`
    - Possible choices.
    - Each invocation of this argument appends the given value to the array.

* `..    scheduled-time=vero`
    - Optional timestamp when this course work is scheduled to be published.
* `state=sadipscing`
    - Status of this course work.
        If unspecified, the default state is `DRAFT`.
* `submission-modification-mode=invidunt`
    - Setting to determine when students are allowed to modify submissions.
        If unspecified, the default value is `MODIFIABLE_UNTIL_TURNED_IN`.
* `title=consetetur`
    - Title of this course work.
        The title must be a valid UTF-8 string containing between 1 and 3000
        characters.
* `update-time=dolore`
    - Timestamp of the most recent change to this course work.
        
        Read-only.
* `work-type=duo`
    - Type of this course work.
        
        The type is set when the course work is created and cannot be changed.


### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.


# Optional Output Flags

The method's return value a JSON encoded structure, which will be written to standard output by default.

* **-o out**
    - *out* specifies the *destination* to which to write the server's result to.
      It will be a JSON-encoded structure.
      The *destination* may be `-` to indicate standard output, or a filepath that is to contain the received bytes.
      If unset, it defaults to standard output.
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p update-mask=string**
    - Mask that identifies which fields on the course work to update.
        This field is required to do an update. The update fails if invalid
        fields are specified. If a field supports empty values, it can be cleared
        by specifying it in the update mask and not in the CourseWork object. If a
        field that does not support empty values is included in the update mask and
        not set in the CourseWork object, an `INVALID_ARGUMENT` error will be
        returned.
        
        The following fields may be specified by teachers:
        
        * `title`
        * `description`
        * `state`
        * `due_date`
        * `due_time`
        * `max_points`
        * `scheduled_time`
        * `submission_modification_mode`

# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p $-xgafv=string**
    - V1 error format.

* **-p access-token=string**
    - OAuth access token.

* **-p alt=string**
    - Data format for response.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).
