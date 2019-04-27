Sets values in a range of a spreadsheet.
The caller must specify the spreadsheet ID, range, and
a valueInputOption.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.file*
* *https://www.googleapis.com/auth/spreadsheets*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive*.
You can set the scope for this method like this: `sheets4 --scope <scope> spreadsheets values-update ...`
# Required Scalar Arguments
* **&lt;spreadsheet-id&gt;** *(string)*
    - The ID of the spreadsheet to update.
* **&lt;range&gt;** *(string)*
    - The A1 notation of the values to update.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ValueRange:
  major-dimension: string
  range: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    major-dimension=sadipscing`
    - The major dimension of the values.
        
        For output, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`,
        then requesting `range=A1:B2,majorDimension=ROWS` will return
        `[[1,2],[3,4]]`,
        whereas requesting `range=A1:B2,majorDimension=COLUMNS` will return
        `[[1,3],[2,4]]`.
        
        For input, with `range=A1:B2,majorDimension=ROWS` then `[[1,2],[3,4]]`
        will set `A1=1,B1=2,A2=3,B2=4`. With `range=A1:B2,majorDimension=COLUMNS`
        then `[[1,2],[3,4]]` will set `A1=1,B1=3,A2=2,B2=4`.
        
        When writing, if this field is not set, it defaults to ROWS.
* `range=invidunt`
    - The range the values cover, in A1 notation.
        For output, this range indicates the entire requested range,
        even though the values will exclude trailing rows and columns.
        When appending values, this field represents the range to search for a
        table, after which values will be appended.


### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.


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

* **-p include-values-in-response=boolean**
    - Determines if the update response should include the values
        of the cells that were updated. By default, responses
        do not include the updated values.
        If the range to write was larger than than the range actually written,
        the response will include all values in the requested range (excluding
        trailing empty rows and columns).

* **-p response-date-time-render-option=string**
    - Determines how dates, times, and durations in the response should be
        rendered. This is ignored if response_value_render_option is
        FORMATTED_VALUE.
        The default dateTime render option is
        DateTimeRenderOption.SERIAL_NUMBER.

* **-p value-input-option=string**
    - How the input data should be interpreted.

* **-p response-value-render-option=string**
    - Determines how values in the response should be rendered.
        The default render option is ValueRenderOption.FORMATTED_VALUE.

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
