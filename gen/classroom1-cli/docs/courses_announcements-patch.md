Updates one or more fields of an announcement.

This method returns the following error codes:

* `PERMISSION_DENIED` if the requesting developer project did not create
the corresponding announcement or for access errors.
* `INVALID_ARGUMENT` if the request is malformed.
* `FAILED_PRECONDITION` if the requested announcement has already been
deleted.
* `NOT_FOUND` if the requested course or announcement does not exist
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/classroom.announcements* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/classroom.announcements*.
You can set the scope for this method like this: `classroom1 --scope <scope> courses announcements-patch ...`
# Required Scalar Arguments
* **&lt;course-id&gt;** *(string)*
    - Identifier of the course.
        This identifier can be either the Classroom-assigned identifier or an
        alias.
* **&lt;id&gt;** *(string)*
    - Identifier of the announcement.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Announcement:
  alternate-link: string
  assignee-mode: string
  course-id: string
  creation-time: string
  creator-user-id: string
  id: string
  individual-students-options:
    student-ids: [string]
  scheduled-time: string
  state: string
  text: string
  update-time: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    alternate-link=dolores`
    - Absolute link to this announcement in the Classroom web UI.
        This is only populated if `state` is `PUBLISHED`.
        
        Read-only.
* `assignee-mode=gubergren`
    - Assignee mode of the announcement.
        If unspecified, the default value is `ALL_STUDENTS`.
* `course-id=sadipscing`
    - Identifier of the course.
        
        Read-only.
* `creation-time=aliquyam`
    - Timestamp when this announcement was created.
        
        Read-only.
* `creator-user-id=ea`
    - Identifier for the user that created the announcement.
        
        Read-only.
* `id=no`
    - Classroom-assigned identifier of this announcement, unique per course.
        
        Read-only.
* `individual-students-options    student-ids=justo`
    - Identifiers for the students that have access to the
        coursework/announcement.
    - Each invocation of this argument appends the given value to the array.

* `..    scheduled-time=justo`
    - Optional timestamp when this announcement is scheduled to be published.
* `state=et`
    - Status of this announcement.
        If unspecified, the default state is `DRAFT`.
* `text=et`
    - Description of this announcement.
        The text must be a valid UTF-8 string containing no more
        than 30,000 characters.
* `update-time=diam`
    - Timestamp of the most recent change to this announcement.
        
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
    - Mask that identifies which fields on the announcement to update.
        This field is required to do an update. The update fails if invalid
        fields are specified. If a field supports empty values, it can be cleared
        by specifying it in the update mask and not in the Announcement object. If
        a field that does not support empty values is included in the update mask
        and not set in the Announcement object, an `INVALID_ARGUMENT` error will be
        returned.
        
        The following fields may be specified by teachers:
        
        * `text`
        * `state`
        * `scheduled_time`

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
