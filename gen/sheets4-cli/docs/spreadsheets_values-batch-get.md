Returns one or more ranges of values from a spreadsheet.
The caller must specify the spreadsheet ID and one or more ranges.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.file*
* *https://www.googleapis.com/auth/drive.readonly*
* *https://www.googleapis.com/auth/spreadsheets*
* *https://www.googleapis.com/auth/spreadsheets.readonly*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive.readonly*.
You can set the scope for this method like this: `sheets4 --scope <scope> spreadsheets values-batch-get ...`
# Required Scalar Argument
* **&lt;spreadsheet-id&gt;** *(string)*
    - The ID of the spreadsheet to retrieve data from.

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

* **-p value-render-option=string**
    - How values should be represented in the output.
        The default render option is ValueRenderOption.FORMATTED_VALUE.

* **-p date-time-render-option=string**
    - How dates, times, and durations should be represented in the output.
        This is ignored if value_render_option is
        FORMATTED_VALUE.
        The default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER].

* **-p major-dimension=string**
    - The major dimension that results should use.
        
        For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`,
        then requesting `range=A1:B2,majorDimension=ROWS` will return
        `[[1,2],[3,4]]`,
        whereas requesting `range=A1:B2,majorDimension=COLUMNS` will return
        `[[1,3],[2,4]]`.

* **-p ranges=string**
    - The A1 notation of the values to retrieve.

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
