Lists fixed width coverage buckets for a read group set, each of which
correspond to a range of a reference sequence. Each bucket summarizes
coverage information across its corresponding genomic range.

Coverage is defined as the number of reads which are aligned to a given
base in the reference sequence. Coverage buckets are available at several
precomputed bucket widths, enabling retrieval of various coverage &#39;zoom
levels&#39;. The caller must have READ permissions for the target read group
set.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/genomics*
* *https://www.googleapis.com/auth/genomics.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/genomics.readonly*.
You can set the scope for this method like this: `genomics1 --scope <scope> readgroupsets coveragebuckets-list ...`
# Required Scalar Argument
* **&lt;read-group-set-id&gt;** *(string)*
    - Required. The ID of the read group set over which coverage is requested.

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
    - The maximum number of results to return in a single page. If unspecified,
        defaults to 1024. The maximum value is 2048.

* **-p target-bucket-width=string**
    - The desired width of each reported coverage bucket in base pairs. This
        will be rounded down to the nearest precomputed bucket width; the value
        of which is returned as `bucketWidth` in the response. Defaults
        to infinity (each bucket spans an entire reference sequence) or the length
        of the target range, if specified. The smallest precomputed
        `bucketWidth` is currently 2048 base pairs; this is subject to
        change.

* **-p reference-name=string**
    - The name of the reference to query, within the reference set associated
        with this query. Optional.

* **-p page-token=string**
    - The continuation token, which is used to page through large result sets.
        To get the next page of results, set this parameter to the value of
        `nextPageToken` from the previous response.

* **-p end=string**
    - The end position of the range on the reference, 0-based exclusive. If
        specified, `referenceName` must also be specified. If unset or 0, defaults
        to the length of the reference.

* **-p start=string**
    - The start position of the range on the reference, 0-based inclusive. If
        specified, `referenceName` must also be specified. Defaults to 0.

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
