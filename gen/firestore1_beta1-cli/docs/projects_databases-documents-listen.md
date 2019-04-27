Listens to changes.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/datastore*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `firestore1-beta1 --scope <scope> projects databases-documents-listen ...`
# Required Scalar Argument
* **&lt;database&gt;** *(string)*
    - The database name. In the format:
        `projects/{project_id}/databases/{database_id}`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ListenRequest:
  add-target:
    documents:
      documents: [string]
    once: boolean
    query:
      parent: string
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
    read-time: string
    resume-token: string
    target-id: integer
  labels: { string: string }
  remove-target: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .add-target.documents    documents=nonumy`
    - The names of the documents to retrieve. In the format:
        `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
        The request will fail if any of the document is not a child resource of
        the given `database`. Duplicate names will be elided.
    - Each invocation of this argument appends the given value to the array.

* `..    once=true`
    - If the target should be removed once it is current and consistent.
* `query    parent=gubergren`
    - The parent resource name. In the format:
        `projects/{project_id}/databases/{database_id}/documents` or
        `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
        For example:
        `projects/my-project/databases/my-database/documents` or
        `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`
* `structured-query.end-at    before=false`
    - If the position is just before or just after the given values, relative
        to the sort order defined by the query.

* `..    limit=70`
    - The maximum number of results to return.
        
        Applies after all other constraints.
        Must be &gt;= 0 if specified.
* `offset=35`
    - The number of results to skip.
        
        Applies before limit, but after all other constraints. Must be &gt;= 0 if
        specified.
* `start-at    before=false`
    - If the position is just before or just after the given values, relative
        to the sort order defined by the query.

* `..where.composite-filter    op=justo`
    - The operator for combining multiple filters.

* `..field-filter.field    field-path=justo`
    - No description provided.

* `..    op=et`
    - The operator to filter by.
* `value    boolean-value=true`
    - A boolean value.
* `bytes-value=diam`
    - A bytes value.
        
        Must not exceed 1 MiB - 89 bytes.
        Only the first 1,500 bytes are considered by queries.
* `double-value=0.460933679688`
    - A double value.
* `geo-point-value    latitude=0.957563386643`
    - The latitude in degrees. It must be in the range [-90.0, +90.0].
* `longitude=0.795720263212`
    - The longitude in degrees. It must be in the range [-180.0, +180.0].

* `..    integer-value=duo`
    - An integer value.
* `null-value=aliquyam`
    - A null value.
* `reference-value=sea`
    - A reference to a document. For example:
        `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
* `string-value=lorem`
    - A string value.
        
        The string, represented as UTF-8, must not exceed 1 MiB - 89 bytes.
        Only the first 1,500 bytes of the UTF-8 representation are considered by
        queries.
* `timestamp-value=eos`
    - A timestamp value.
        
        Precise only to microseconds. When stored, any additional precision is
        rounded down.


* `...unary-filter.field    field-path=erat`
    - No description provided.

* `..    op=sadipscing`
    - The unary operator to apply.




* `.....    read-time=dolor`
    - Start listening after a specific `read_time`.
        
        The client must know the state of matching documents at this time.
* `resume-token=eirmod`
    - A resume token from a prior TargetChange for an identical target.
        
        Using a resume token with a different target is unsupported and may fail.
* `target-id=58`
    - A client provided target ID.
        
        If not set, the server will assign an ID for the target.
        
        Used for resuming a target without changing IDs. The IDs can either be
        client-assigned or be server-assigned in a previous stream. All targets
        with client provided IDs must be added before adding a target that needs
        a server-assigned id.

* `..    labels=key=amet`
    - Labels associated with this target change.
    - the value will be associated with the given `key`
* `remove-target=41`
    - The ID of a target to remove from this stream.


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
