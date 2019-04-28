List all of the comments for an activity.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/plus.login*
* *https://www.googleapis.com/auth/plus.me*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/plus.login*.
You can set the scope for this method like this: `plus1 --scope <scope> comments list ...`
# Required Scalar Argument
* **&lt;activity-id&gt;** *(string)*
    - The ID of the activity to get comments for.

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

* **-p max-results=integer**
    - The maximum number of comments to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.

* **-p sort-order=string**
    - The order in which to sort the list of comments.

* **-p page-token=string**
    - The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of &#34;nextPageToken&#34; from the previous response.

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