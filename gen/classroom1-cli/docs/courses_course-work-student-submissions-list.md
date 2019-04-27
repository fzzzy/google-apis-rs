Returns a list of student submissions that the requester is permitted to
view, factoring in the OAuth scopes of the request.
`-` may be specified as the `course_work_id` to include student
submissions for multiple course work items.

Course students may only view their own work. Course teachers
and domain administrators may view all student submissions.

This method returns the following error codes:

* `PERMISSION_DENIED` if the requesting user is not permitted to access the
requested course or course work, or for access errors.
* `INVALID_ARGUMENT` if the request is malformed.
* `NOT_FOUND` if the requested course does not exist.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/classroom.coursework.me*
* *https://www.googleapis.com/auth/classroom.coursework.me.readonly*
* *https://www.googleapis.com/auth/classroom.coursework.students*
* *https://www.googleapis.com/auth/classroom.coursework.students.readonly*
* *https://www.googleapis.com/auth/classroom.student-submissions.me.readonly*
* *https://www.googleapis.com/auth/classroom.student-submissions.students.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/classroom.coursework.me.readonly*.
You can set the scope for this method like this: `classroom1 --scope <scope> courses course-work-student-submissions-list ...`
# Required Scalar Arguments
* **&lt;course-id&gt;** *(string)*
    - Identifier of the course.
        This identifier can be either the Classroom-assigned identifier or an
        alias.
* **&lt;course-work-id&gt;** *(string)*
    - Identifier of the student work to request.
        This may be set to the string literal `&#34;-&#34;` to request student work for
        all course work in the specified course.

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

* **-p page-size=integer**
    - Maximum number of items to return. Zero or unspecified indicates that the
        server may assign a maximum.
        
        The server may return fewer than the specified number of results.

* **-p late=string**
    - Requested lateness value. If specified, returned student submissions are
        restricted by the requested value.
        If unspecified, submissions are returned regardless of `late` value.

* **-p states=string**
    - Requested submission states. If specified, returned student submissions
        match one of the specified submission states.

* **-p user-id=string**
    - Optional argument to restrict returned student work to those owned by the
        student with the specified identifier. The identifier can be one of the
        following:
        
        * the numeric identifier for the user
        * the email address of the user
        * the string literal `&#34;me&#34;`, indicating the requesting user

* **-p page-token=string**
    - nextPageToken
        value returned from a previous
        list call,
        indicating that the subsequent page of results should be returned.
        
        The list request
        must be otherwise identical to the one that resulted in this token.

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
