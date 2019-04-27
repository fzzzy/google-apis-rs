Returns a list of comment threads that match the API request parameters.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/youtube.force-ssl* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube.force-ssl*.
You can set the scope for this method like this: `youtube3 --scope <scope> comment-threads list ...`
# Required Scalar Argument
* **&lt;part&gt;** *(string)*
    - The part parameter specifies a comma-separated list of one or more commentThread resource properties that the API response will include.

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

* **-p all-threads-related-to-channel-id=string**
    - The allThreadsRelatedToChannelId parameter instructs the API to return all comment threads associated with the specified channel. The response can include comments about the channel or about the channel&#39;s videos.

* **-p text-format=string**
    - Set this parameter&#39;s value to html or plainText to instruct the API to return the comments left by users in html formatted or in plain text.

* **-p moderation-status=string**
    - Set this parameter to limit the returned comment threads to a particular moderation state.
        
        Note: This parameter is not supported for use in conjunction with the id parameter.

* **-p channel-id=string**
    - The channelId parameter instructs the API to return comment threads containing comments about the specified channel. (The response will not include comments left on videos that the channel uploaded.)

* **-p order=string**
    - The order parameter specifies the order in which the API response should list comment threads. Valid values are: 
        - time - Comment threads are ordered by time. This is the default behavior.
        - relevance - Comment threads are ordered by relevance.Note: This parameter is not supported for use in conjunction with the id parameter.

* **-p id=string**
    - The id parameter specifies a comma-separated list of comment thread IDs for the resources that should be retrieved.

* **-p search-terms=string**
    - The searchTerms parameter instructs the API to limit the API response to only contain comments that contain the specified search terms.
        
        Note: This parameter is not supported for use in conjunction with the id parameter.

* **-p max-results=integer**
    - The maxResults parameter specifies the maximum number of items that should be returned in the result set.
        
        Note: This parameter is not supported for use in conjunction with the id parameter.

* **-p page-token=string**
    - The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken property identifies the next page of the result that can be retrieved.
        
        Note: This parameter is not supported for use in conjunction with the id parameter.

* **-p video-id=string**
    - The videoId parameter instructs the API to return comment threads associated with the specified video ID.

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
