Returns a list of comments that match the API request parameters.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/youtube.force-ssl* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube.force-ssl*.
You can set the scope for this method like this: `youtube3 --scope <scope> comments list ...`
# Required Scalar Argument
* **&lt;part&gt;** *(string)*
    - The part parameter specifies a comma-separated list of one or more comment resource properties that the API response will include.

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

* **-p text-format=string**
    - This parameter indicates whether the API should return comments formatted as HTML or as plain text.

* **-p id=string**
    - The id parameter specifies a comma-separated list of comment IDs for the resources that are being retrieved. In a comment resource, the id property specifies the comment&#39;s ID.

* **-p parent-id=string**
    - The parentId parameter specifies the ID of the comment for which replies should be retrieved.
        
        Note: YouTube currently supports replies only for top-level comments. However, replies to replies may be supported in the future.

* **-p page-token=string**
    - The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken property identifies the next page of the result that can be retrieved.
        
        Note: This parameter is not supported for use in conjunction with the id parameter.

* **-p max-results=integer**
    - The maxResults parameter specifies the maximum number of items that should be returned in the result set.
        
        Note: This parameter is not supported for use in conjunction with the id parameter.

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