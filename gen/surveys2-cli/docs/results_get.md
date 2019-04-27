Retrieves any survey results that have been produced so far. Results are formatted as an Excel file. You must add &#34;?alt=media&#34; to the URL as an argument to get results.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/surveys*
* *https://www.googleapis.com/auth/surveys.readonly*
* *https://www.googleapis.com/auth/userinfo.email*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/surveys.readonly*.
You can set the scope for this method like this: `surveys2 --scope <scope> results get ...`
# Required Scalar Argument
* **&lt;survey-url-id&gt;** *(string)*
    - External URL ID for the survey.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ResultsGetRequest:
  result-mask:
    projection: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .result-mask    projection=eirmod`
    - No description provided.



### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.


# Optional Output Flags

The method's return value a JSON encoded structure, which will be written to standard output by default.

As this method supports **media download**, you may specify the `-p alt=media` flag to set the output to be an octet stream of the underlying media. In that case, you will not receive JSON output anymore.

* **-o out**
    - *out* specifies the *destination* to which to write the server's result to.
      It will either be a JSON-encoded structure, or the media file you are downloading.
      The *destination* may be `-` to indicate standard output, or a filepath that is to contain the received bytes.
      If unset, it defaults to standard output.
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
