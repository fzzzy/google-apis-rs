Returns a collection of groups that match the API request parameters. For
example, you can retrieve all groups that the authenticated user owns,
or you can retrieve one or more groups by their unique IDs.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.readonly*
* *https://www.googleapis.com/auth/youtubepartner*
* *https://www.googleapis.com/auth/yt-analytics-monetary.readonly*
* *https://www.googleapis.com/auth/yt-analytics.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube.readonly*.
You can set the scope for this method like this: `youtubeanalytics2 --scope <scope> groups list ...`

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

* **-p id=string**
    - The `id` parameter specifies a comma-separated list of the YouTube group
        ID(s) for the resource(s) that are being retrieved. Each group must be
        owned by the authenticated user. In a `group` resource, the `id` property
        specifies the group&#39;s YouTube group ID.
        
        Note that if you do not specify a value for the `id` parameter, then you
        must set the `mine` parameter to `true`.

* **-p mine=boolean**
    - This parameter can only be used in a properly authorized request. Set this
        parameter&#39;s value to true to retrieve all groups owned by the authenticated
        user.

* **-p on-behalf-of-content-owner=string**
    - This parameter can only be used in a properly authorized request. **Note:**
        This parameter is intended exclusively for YouTube content partners that
        own and manage many different YouTube channels.
        
        The `onBehalfOfContentOwner` parameter indicates that the request&#39;s
        authorization credentials identify a YouTube user who is acting on behalf
        of the content owner specified in the parameter value. It allows content
        owners to authenticate once and get access to all their video and channel
        data, without having to provide authentication credentials for each
        individual channel. The account that the user authenticates with must be
        linked to the specified YouTube content owner.

* **-p page-token=string**
    - The `pageToken` parameter identifies a specific page in the result set that
        should be returned. In an API response, the `nextPageToken` property
        identifies the next page that can be retrieved.

# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p $-xgafv=string**
    - V1 error format.

* **-p access-token=string**
    - OAuth access token.

* **-p alt=string**
    - Data format for response.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).
