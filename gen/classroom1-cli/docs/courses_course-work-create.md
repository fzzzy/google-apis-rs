Creates course work.

The resulting course work (and corresponding student submissions) are
associated with the Developer Console project of the
[OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
make the request. Classroom API requests to modify course work and student
submissions must be made with an OAuth client ID from the associated
Developer Console project.

This method returns the following error codes:

* `PERMISSION_DENIED` if the requesting user is not permitted to access the
requested course, create course work in the requested course, share a
Drive attachment, or for access errors.
* `INVALID_ARGUMENT` if the request is malformed.
* `NOT_FOUND` if the requested course does not exist.
* `FAILED_PRECONDITION` for the following request error:
    * AttachmentNotVisible
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/classroom.coursework.students* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/classroom.coursework.students*.
You can set the scope for this method like this: `classroom1 --scope <scope> courses course-work-create ...`
# Required Scalar Argument
* **&lt;course-id&gt;** *(string)*
    - Identifier of the course.
        This identifier can be either the Classroom-assigned identifier or an
        alias.
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

* `-r .    alternate-link=ipsum`
    - Absolute link to this course work in the Classroom web UI.
        This is only populated if `state` is `PUBLISHED`.
        
        Read-only.
* `assignee-mode=lorem`
    - Assignee mode of the coursework.
        If unspecified, the default value is `ALL_STUDENTS`.
* `assignment.student-work-folder    alternate-link=et`
    - URL that can be used to access the Drive folder.
        
        Read-only.
* `id=duo`
    - Drive API resource ID.
* `title=aliquyam`
    - Title of the Drive folder.
        
        Read-only.


* `...    associated-with-developer=true`
    - Whether this course work item is associated with the Developer Console
        project making the request.
        
        See google.classroom.Work.CreateCourseWork for more
        details.
        
        Read-only.
* `course-id=lorem`
    - Identifier of the course.
        
        Read-only.
* `creation-time=eos`
    - Timestamp when this course work was created.
        
        Read-only.
* `creator-user-id=erat`
    - Identifier for the user that created the coursework.
        
        Read-only.
* `description=sadipscing`
    - Optional description of this course work.
        If set, the description must be a valid UTF-8 string containing no more
        than 30,000 characters.
* `due-date    day=53`
    - Day of month. Must be from 1 to 31 and valid for the year and month, or 0
        if specifying a year by itself or a year and month where the day is not
        significant.
* `month=62`
    - Month of year. Must be from 1 to 12, or 0 if specifying a year without a
        month and day.
* `year=58`
    - Year of date. Must be from 1 to 9999, or 0 if specifying a date without
        a year.

* `..due-time    hours=4`
    - Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
        to allow the value &#34;24:00:00&#34; for scenarios like business closing time.
* `minutes=41`
    - Minutes of hour of day. Must be from 0 to 59.
* `nanos=65`
    - Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
* `seconds=62`
    - Seconds of minutes of the time. Must normally be from 0 to 59. An API may
        allow the value 60 if it allows leap-seconds.

* `..    id=dolore`
    - Classroom-assigned identifier of this course work, unique per course.
        
        Read-only.
* `individual-students-options    student-ids=invidunt`
    - Identifiers for the students that have access to the
        coursework/announcement.
    - Each invocation of this argument appends the given value to the array.

* `..    max-points=0.197022051398`
    - Maximum grade for this course work.
        If zero or unspecified, this assignment is considered ungraded.
        This must be a non-negative integer value.
* `multiple-choice-question    choices=accusam`
    - Possible choices.
    - Each invocation of this argument appends the given value to the array.

* `..    scheduled-time=lorem`
    - Optional timestamp when this course work is scheduled to be published.
* `state=sea`
    - Status of this course work.
        If unspecified, the default state is `DRAFT`.
* `submission-modification-mode=et`
    - Setting to determine when students are allowed to modify submissions.
        If unspecified, the default value is `MODIFIABLE_UNTIL_TURNED_IN`.
* `title=duo`
    - Title of this course work.
        The title must be a valid UTF-8 string containing between 1 and 3000
        characters.
* `update-time=et`
    - Timestamp of the most recent change to this course work.
        
        Read-only.
* `work-type=eirmod`
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
