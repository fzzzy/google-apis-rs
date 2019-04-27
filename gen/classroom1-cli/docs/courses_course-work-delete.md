Deletes a course work.

This request must be made by the Developer Console project of the
[OAuth client ID](https://support.google.com/cloud/answer/6158849) used to
create the corresponding course work item.

This method returns the following error codes:

* `PERMISSION_DENIED` if the requesting developer project did not create
the corresponding course work, if the requesting user is not permitted
to delete the requested course or for access errors.
* `FAILED_PRECONDITION` if the requested course work has already been
deleted.
* `NOT_FOUND` if no course exists with the requested ID.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/classroom.coursework.students* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/classroom.coursework.students*.
You can set the scope for this method like this: `classroom1 --scope <scope> courses course-work-delete ...`
# Required Scalar Arguments
* **&lt;course-id&gt;** *(string)*
    - Identifier of the course.
        This identifier can be either the Classroom-assigned identifier or an
        alias.
* **&lt;id&gt;** *(string)*
    - Identifier of the course work to delete.
        This identifier is a Classroom-assigned identifier.

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
