Updates or insert a given session.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/fitness.activity.write* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/fitness.activity.write*.
You can set the scope for this method like this: `fitness1 --scope <scope> users sessions-update ...`
# Required Scalar Arguments
* **&lt;user-id&gt;** *(string)*
    - Create sessions for the person identified. Use me to indicate the authenticated user. Only me is supported at this time.
* **&lt;session-id&gt;** *(string)*
    - The ID of the session to be created.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Session:
  active-time-millis: string
  activity-type: integer
  application:
    details-url: string
    name: string
    package-name: string
    version: string
  description: string
  end-time-millis: string
  id: string
  modified-time-millis: string
  name: string
  start-time-millis: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    active-time-millis=dolor`
    - Session active time. While start_time_millis and end_time_millis define the full session time, the active time can be shorter and specified by active_time_millis. If the inactive time during the session is known, it should also be inserted via a com.google.activity.segment data point with a STILL activity value
* `activity-type=53`
    - The type of activity this session represents.
* `application    details-url=dolor`
    - An optional URI that can be used to link back to the application.
* `name=et`
    - The name of this application. This is required for REST clients, but we do not enforce uniqueness of this name. It is provided as a matter of convenience for other developers who would like to identify which REST created an Application or Data Source.
* `package-name=consetetur`
    - Package name for this application. This is used as a unique identifier when created by Android applications, but cannot be specified by REST clients. REST clients will have their developer project number reflected into the Data Source data stream IDs, instead of the packageName.
* `version=amet.`
    - Version of the application. You should update this field whenever the application changes in a way that affects the computation of the data.

* `..    description=voluptua.`
    - A description for this session.
* `end-time-millis=lorem`
    - An end time, in milliseconds since epoch, inclusive.
* `id=gubergren`
    - A client-generated identifier that is unique across all sessions owned by this particular user.
* `modified-time-millis=justo`
    - A timestamp that indicates when the session was last modified.
* `name=sit`
    - A human readable name of the session.
* `start-time-millis=vero`
    - A start time, in milliseconds since epoch, inclusive.


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

* **-p current-time-millis=string**
    - The client&#39;s current time in milliseconds since epoch.

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
