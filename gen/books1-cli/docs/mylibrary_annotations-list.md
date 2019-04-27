Retrieves a list of annotations, possibly filtered.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/books* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/books*.
You can set the scope for this method like this: `books1 --scope <scope> mylibrary annotations-list ...`

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

* **-p source=string**
    - String to identify the originator of this request.

* **-p updated-min=string**
    - RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive).

* **-p volume-id=string**
    - The volume to restrict annotations to.

* **-p layer-ids=string**
    - The layer ID(s) to limit annotation by.

* **-p show-deleted=boolean**
    - Set to true to return deleted annotations. updatedMin must be in the request to use this. Defaults to false.

* **-p updated-max=string**
    - RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive).

* **-p page-token=string**
    - The value of the nextToken from the previous page.

* **-p max-results=integer**
    - Maximum number of results to return

* **-p content-version=string**
    - The content version for the requested volume.

* **-p layer-id=string**
    - The layer ID to limit annotation by.

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
