Returns a list of channel activity events that match the request criteria. For example, you can retrieve events associated with a particular channel, events associated with the user&#39;s subscriptions and Google+ friends, or the YouTube home page feed, which is customized for each user.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.force-ssl*
* *https://www.googleapis.com/auth/youtube.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube.readonly*.
You can set the scope for this method like this: `youtube3 --scope <scope> activities list ...`
# Required Scalar Argument
* **&lt;part&gt;** *(string)*
    - The part parameter specifies a comma-separated list of one or more activity resource properties that the API response will include.
        
        If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in an activity resource, the snippet property contains other properties that identify the type of activity, a display title for the activity, and so forth. If you set part=snippet, the API response will also contain all of those nested properties.

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

* **-p page-token=string**
    - The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.

* **-p channel-id=string**
    - The channelId parameter specifies a unique YouTube channel ID. The API will then return a list of that channel&#39;s activities.

* **-p published-before=string**
    - The publishedBefore parameter specifies the date and time before which an activity must have occurred for that activity to be included in the API response. If the parameter value specifies a day, but not a time, then any activities that occurred that day will be excluded from the result set. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.

* **-p max-results=integer**
    - The maxResults parameter specifies the maximum number of items that should be returned in the result set.

* **-p mine=boolean**
    - Set this parameter&#39;s value to true to retrieve a feed of the authenticated user&#39;s activities.

* **-p region-code=string**
    - The regionCode parameter instructs the API to return results for the specified country. The parameter value is an ISO 3166-1 alpha-2 country code. YouTube uses this value when the authorized user&#39;s previous activity on YouTube does not provide enough information to generate the activity feed.

* **-p home=boolean**
    - Set this parameter&#39;s value to true to retrieve the activity feed that displays on the YouTube home page for the currently authenticated user.

* **-p published-after=string**
    - The publishedAfter parameter specifies the earliest date and time that an activity could have occurred for that activity to be included in the API response. If the parameter value specifies a day, but not a time, then any activities that occurred that day will be included in the result set. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.

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
