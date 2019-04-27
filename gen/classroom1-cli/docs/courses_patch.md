Updates one or more fields in a course.

This method returns the following error codes:

* `PERMISSION_DENIED` if the requesting user is not permitted to modify the
requested course or for access errors.
* `NOT_FOUND` if no course exists with the requested ID.
* `INVALID_ARGUMENT` if invalid fields are specified in the update mask or
if no update mask is supplied.
* `FAILED_PRECONDITION` for the following request errors:
    * CourseNotModifiable
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/classroom.courses* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/classroom.courses*.
You can set the scope for this method like this: `classroom1 --scope <scope> courses patch ...`
# Required Scalar Argument
* **&lt;id&gt;** *(string)*
    - Identifier of the course to update.
        This identifier can be either the Classroom-assigned identifier or an
        alias.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Course:
  alternate-link: string
  calendar-id: string
  course-group-email: string
  course-state: string
  creation-time: string
  description: string
  description-heading: string
  enrollment-code: string
  guardians-enabled: boolean
  id: string
  name: string
  owner-id: string
  room: string
  section: string
  teacher-folder:
    alternate-link: string
    id: string
    title: string
  teacher-group-email: string
  update-time: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    alternate-link=dolores`
    - Absolute link to this course in the Classroom web UI.
        
        Read-only.
* `calendar-id=sit`
    - The Calendar ID for a calendar that all course members can see, to which
        Classroom adds events for course work and announcements in the course.
        
        Read-only.
* `course-group-email=diam`
    - The email address of a Google group containing all members of the course.
        This group does not accept email and can only be used for permissions.
        
        Read-only.
* `course-state=ut`
    - State of the course.
        If unspecified, the default state is `PROVISIONED`.
* `creation-time=justo`
    - Creation time of the course.
        Specifying this field in a course update mask results in an error.
        
        Read-only.
* `description=est`
    - Optional description.
        For example, &#34;We&#39;ll be learning about the structure of living
        creatures from a combination of textbooks, guest lectures, and lab work.
        Expect to be excited!&#34;
        If set, this field must be a valid UTF-8 string and no longer than 30,000
        characters.
* `description-heading=amet`
    - Optional heading for the description.
        For example, &#34;Welcome to 10th Grade Biology.&#34;
        If set, this field must be a valid UTF-8 string and no longer than 3600
        characters.
* `enrollment-code=accusam`
    - Enrollment code to use when joining this course.
        Specifying this field in a course update mask results in an error.
        
        Read-only.
* `guardians-enabled=true`
    - Whether or not guardian notifications are enabled for this course.
        
        Read-only.
* `id=diam`
    - Identifier for this course assigned by Classroom.
        
        When
        creating a course,
        you may optionally set this identifier to an
        alias string in the
        request to create a corresponding alias. The `id` is still assigned by
        Classroom and cannot be updated after the course is created.
        
        Specifying this field in a course update mask results in an error.
* `name=justo`
    - Name of the course.
        For example, &#34;10th Grade Biology&#34;.
        The name is required. It must be between 1 and 750 characters and a valid
        UTF-8 string.
* `owner-id=est`
    - The identifier of the owner of a course.
        
        When specified as a parameter of a
        create course request, this
        field is required.
        The identifier can be one of the following:
        
        * the numeric identifier for the user
        * the email address of the user
        * the string literal `&#34;me&#34;`, indicating the requesting user
        
        This must be set in a create request. Admins can also specify this field
        in a patch course request to
        transfer ownership. In other contexts, it is read-only.
* `room=clita`
    - Optional room location.
        For example, &#34;301&#34;.
        If set, this field must be a valid UTF-8 string and no longer than 650
        characters.
* `section=invidunt`
    - Section of the course.
        For example, &#34;Period 2&#34;.
        If set, this field must be a valid UTF-8 string and no longer than 2800
        characters.
* `teacher-folder    alternate-link=ut`
    - URL that can be used to access the Drive folder.
        
        Read-only.
* `id=dolores`
    - Drive API resource ID.
* `title=eos`
    - Title of the Drive folder.
        
        Read-only.

* `..    teacher-group-email=voluptua.`
    - The email address of a Google group containing all teachers of the course.
        This group does not accept email and can only be used for permissions.
        
        Read-only.
* `update-time=duo`
    - Time of the most recent update to this course.
        Specifying this field in a course update mask results in an error.
        
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
    - Mask that identifies which fields on the course to update.
        This field is required to do an update. The update will fail if invalid
        fields are specified. The following fields are valid:
        
        * `name`
        * `section`
        * `descriptionHeading`
        * `description`
        * `room`
        * `courseState`
        * `ownerId`
        
        Note: patches to ownerId are treated as being effective immediately, but in
        practice it may take some time for the ownership transfer of all affected
        resources to complete.
        
        When set in a query parameter, this field should be specified as
        
        `updateMask=&lt;field1&gt;,&lt;field2&gt;,...`

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
