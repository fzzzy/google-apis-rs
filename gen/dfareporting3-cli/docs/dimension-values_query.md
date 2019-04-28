Retrieves list of report dimension values for a list of filters.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfareporting* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfareporting*.
You can set the scope for this method like this: `dfareporting3 --scope <scope> dimension-values query ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - The DFA user profile ID.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
DimensionValueRequest:
  dimension-name: string
  end-date: string
  kind: string
  start-date: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    dimension-name=tempor`
    - The name of the dimension for which values should be requested.
* `end-date=dolor`
    - The end date of the date range for which to retrieve dimension values. A string of the format &#34;yyyy-MM-dd&#34;.
* `kind=ea`
    - The kind of request this is, in this case dfareporting#dimensionValueRequest.
* `start-date=elitr`
    - The start date of the date range for which to retrieve dimension values. A string of the format &#34;yyyy-MM-dd&#34;.


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

* **-p max-results=integer**
    - Maximum number of results to return.

* **-p page-token=string**
    - The value of the nextToken from the previous result page.

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