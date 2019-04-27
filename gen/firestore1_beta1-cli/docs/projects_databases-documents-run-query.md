Runs a query.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/datastore*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `firestore1-beta1 --scope <scope> projects databases-documents-run-query ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - The parent resource name. In the format:
        `projects/{project_id}/databases/{database_id}/documents` or
        `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
        For example:
        `projects/my-project/databases/my-database/documents` or
        `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
RunQueryRequest:
  new-transaction:
    read-only:
      read-time: string
    read-write:
      retry-transaction: string
  read-time: string
  structured-query:
    end-at:
      before: boolean
    limit: integer
    offset: integer
    start-at:
      before: boolean
    where:
      composite-filter:
        op: string
      field-filter:
        field:
          field-path: string
        op: string
        value:
          boolean-value: boolean
          bytes-value: string
          double-value: number
          geo-point-value:
            latitude: number
            longitude: number
          integer-value: string
          null-value: string
          reference-value: string
          string-value: string
          timestamp-value: string
      unary-filter:
        field:
          field-path: string
        op: string
  transaction: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .new-transaction.read-only    read-time=aliquyam`
    - Reads documents at the given time.
        This may not be older than 60 seconds.

* `..read-write    retry-transaction=accusam`
    - An optional transaction to retry.


* `...    read-time=lorem`
    - Reads documents as they were at the given time.
        This may not be older than 60 seconds.
* `structured-query.end-at    before=true`
    - If the position is just before or just after the given values, relative
        to the sort order defined by the query.

* `..    limit=80`
    - The maximum number of results to return.
        
        Applies after all other constraints.
        Must be &gt;= 0 if specified.
* `offset=31`
    - The number of results to skip.
        
        Applies before limit, but after all other constraints. Must be &gt;= 0 if
        specified.
* `start-at    before=true`
    - If the position is just before or just after the given values, relative
        to the sort order defined by the query.

* `..where.composite-filter    op=eirmod`
    - The operator for combining multiple filters.

* `..field-filter.field    field-path=sanctus`
    - No description provided.

* `..    op=et`
    - The operator to filter by.
* `value    boolean-value=true`
    - A boolean value.
* `bytes-value=et`
    - A bytes value.
        
        Must not exceed 1 MiB - 89 bytes.
        Only the first 1,500 bytes are considered by queries.
* `double-value=0.555410189379`
    - A double value.
* `geo-point-value    latitude=0.645756686533`
    - The latitude in degrees. It must be in the range [-90.0, +90.0].
* `longitude=0.848429813172`
    - The longitude in degrees. It must be in the range [-180.0, +180.0].

* `..    integer-value=sed`
    - An integer value.
* `null-value=dolor`
    - A null value.
* `reference-value=dolor`
    - A reference to a document. For example:
        `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
* `string-value=dolor`
    - A string value.
        
        The string, represented as UTF-8, must not exceed 1 MiB - 89 bytes.
        Only the first 1,500 bytes of the UTF-8 representation are considered by
        queries.
* `timestamp-value=et`
    - A timestamp value.
        
        Precise only to microseconds. When stored, any additional precision is
        rounded down.


* `...unary-filter.field    field-path=consetetur`
    - No description provided.

* `..    op=amet.`
    - The unary operator to apply.



* `....    transaction=voluptua.`
    - Reads documents in a transaction.


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
