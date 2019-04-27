Gets a list of reads for one or more read group sets.

Reads search operates over a genomic coordinate space of reference sequence
&amp; position defined over the reference sequences to which the requested
read group sets are aligned.

If a target positional range is specified, search returns all reads whose
alignment to the reference genome overlap the range. A query which
specifies only read group set IDs yields all reads in those read group
sets, including unmapped reads.

All reads returned (including reads on subsequent pages) are ordered by
genomic coordinate (by reference sequence, then position). Reads with
equivalent genomic coordinates are returned in an unspecified order. This
order is consistent, such that two queries for the same content (regardless
of page size) yield reads in the same order across their respective streams
of paginated responses.

Implements
[GlobalAllianceApi.searchReads](https://github.com/ga4gh/schemas/blob/v0.5.1/src/main/resources/avro/readmethods.avdl#L85).
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/genomics*
* *https://www.googleapis.com/auth/genomics.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `genomics1 --scope <scope> reads search ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
SearchReadsRequest:
  end: string
  page-size: integer
  page-token: string
  read-group-ids: [string]
  read-group-set-ids: [string]
  reference-name: string
  start: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    end=sit`
    - The end position of the range on the reference, 0-based exclusive. If
        specified, `referenceName` must also be specified.
* `page-size=61`
    - The maximum number of results to return in a single page. If unspecified,
        defaults to 256. The maximum value is 2048.
* `page-token=consetetur`
    - The continuation token, which is used to page through large result sets.
        To get the next page of results, set this parameter to the value of
        `nextPageToken` from the previous response.
* `read-group-ids=labore`
    - The IDs of the read groups within which to search for reads. All specified
        read groups must belong to the same read group sets. Must specify one of
        `readGroupSetIds` or `readGroupIds`.
    - Each invocation of this argument appends the given value to the array.
* `read-group-set-ids=sed`
    - The IDs of the read groups sets within which to search for reads. All
        specified read group sets must be aligned against a common set of reference
        sequences; this defines the genomic coordinates for the query. Must specify
        one of `readGroupSetIds` or `readGroupIds`.
    - Each invocation of this argument appends the given value to the array.
* `reference-name=ea`
    - The reference sequence name, for example `chr1`, `1`, or `chrX`. If set to
        `*`, only unmapped reads are returned. If unspecified, all reads (mapped
        and unmapped) are returned.
* `start=gubergren`
    - The start position of the range on the reference, 0-based inclusive. If
        specified, `referenceName` must also be specified.


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
