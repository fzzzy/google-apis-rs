Returns a collection of search results that match the query parameters specified in the API request. By default, a search result set identifies matching video, channel, and playlist resources, but you can also configure queries to only retrieve a specific type of resource.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.force-ssl*
* *https://www.googleapis.com/auth/youtube.readonly*
* *https://www.googleapis.com/auth/youtubepartner*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube.readonly*.
You can set the scope for this method like this: `youtube3 --scope <scope> search list ...`
# Required Scalar Argument
* **&lt;part&gt;** *(string)*
    - The part parameter specifies a comma-separated list of one or more search resource properties that the API response will include. Set the parameter value to snippet.

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

* **-p location-radius=string**
    - The locationRadius parameter, in conjunction with the location parameter, defines a circular geographic area.
        
        The parameter value must be a floating point number followed by a measurement unit. Valid measurement units are m, km, ft, and mi. For example, valid parameter values include 1500m, 5km, 10000ft, and 0.75mi. The API does not support locationRadius parameter values larger than 1000 kilometers.
        
        Note: See the definition of the location parameter for more information.

* **-p order=string**
    - The order parameter specifies the method that will be used to order resources in the API response.

* **-p event-type=string**
    - The eventType parameter restricts a search to broadcast events. If you specify a value for this parameter, you must also set the type parameter&#39;s value to video.

* **-p video-definition=string**
    - The videoDefinition parameter lets you restrict a search to only include either high definition (HD) or standard definition (SD) videos. HD videos are available for playback in at least 720p, though higher resolutions, like 1080p, might also be available. If you specify a value for this parameter, you must also set the type parameter&#39;s value to video.

* **-p location=string**
    - The location parameter, in conjunction with the locationRadius parameter, defines a circular geographic area and also restricts a search to videos that specify, in their metadata, a geographic location that falls within that area. The parameter value is a string that specifies latitude/longitude coordinates e.g. (37.42307,-122.08427).
        
        
        - The location parameter value identifies the point at the center of the area.
        - The locationRadius parameter specifies the maximum distance that the location associated with a video can be from that point for the video to still be included in the search results.The API returns an error if your request specifies a value for the location parameter but does not also specify a value for the locationRadius parameter.

* **-p published-before=string**
    - The publishedBefore parameter indicates that the API response should only contain resources created before the specified time. The value is an RFC 3339 formatted date-time value (1970-01-01T00:00:00Z).

* **-p video-caption=string**
    - The videoCaption parameter indicates whether the API should filter video search results based on whether they have captions. If you specify a value for this parameter, you must also set the type parameter&#39;s value to video.

* **-p published-after=string**
    - The publishedAfter parameter indicates that the API response should only contain resources created after the specified time. The value is an RFC 3339 formatted date-time value (1970-01-01T00:00:00Z).

* **-p video-duration=string**
    - The videoDuration parameter filters video search results based on their duration. If you specify a value for this parameter, you must also set the type parameter&#39;s value to video.

* **-p channel-id=string**
    - The channelId parameter indicates that the API response should only contain resources created by the channel

* **-p for-developer=boolean**
    - The forDeveloper parameter restricts the search to only retrieve videos uploaded via the developer&#39;s application or website. The API server uses the request&#39;s authorization credentials to identify the developer. Therefore, a developer can restrict results to videos uploaded through the developer&#39;s own app or website but not to videos uploaded through other apps or sites.

* **-p relevance-language=string**
    - The relevanceLanguage parameter instructs the API to return search results that are most relevant to the specified language. The parameter value is typically an ISO 639-1 two-letter language code. However, you should use the values zh-Hans for simplified Chinese and zh-Hant for traditional Chinese. Please note that results in other languages will still be returned if they are highly relevant to the search query term.

* **-p video-type=string**
    - The videoType parameter lets you restrict a search to a particular type of videos. If you specify a value for this parameter, you must also set the type parameter&#39;s value to video.

* **-p video-syndicated=string**
    - The videoSyndicated parameter lets you to restrict a search to only videos that can be played outside youtube.com. If you specify a value for this parameter, you must also set the type parameter&#39;s value to video.

* **-p on-behalf-of-content-owner=string**
    - Note: This parameter is intended exclusively for YouTube content partners.
        
        The onBehalfOfContentOwner parameter indicates that the request&#39;s authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

* **-p region-code=string**
    - The regionCode parameter instructs the API to return search results for the specified country. The parameter value is an ISO 3166-1 alpha-2 country code.

* **-p for-mine=boolean**
    - The forMine parameter restricts the search to only retrieve videos owned by the authenticated user. If you set this parameter to true, then the type parameter&#39;s value must also be set to video.

* **-p q=string**
    - The q parameter specifies the query term to search for.
        
        Your request can also use the Boolean NOT (-) and OR (|) operators to exclude videos or to find videos that are associated with one of several search terms. For example, to search for videos matching either &#34;boating&#34; or &#34;sailing&#34;, set the q parameter value to boating|sailing. Similarly, to search for videos matching either &#34;boating&#34; or &#34;sailing&#34; but not &#34;fishing&#34;, set the q parameter value to boating|sailing -fishing. Note that the pipe character must be URL-escaped when it is sent in your API request. The URL-escaped value for the pipe character is %7C.

* **-p video-category-id=string**
    - The videoCategoryId parameter filters video search results based on their category. If you specify a value for this parameter, you must also set the type parameter&#39;s value to video.

* **-p for-content-owner=boolean**
    - Note: This parameter is intended exclusively for YouTube content partners.
        
        The forContentOwner parameter restricts the search to only retrieve resources owned by the content owner specified by the onBehalfOfContentOwner parameter. The user must be authenticated using a CMS account linked to the specified content owner and onBehalfOfContentOwner must be provided.

* **-p topic-id=string**
    - The topicId parameter indicates that the API response should only contain resources associated with the specified topic. The value identifies a Freebase topic ID.

* **-p video-license=string**
    - The videoLicense parameter filters search results to only include videos with a particular license. YouTube lets video uploaders choose to attach either the Creative Commons license or the standard YouTube license to each of their videos. If you specify a value for this parameter, you must also set the type parameter&#39;s value to video.

* **-p type=string**
    - The type parameter restricts a search query to only retrieve a particular type of resource. The value is a comma-separated list of resource types.

* **-p page-token=string**
    - The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.

* **-p video-embeddable=string**
    - The videoEmbeddable parameter lets you to restrict a search to only videos that can be embedded into a webpage. If you specify a value for this parameter, you must also set the type parameter&#39;s value to video.

* **-p channel-type=string**
    - The channelType parameter lets you restrict a search to a particular type of channel.

* **-p safe-search=string**
    - The safeSearch parameter indicates whether the search results should include restricted content as well as standard content.

* **-p max-results=integer**
    - The maxResults parameter specifies the maximum number of items that should be returned in the result set.

* **-p video-dimension=string**
    - The videoDimension parameter lets you restrict a search to only retrieve 2D or 3D videos. If you specify a value for this parameter, you must also set the type parameter&#39;s value to video.

* **-p related-to-video-id=string**
    - The relatedToVideoId parameter retrieves a list of videos that are related to the video that the parameter value identifies. The parameter value must be set to a YouTube video ID and, if you are using this parameter, the type parameter must be set to video.

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
