Retrieves a list of objects matching the criteria.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloud-platform.read-only*
* *https://www.googleapis.com/auth/devstorage.full_control*
* *https://www.googleapis.com/auth/devstorage.read_only*
* *https://www.googleapis.com/auth/devstorage.read_write*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `storage1 --scope <scope> objects list ...`
# Required Scalar Argument
* **&lt;bucket&gt;** *(string)*
    - Name of the bucket in which to look for objects.

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

* **-p user-project=string**
    - The project to be billed for this request. Required for Requester Pays buckets.

* **-p projection=string**
    - Set of properties to return. Defaults to noAcl.

* **-p versions=boolean**
    - If true, lists all versions of an object as distinct results. The default is false. For more information, see Object Versioning.

* **-p include-trailing-delimiter=boolean**
    - If true, objects that end in exactly one instance of delimiter will have their metadata included in items in addition to prefixes.

* **-p max-results=integer**
    - Maximum number of items plus prefixes to return in a single page of responses. As duplicate prefixes are omitted, fewer total results may be returned than requested. The service will use this parameter or 1,000 items, whichever is smaller.

* **-p prefix=string**
    - Filter results to objects whose names begin with this prefix.

* **-p page-token=string**
    - A previously-returned page token representing part of the larger set of results to view.

* **-p delimiter=string**
    - Returns results in a directory-like mode. items will contain only objects whose names, aside from the prefix, do not contain delimiter. Objects whose names, aside from the prefix, contain delimiter will have their name, truncated after the delimiter, returned in prefixes. Duplicate prefixes are omitted.

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
