Generate an AdSense report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify &#34;alt=csv&#34; as a query parameter.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/adsense*
* *https://www.googleapis.com/auth/adsense.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adsense.readonly*.
You can set the scope for this method like this: `adsense1d4 --scope <scope> reports generate ...`
# Required Scalar Arguments
* **&lt;start-date&gt;** *(string)*
    - Start of the date range to report on in &#34;YYYY-MM-DD&#34; format, inclusive.
* **&lt;end-date&gt;** *(string)*
    - End of the date range to report on in &#34;YYYY-MM-DD&#34; format, inclusive.

# Optional Output Flags

The method's return value a JSON encoded structure, which will be written to standard output by default.

As this method supports **media download**, you may specify the `-p alt=media` flag to set the output to be an octet stream of the underlying media. In that case, you will not receive JSON output anymore.

* **-o out**
    - *out* specifies the *destination* to which to write the server's result to.
      It will either be a JSON-encoded structure, or the media file you are downloading.
      The *destination* may be `-` to indicate standard output, or a filepath that is to contain the received bytes.
      If unset, it defaults to standard output.
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p max-results=integer**
    - The maximum number of rows of report data to return.

* **-p filter=string**
    - Filters to be run on the report.

* **-p start-index=integer**
    - Index of the first row of report data to return.

* **-p currency=string**
    - Optional currency to use when reporting on monetary metrics. Defaults to the account&#39;s currency if not set.

* **-p use-timezone-reporting=boolean**
    - Whether the report should be generated in the AdSense account&#39;s local timezone. If false default PST/PDT timezone will be used.

* **-p dimension=string**
    - Dimensions to base the report on.

* **-p account-id=string**
    - Accounts upon which to report.

* **-p locale=string**
    - Optional locale to use for translating report output to a local language. Defaults to &#34;en_US&#34; if not specified.

* **-p sort=string**
    - The name of a dimension or metric to sort the resulting report on, optionally prefixed with &#34;+&#34; to sort ascending or &#34;-&#34; to sort descending. If no prefix is specified, the column is sorted ascending.

* **-p metric=string**
    - Numeric columns to include in the report.

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