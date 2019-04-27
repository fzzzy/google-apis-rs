Creates a teacher of a course.

This method returns the following error codes:

* `PERMISSION_DENIED` if the requesting user is not  permitted to create
teachers in this course or for access errors.
* `NOT_FOUND` if the requested course ID does not exist.
* `FAILED_PRECONDITION` if the requested user&#39;s account is disabled,
for the following request errors:
    * CourseMemberLimitReached
    * CourseNotModifiable
    * CourseTeacherLimitReached
    * UserGroupsMembershipLimitReached
* `ALREADY_EXISTS` if the user is already a teacher or student in the
course.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/classroom.profile.emails*
* *https://www.googleapis.com/auth/classroom.profile.photos*
* *https://www.googleapis.com/auth/classroom.rosters*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/classroom.profile.emails*.
You can set the scope for this method like this: `classroom1 --scope <scope> courses teachers-create ...`
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
Teacher:
  course-id: string
  profile:
    email-address: string
    id: string
    name:
      family-name: string
      full-name: string
      given-name: string
    photo-url: string
    verified-teacher: boolean
  user-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    course-id=invidunt`
    - Identifier of the course.
        
        Read-only.
* `profile    email-address=eirmod`
    - Email address of the user.
        
        Read-only.
* `id=at`
    - Identifier of the user.
        
        Read-only.
* `name    family-name=consetetur`
    - The user&#39;s last name.
        
        Read-only.
* `full-name=et`
    - The user&#39;s full name formed by concatenating the first and last name
        values.
        
        Read-only.
* `given-name=sed`
    - The user&#39;s first name.
        
        Read-only.

* `..    photo-url=sit`
    - URL of user&#39;s profile photo.
        
        Read-only.
* `verified-teacher=true`
    - Represents whether a G Suite for Education user&#39;s domain administrator has
        explicitly verified them as being a teacher. If the user is not a member of
        a G Suite for Education domain, than this field will always be false.
        
        Read-only

* `..    user-id=elitr`
    - Identifier of the user.
        
        When specified as a parameter of a request, this identifier can be one of
        the following:
        
        * the numeric identifier for the user
        * the email address of the user
        * the string literal `&#34;me&#34;`, indicating the requesting user


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
