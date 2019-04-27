Returns a list of announcements that the requester is permitted to view.

Course students may only view `PUBLISHED` announcements. Course teachers
and domain administrators may view all announcements.

This method returns the following error codes:

* `PERMISSION_DENIED` if the requesting user is not permitted to access
the requested course or for access errors.
* `INVALID_ARGUMENT` if the request is malformed.
* `NOT_FOUND` if the requested course does not exist.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/classroom.announcements*
* *https://www.googleapis.com/auth/classroom.announcements.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/classroom.announcements.readonly*.
You can set the scope for this method like this: `classroom1 --scope <scope> courses announcements-list ...`
# Required Scalar Argument
* **&lt;course-id&gt;** *(string)*
    - Identifier of the course.
        This identifier can be either the Classroom-assigned identifier or an
        alias.

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

* **-p announcement-states=string**
    - Restriction on the `state` of announcements returned.
        If this argument is left unspecified, the default value is `PUBLISHED`.

* **-p order-by=string**
    - Optional sort ordering for results. A comma-separated list of fields with
        an optional sort direction keyword. Supported field is `updateTime`.
        Supported direction keywords are `asc` and `desc`.
        If not specified, `updateTime desc` is the default behavior.
        Examples: `updateTime asc`, `updateTime`

* **-p page-token=string**
    - nextPageToken
        value returned from a previous
        list call,
        indicating that the subsequent page of results should be returned.
        
        The list request
        must be otherwise identical to the one that resulted in this token.

* **-p page-size=integer**
    - Maximum number of items to return. Zero or unspecified indicates that the
        server may assign a maximum.
        
        The server may return fewer than the specified number of results.

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
