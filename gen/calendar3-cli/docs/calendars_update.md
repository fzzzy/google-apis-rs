Updates metadata for a calendar.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/calendar* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/calendar*.
You can set the scope for this method like this: `calendar3 --scope <scope> calendars update ...`
# Required Scalar Argument
* **&lt;calendar-id&gt;** *(string)*
    - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the &#34;primary&#34; keyword.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Calendar:
  conference-properties:
    allowed-conference-solution-types: [string]
  description: string
  etag: string
  id: string
  kind: string
  location: string
  summary: string
  time-zone: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .conference-properties    allowed-conference-solution-types=consetetur`
    - The types of conference solutions that are supported for this calendar.
        The possible values are:  
        - &#34;eventHangout&#34; 
        - &#34;eventNamedHangout&#34; 
        - &#34;hangoutsMeet&#34;  Optional.
    - Each invocation of this argument appends the given value to the array.

* `..    description=labore`
    - Description of the calendar. Optional.
* `etag=sed`
    - ETag of the resource.
* `id=ea`
    - Identifier of the calendar. To retrieve IDs call the calendarList.list() method.
* `kind=gubergren`
    - Type of the resource (&#34;calendar#calendar&#34;).
* `location=aliquyam`
    - Geographic location of the calendar as free-form text. Optional.
* `summary=eos`
    - Title of the calendar.
* `time-zone=tempor`
    - The time zone of the calendar. (Formatted as an IANA Time Zone Database name, e.g. &#34;Europe/Zurich&#34;.) Optional.


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
