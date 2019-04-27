Returns a list of videos that match the API request parameters.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.force-ssl*
* *https://www.googleapis.com/auth/youtube.readonly*
* *https://www.googleapis.com/auth/youtubepartner*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube.readonly*.
You can set the scope for this method like this: `youtube3 --scope <scope> videos list ...`
# Required Scalar Argument
* **&lt;part&gt;** *(string)*
    - The part parameter specifies a comma-separated list of one or more video resource properties that the API response will include.
        
        If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a video resource, the snippet property contains the channelId, title, description, tags, and categoryId properties. As such, if you set part=snippet, the API response will contain all of those properties.

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

* **-p locale=string**
    - DEPRECATED

* **-p on-behalf-of-content-owner=string**
    - Note: This parameter is intended exclusively for YouTube content partners.
        
        The onBehalfOfContentOwner parameter indicates that the request&#39;s authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

* **-p hl=string**
    - The hl parameter instructs the API to retrieve localized resource metadata for a specific application language that the YouTube website supports. The parameter value must be a language code included in the list returned by the i18nLanguages.list method.
        
        If localized resource details are available in that language, the resource&#39;s snippet.localized object will contain the localized values. However, if localized details are not available, the snippet.localized object will contain resource details in the resource&#39;s default language.

* **-p max-results=integer**
    - The maxResults parameter specifies the maximum number of items that should be returned in the result set.
        
        Note: This parameter is supported for use in conjunction with the myRating and chart parameters, but it is not supported for use in conjunction with the id parameter.

* **-p chart=string**
    - The chart parameter identifies the chart that you want to retrieve.

* **-p video-category-id=string**
    - The videoCategoryId parameter identifies the video category for which the chart should be retrieved. This parameter can only be used in conjunction with the chart parameter. By default, charts are not restricted to a particular category.

* **-p region-code=string**
    - The regionCode parameter instructs the API to select a video chart available in the specified region. This parameter can only be used in conjunction with the chart parameter. The parameter value is an ISO 3166-1 alpha-2 country code.

* **-p max-height=integer**
    - The maxHeight parameter specifies a maximum height of the embedded player. If maxWidth is provided, maxHeight may not be reached in order to not violate the width request.

* **-p id=string**
    - The id parameter specifies a comma-separated list of the YouTube video ID(s) for the resource(s) that are being retrieved. In a video resource, the id property specifies the video&#39;s ID.

* **-p my-rating=string**
    - Set this parameter&#39;s value to like or dislike to instruct the API to only return videos liked or disliked by the authenticated user.

* **-p max-width=integer**
    - The maxWidth parameter specifies a maximum width of the embedded player. If maxHeight is provided, maxWidth may not be reached in order to not violate the height request.

* **-p page-token=string**
    - The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.
        
        Note: This parameter is supported for use in conjunction with the myRating and chart parameters, but it is not supported for use in conjunction with the id parameter.

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
