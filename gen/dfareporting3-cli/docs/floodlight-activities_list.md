Retrieves a list of floodlight activities, possibly filtered. This method supports paging.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3 --scope <scope> floodlight-activities list ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.

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

* **-p floodlight-activity-group-type=string**
    - Select only floodlight activities with the specified floodlight activity group type.

* **-p page-token=string**
    - Value of the nextPageToken from the previous result page.

* **-p tag-string=string**
    - Select only floodlight activities with the specified tag string.

* **-p ids=string**
    - Select only floodlight activities with the specified IDs. Must specify either ids, advertiserId, or floodlightConfigurationId for a non-empty result.

* **-p floodlight-activity-group-name=string**
    - Select only floodlight activities with the specified floodlight activity group name.

* **-p sort-field=string**
    - Field by which to sort the list.

* **-p floodlight-activity-group-ids=string**
    - Select only floodlight activities with the specified floodlight activity group IDs.

* **-p search-string=string**
    - Allows searching for objects by name or ID. Wildcards (*) are allowed. For example, &#34;floodlightactivity*2015&#34; will return objects with names like &#34;floodlightactivity June 2015&#34;, &#34;floodlightactivity April 2015&#34;, or simply &#34;floodlightactivity 2015&#34;. Most of the searches also add wildcards implicitly at the start and the end of the search string. For example, a search string of &#34;floodlightactivity&#34; will match objects with name &#34;my floodlightactivity activity&#34;, &#34;floodlightactivity 2015&#34;, or simply &#34;floodlightactivity&#34;.

* **-p sort-order=string**
    - Order of sorted results.

* **-p max-results=integer**
    - Maximum number of results to return.

* **-p floodlight-configuration-id=string**
    - Select only floodlight activities for the specified floodlight configuration ID. Must specify either ids, advertiserId, or floodlightConfigurationId for a non-empty result.

* **-p advertiser-id=string**
    - Select only floodlight activities for the specified advertiser ID. Must specify either ids, advertiserId, or floodlightConfigurationId for a non-empty result.

* **-p floodlight-activity-group-tag-string=string**
    - Select only floodlight activities with the specified floodlight activity group tag string.

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
