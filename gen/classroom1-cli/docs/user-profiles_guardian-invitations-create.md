Creates a guardian invitation, and sends an email to the guardian asking
them to confirm that they are the student&#39;s guardian.

Once the guardian accepts the invitation, their `state` will change to
`COMPLETED` and they will start receiving guardian notifications. A
`Guardian` resource will also be created to represent the active guardian.

The request object must have the `student_id` and
`invited_email_address` fields set. Failing to set these fields, or
setting any other fields in the request, will result in an error.

This method returns the following error codes:

* `PERMISSION_DENIED` if the current user does not have permission to
  manage guardians, if the guardian in question has already rejected
  too many requests for that student, if guardians are not enabled for the
  domain in question, or for other access errors.
* `RESOURCE_EXHAUSTED` if the student or guardian has exceeded the guardian
  link limit.
* `INVALID_ARGUMENT` if the guardian email address is not valid (for
  example, if it is too long), or if the format of the student ID provided
  cannot be recognized (it is not an email address, nor a `user_id` from
  this API). This error will also be returned if read-only fields are set,
  or if the `state` field is set to to a value other than `PENDING`.
* `NOT_FOUND` if the student ID provided is a valid student ID, but
  Classroom has no record of that student.
* `ALREADY_EXISTS` if there is already a pending guardian invitation for
  the student and `invited_email_address` provided, or if the provided
  `invited_email_address` matches the Google account of an existing
  `Guardian` for this user.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/classroom.guardianlinks.students* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/classroom.guardianlinks.students*.
You can set the scope for this method like this: `classroom1 --scope <scope> user-profiles guardian-invitations-create ...`
# Required Scalar Argument
* **&lt;student-id&gt;** *(string)*
    - ID of the student (in standard format)
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GuardianInvitation:
  creation-time: string
  invitation-id: string
  invited-email-address: string
  state: string
  student-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    creation-time=et`
    - The time that this invitation was created.
        
        Read-only.
* `invitation-id=nonumy`
    - Unique identifier for this invitation.
        
        Read-only.
* `invited-email-address=et`
    - Email address that the invitation was sent to.
        This field is only visible to domain administrators.
* `state=sed`
    - The state that this invitation is in.
* `student-id=no`
    - ID of the student (in standard format)


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
