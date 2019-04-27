Sets values in one or more ranges of a spreadsheet.
The caller must specify the spreadsheet ID,
a valueInputOption, and one or more
DataFilterValueRanges.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.file*
* *https://www.googleapis.com/auth/spreadsheets*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive*.
You can set the scope for this method like this: `sheets4 --scope <scope> spreadsheets values-batch-update-by-data-filter ...`
# Required Scalar Argument
* **&lt;spreadsheet-id&gt;** *(string)*
    - The ID of the spreadsheet to update.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
BatchUpdateValuesByDataFilterRequest:
  include-values-in-response: boolean
  response-date-time-render-option: string
  response-value-render-option: string
  value-input-option: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    include-values-in-response=false`
    - Determines if the update response should include the values
        of the cells that were updated. By default, responses
        do not include the updated values. The `updatedData` field within
        each of the BatchUpdateValuesResponse.responses will contain
        the updated values. If the range to write was larger than than the range
        actually written, the response will include all values in the requested
        range (excluding trailing empty rows and columns).
* `response-date-time-render-option=consetetur`
    - Determines how dates, times, and durations in the response should be
        rendered. This is ignored if response_value_render_option is
        FORMATTED_VALUE.
        The default dateTime render option is
        DateTimeRenderOption.SERIAL_NUMBER.
* `response-value-render-option=sadipscing`
    - Determines how values in the response should be rendered.
        The default render option is ValueRenderOption.FORMATTED_VALUE.
* `value-input-option=vero`
    - How the input data should be interpreted.


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
