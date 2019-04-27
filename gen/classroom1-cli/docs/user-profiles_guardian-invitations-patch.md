Modifies a guardian invitation.

Currently, the only valid modification is to change the `state` from
`PENDING` to `COMPLETE`. This has the effect of withdrawing the invitation.

This method returns the following error codes:

* `PERMISSION_DENIED` if the current user does not have permission to
  manage guardians, if guardians are not enabled for the domain in question
  or for other access errors.
* `FAILED_PRECONDITION` if the guardian link is not in the `PENDING` state.
* `INVALID_ARGUMENT` if the format of the student ID provided
  cannot be recognized (it is not an email address, nor a `user_id` from
  this API), or if the passed `GuardianInvitation` has a `state` other than
  `COMPLETE`, or if it modifies fields other than `state`.
* `NOT_FOUND` if the student ID provided is a valid student ID, but
  Classroom has no record of that student, or if the `id` field does not
  refer to a guardian invitation known to Classroom.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/classroom.guardianlinks.students* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/classroom.guardianlinks.students*.
You can set the scope for this method like this: `classroom1 --scope <scope> user-profiles guardian-invitations-patch ...`
# Required Scalar Arguments
* **&lt;student-id&gt;** *(string)*
    - The ID of the student whose guardian invitation is to be modified.
* **&lt;invitation-id&gt;** *(string)*
    - The `id` field of the `GuardianInvitation` to be modified.
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

* `-r .    creation-time=invidunt`
    - The time that this invitation was created.
        
        Read-only.
* `invitation-id=rebum.`
    - Unique identifier for this invitation.
        
        Read-only.
* `invited-email-address=labore`
    - Email address that the invitation was sent to.
        This field is only visible to domain administrators.
* `state=aliquyam`
    - The state that this invitation is in.
* `student-id=elitr`
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
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p update-mask=string**
    - Mask that identifies which fields on the course to update.
        This field is required to do an update. The update will fail if invalid
        fields are specified. The following fields are valid:
        
        * `state`
        
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
