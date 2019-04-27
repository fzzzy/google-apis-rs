Retrieves a list of posts, possibly filtered.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/blogger*
* *https://www.googleapis.com/auth/blogger.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/blogger.readonly*.
You can set the scope for this method like this: `blogger3 --scope <scope> posts list ...`
# Required Scalar Argument
* **&lt;blog-id&gt;** *(string)*
    - ID of the blog to fetch posts from.

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

* **-p fetch-images=boolean**
    - Whether image URL metadata for each post is included.

* **-p page-token=string**
    - Continuation token if the request is paged.

* **-p view=string**
    - Access level with which to view the returned result. Note that some fields require escalated access.

* **-p order-by=string**
    - Sort search results

* **-p end-date=string**
    - Latest post date to fetch, a date-time with RFC 3339 formatting.

* **-p status=string**
    - Statuses to include in the results.

* **-p fetch-bodies=boolean**
    - Whether the body content of posts is included (default: true). This should be set to false when the post bodies are not required, to help minimize traffic.

* **-p labels=string**
    - Comma-separated list of labels to search for.

* **-p start-date=string**
    - Earliest post date to fetch, a date-time with RFC 3339 formatting.

* **-p max-results=integer**
    - Maximum number of posts to fetch.

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
