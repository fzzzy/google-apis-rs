Updates one or more fields of a student submission.

See google.classroom.v1.StudentSubmission for details
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
* `NOT_FOUND` if the requested course, course work, or student submission
does not exist.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/classroom.coursework.me*
* *https://www.googleapis.com/auth/classroom.coursework.students*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/classroom.coursework.me*.
You can set the scope for this method like this: `classroom1 --scope <scope> courses course-work-student-submissions-patch ...`
# Required Scalar Arguments
* **&lt;course-id&gt;** *(string)*
    - Identifier of the course.
        This identifier can be either the Classroom-assigned identifier or an
        alias.
* **&lt;course-work-id&gt;** *(string)*
    - Identifier of the course work.
* **&lt;id&gt;** *(string)*
    - Identifier of the student submission.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
StudentSubmission:
  alternate-link: string
  assigned-grade: number
  associated-with-developer: boolean
  course-id: string
  course-work-id: string
  course-work-type: string
  creation-time: string
  draft-grade: number
  id: string
  late: boolean
  multiple-choice-submission:
    answer: string
  short-answer-submission:
    answer: string
  state: string
  update-time: string
  user-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    alternate-link=aliquyam`
    - Absolute link to the submission in the Classroom web UI.
        
        Read-only.
* `assigned-grade=0.952666577253`
    - Optional grade. If unset, no grade was set.
        This value must be non-negative. Decimal (i.e. non-integer) values are
        allowed, but will be rounded to two decimal places.
        
        This may be modified only by course teachers.
* `associated-with-developer=true`
    - Whether this student submission is associated with the Developer Console
        project making the request.
        
        See google.classroom.Work.CreateCourseWork for more
        details.
        
        Read-only.
* `course-id=clita`
    - Identifier of the course.
        
        Read-only.
* `course-work-id=consetetur`
    - Identifier for the course work this corresponds to.
        
        Read-only.
* `course-work-type=takimata`
    - Type of course work this submission is for.
        
        Read-only.
* `creation-time=nonumy`
    - Creation time of this submission.
        This may be unset if the student has not accessed this item.
        
        Read-only.
* `draft-grade=0.881135671636`
    - Optional pending grade. If unset, no grade was set.
        This value must be non-negative. Decimal (i.e. non-integer) values are
        allowed, but will be rounded to two decimal places.
        
        This is only visible to and modifiable by course teachers.
* `id=sanctus`
    - Classroom-assigned Identifier for the student submission.
        This is unique among submissions for the relevant course work.
        
        Read-only.
* `late=false`
    - Whether this submission is late.
        
        Read-only.
* `multiple-choice-submission    answer=at`
    - Student&#39;s select choice.

* `..short-answer-submission    answer=labore`
    - Student response to a short-answer question.

* `..    state=invidunt`
    - State of this submission.
        
        Read-only.
* `update-time=ea`
    - Last update time of this submission.
        This may be unset if the student has not accessed this item.
        
        Read-only.
* `user-id=sadipscing`
    - Identifier for the student that owns this submission.
        
        Read-only.


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
    - Mask that identifies which fields on the student submission to update.
        This field is required to do an update. The update fails if invalid
        fields are specified.
        
        The following fields may be specified by teachers:
        
        * `draft_grade`
        * `assigned_grade`

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
