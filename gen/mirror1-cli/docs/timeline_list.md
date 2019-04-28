Retrieves a list of timeline items for the authenticated user.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/glass.location*
* *https://www.googleapis.com/auth/glass.timeline*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/glass.location*.
You can set the scope for this method like this: `mirror1 --scope <scope> timeline list ...`

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

* **-p include-deleted=boolean**
    - If true, tombstone records for deleted items will be returned.

* **-p pinned-only=boolean**
    - If true, only pinned items will be returned.

* **-p source-item-id=string**
    - If provided, only items with the given sourceItemId will be returned.

* **-p page-token=string**
    - Token for the page of results to return.

* **-p order-by=string**
    - Controls the order in which timeline items are returned.

* **-p bundle-id=string**
    - If provided, only items with the given bundleId will be returned.

* **-p max-results=integer**
    - The maximum number of items to include in the response, used for paging.

# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p alt=string**
    - Data format for the response.

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.

* **-p user-ip=string**
    - Deprecated. Please use quotaUser instead.