Retrieve your YouTube Analytics reports.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.readonly*
* *https://www.googleapis.com/auth/yt-analytics-monetary.readonly*
* *https://www.googleapis.com/auth/yt-analytics.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube.readonly*.
You can set the scope for this method like this: `youtubeanalytics1 --scope <scope> reports query ...`
# Required Scalar Arguments
* **&lt;ids&gt;** *(string)*
    - Identifies the YouTube channel or content owner for which you are retrieving YouTube Analytics data.
        - To request data for a YouTube user, set the ids parameter value to channel==CHANNEL_ID, where CHANNEL_ID specifies the unique YouTube channel ID.
        - To request data for a YouTube CMS content owner, set the ids parameter value to contentOwner==OWNER_NAME, where OWNER_NAME is the CMS name of the content owner.
* **&lt;start-date&gt;** *(string)*
    - The start date for fetching YouTube Analytics data. The value should be in YYYY-MM-DD format.
* **&lt;end-date&gt;** *(string)*
    - The end date for fetching YouTube Analytics data. The value should be in YYYY-MM-DD format.
* **&lt;metrics&gt;** *(string)*
    - A comma-separated list of YouTube Analytics metrics, such as views or likes,dislikes. See the Available Reports document for a list of the reports that you can retrieve and the metrics available in each report, and see the Metrics document for definitions of those metrics.

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

* **-p dimensions=string**
    - A comma-separated list of YouTube Analytics dimensions, such as views or ageGroup,gender. See the Available Reports document for a list of the reports that you can retrieve and the dimensions used for those reports. Also see the Dimensions document for definitions of those dimensions.

* **-p max-results=integer**
    - The maximum number of rows to include in the response.

* **-p sort=string**
    - A comma-separated list of dimensions or metrics that determine the sort order for YouTube Analytics data. By default the sort order is ascending. The &#39;-&#39; prefix causes descending sort order.

* **-p filters=string**
    - A list of filters that should be applied when retrieving YouTube Analytics data. The Available Reports document identifies the dimensions that can be used to filter each report, and the Dimensions document defines those dimensions. If a request uses multiple filters, join them together with a semicolon (;), and the returned result table will satisfy both filters. For example, a filters parameter value of video==dMH0bHeiRNg;country==IT restricts the result set to include data for the given video in Italy.

* **-p include-historical-channel-data=boolean**
    - If set to true historical data (i.e. channel data from before the linking of the channel to the content owner) will be retrieved.

* **-p start-index=integer**
    - An index of the first entity to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter (one-based, inclusive).

* **-p currency=string**
    - The currency to which financial metrics should be converted. The default is US Dollar (USD). If the result contains no financial metrics, this flag will be ignored. Responds with an error if the specified currency is not recognized.

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
