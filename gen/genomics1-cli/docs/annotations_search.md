Searches for annotations that match the given criteria. Results are
ordered by genomic coordinate (by reference sequence, then position).
Annotations with equivalent genomic coordinates are returned in an
unspecified order. This order is consistent, such that two queries for the
same content (regardless of page size) yield annotations in the same order
across their respective streams of paginated responses. Caller must have
READ permission for the queried annotation sets.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/genomics*
* *https://www.googleapis.com/auth/genomics.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `genomics1 --scope <scope> annotations search ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
SearchAnnotationsRequest:
  annotation-set-ids: [string]
  end: string
  page-size: integer
  page-token: string
  reference-id: string
  reference-name: string
  start: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    annotation-set-ids=ea`
    - Required. The annotation sets to search within. The caller must have
        `READ` access to these annotation sets.
        All queried annotation sets must have the same type.
    - Each invocation of this argument appends the given value to the array.
* `end=no`
    - The end position of the range on the reference, 0-based exclusive. If
        referenceId or
        referenceName
        must be specified, Defaults to the length of the reference.
* `page-size=80`
    - The maximum number of results to return in a single page. If unspecified,
        defaults to 256. The maximum value is 2048.
* `page-token=justo`
    - The continuation token, which is used to page through large result sets.
        To get the next page of results, set this parameter to the value of
        `nextPageToken` from the previous response.
* `reference-id=et`
    - The ID of the reference to query.
* `reference-name=et`
    - The name of the reference to query, within the reference set associated
        with this query.
* `start=diam`
    - The start position of the range on the reference, 0-based inclusive. If
        specified,
        referenceId or
        referenceName
        must be specified. Defaults to 0.


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
