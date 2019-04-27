Replaces the schedule of a job with the provided schedule.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/coordinate* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/coordinate*.
You can set the scope for this method like this: `coordinate1 --scope <scope> schedule update ...`
# Required Scalar Arguments
* **&lt;team-id&gt;** *(string)*
    - Team ID
* **&lt;job-id&gt;** *(string)*
    - Job number
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Schedule:
  all-day: boolean
  duration: string
  end-time: string
  kind: string
  start-time: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    all-day=false`
    - Whether the job is scheduled for the whole day. Time of day in start/end times is ignored if this is true.
* `duration=sea`
    - Job duration in milliseconds.
* `end-time=et`
    - Scheduled end time in milliseconds since epoch.
* `kind=duo`
    - Identifies this object as a job schedule.
* `start-time=et`
    - Scheduled start time in milliseconds since epoch.


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

* **-p all-day=boolean**
    - Whether the job is scheduled for the whole day. Time of day in start/end times is ignored if this is true.

* **-p start-time=string**
    - Scheduled start time in milliseconds since epoch.

* **-p duration=string**
    - Job duration in milliseconds.

* **-p end-time=string**
    - Scheduled end time in milliseconds since epoch.

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
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.

* **-p user-ip=string**
    - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
