Push changes to activities
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/admin.reports.audit.readonly* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/admin.reports.audit.readonly*.
You can set the scope for this method like this: `admin1-reports --scope <scope> activities watch ...`
# Required Scalar Arguments
* **&lt;user-key&gt;** *(string)*
    - Represents the profile id or the user email for which the data should be filtered. When &#39;all&#39; is specified as the userKey, it returns usageReports for all users.
* **&lt;application-name&gt;** *(string)*
    - Application name for which the events are to be retrieved.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Channel:
  address: string
  expiration: string
  id: string
  kind: string
  params: { string: string }
  payload: boolean
  resource-id: string
  resource-uri: string
  token: string
  type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    address=eirmod`
    - The address where notifications are delivered for this channel.
* `expiration=sit`
    - Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
* `id=stet`
    - A UUID or similar unique string that identifies this channel.
* `kind=sed`
    - Identifies this as a notification channel used to watch for changes to a resource. Value: the fixed string &#34;api#channel&#34;.
* `params=key=et`
    - Additional parameters controlling delivery channel behavior. Optional.
    - the value will be associated with the given `key`
* `payload=true`
    - A Boolean value to indicate whether payload is wanted. Optional.
* `resource-id=kasd`
    - An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.
* `resource-uri=accusam`
    - A version-specific identifier for the watched resource.
* `token=takimata`
    - An arbitrary string delivered to the target address with each notification delivered over this channel. Optional.
* `type=justo`
    - The type of delivery mechanism used for this channel.


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

* **-p start-time=string**
    - Return events which occurred at or after this time.

* **-p max-results=integer**
    - Number of activity records to be shown in each page.

* **-p filters=string**
    - Event parameters in the form [parameter1 name][operator][parameter1 value],[parameter2 name][operator][parameter2 value],...

* **-p actor-ip-address=string**
    - IP Address of host where the event was performed. Supports both IPv4 and IPv6 addresses.

* **-p event-name=string**
    - Name of the event being queried.

* **-p page-token=string**
    - Token to specify next page.

* **-p end-time=string**
    - Return events which occurred at or before this time.

* **-p customer-id=string**
    - Represents the customer for which the data is to be fetched.

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
