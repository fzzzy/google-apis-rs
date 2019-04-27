Creates a new session. A session can be used to perform
transactions that read and/or modify data in a Cloud Spanner database.
Sessions are meant to be reused for many consecutive
transactions.

Sessions can only execute one transaction at a time. To execute
multiple concurrent read-write/write-only transactions, create
multiple sessions. Note that standalone reads and queries use a
transaction internally, and count toward the one transaction
limit.

Cloud Spanner limits the number of sessions that can exist at any given
time; thus, it is a good idea to delete idle and/or unneeded sessions.
Aside from explicit deletes, Cloud Spanner can delete sessions for which no
operations are sent for more than an hour. If a session is deleted,
requests to it return `NOT_FOUND`.

Idle sessions can be kept alive by sending a trivial SQL query
periodically, e.g., `&#34;SELECT 1&#34;`.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/spanner.data*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `spanner1 --scope <scope> projects instances-databases-sessions-create ...`
# Required Scalar Argument
* **&lt;database&gt;** *(string)*
    - Required. The database in which the new session is created.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CreateSessionRequest:
  session:
    approximate-last-use-time: string
    create-time: string
    labels: { string: string }
    name: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .session    approximate-last-use-time=justo`
    - Output only. The approximate timestamp when the session is last used. It is
        typically earlier than the actual last use time.
* `create-time=et`
    - Output only. The timestamp when the session is created.
* `labels=key=et`
    - The labels for the session.
        
         * Label keys must be between 1 and 63 characters long and must conform to
           the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`.
         * Label values must be between 0 and 63 characters long and must conform
           to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`.
         * No more than 64 labels can be associated with a given session.
        
        See https://goo.gl/xmQnxf for more information on and examples of labels.
    - the value will be associated with the given `key`
* `name=diam`
    - The name of the session. This is always system-assigned; values provided
        when creating a session are ignored.



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
