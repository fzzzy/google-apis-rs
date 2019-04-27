Queries for entities.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/datastore*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `datastore1 --scope <scope> projects run-query ...`
# Required Scalar Argument
* **&lt;project-id&gt;** *(string)*
    - The ID of the project against which to make the request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
RunQueryRequest:
  gql-query:
    allow-literals: boolean
    query-string: string
  partition-id:
    namespace-id: string
    project-id: string
  query:
    end-cursor: string
    filter:
      composite-filter:
        op: string
      property-filter:
        op: string
        property:
          name: string
        value:
          blob-value: string
          boolean-value: boolean
          double-value: number
          entity-value:
            key:
              partition-id:
                namespace-id: string
                project-id: string
          exclude-from-indexes: boolean
          geo-point-value:
            latitude: number
            longitude: number
          integer-value: string
          key-value:
            partition-id:
              namespace-id: string
              project-id: string
          meaning: integer
          null-value: string
          string-value: string
          timestamp-value: string
    limit: integer
    offset: integer
    start-cursor: string
  read-options:
    read-consistency: string
    transaction: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .gql-query    allow-literals=true`
    - When false, the query string must not contain any literals and instead must
        bind all values. For example,
        `SELECT * FROM Kind WHERE a = &#39;string literal&#39;` is not allowed, while
        `SELECT * FROM Kind WHERE a = @value` is.
* `query-string=gubergren`
    - A string of the format described
        [here](https://cloud.google.com/datastore/docs/apis/gql/gql_reference).

* `..partition-id    namespace-id=sadipscing`
    - If not empty, the ID of the namespace to which the entities belong.
* `project-id=aliquyam`
    - The ID of the project to which the entities belong.

* `..query    end-cursor=ea`
    - An ending point for the query results. Query cursors are
        returned in query result batches and
        [can only be used to limit the same query](https://cloud.google.com/datastore/docs/concepts/queries#cursors_limits_and_offsets).
* `filter.composite-filter    op=no`
    - The operator for combining multiple filters.

* `..property-filter    op=justo`
    - The operator to filter by.
* `property    name=justo`
    - The name of the property.
        If name includes &#34;.&#34;s, it may be interpreted as a property name path.

* `..value    blob-value=et`
    - A blob value.
        May have at most 1,000,000 bytes.
        When `exclude_from_indexes` is false, may have at most 1500 bytes.
        In JSON requests, must be base64-encoded.
* `boolean-value=true`
    - A boolean value.
* `double-value=0.594456807662`
    - A double value.
* `entity-value.key.partition-id    namespace-id=ipsum`
    - If not empty, the ID of the namespace to which the entities belong.
* `project-id=lorem`
    - The ID of the project to which the entities belong.



* `....    exclude-from-indexes=true`
    - If the value should be excluded from all indexes including those defined
        explicitly.
* `geo-point-value    latitude=0.313727897996`
    - The latitude in degrees. It must be in the range [-90.0, +90.0].
* `longitude=0.69054137112`
    - The longitude in degrees. It must be in the range [-180.0, +180.0].

* `..    integer-value=sea`
    - An integer value.
* `key-value.partition-id    namespace-id=lorem`
    - If not empty, the ID of the namespace to which the entities belong.
* `project-id=eos`
    - The ID of the project to which the entities belong.


* `...    meaning=20`
    - The `meaning` field should only be populated for backwards compatibility.
* `null-value=sadipscing`
    - A null value.
* `string-value=dolor`
    - A UTF-8 encoded string value.
        When `exclude_from_indexes` is false (it is indexed) , may have at most 1500 bytes.
        Otherwise, may be set to at least 1,000,000 bytes.
* `timestamp-value=eirmod`
    - A timestamp value.
        When stored in the Datastore, precise only to microseconds;
        any additional precision is rounded down.



* `....    limit=58`
    - The maximum number of results to return. Applies after all other
        constraints. Optional.
        Unspecified is interpreted as no limit.
        Must be &gt;= 0 if specified.
* `offset=4`
    - The number of results to skip. Applies before limit, but after all other
        constraints. Optional. Must be &gt;= 0 if specified.
* `start-cursor=no`
    - A starting point for the query results. Query cursors are
        returned in query result batches and
        [can only be used to continue the same query](https://cloud.google.com/datastore/docs/concepts/queries#cursors_limits_and_offsets).

* `..read-options    read-consistency=labore`
    - The non-transactional read consistency to use.
        Cannot be set to `STRONG` for global queries.
* `transaction=eirmod`
    - The identifier of the transaction in which to read. A
        transaction identifier is returned by a call to
        Datastore.BeginTransaction.



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
