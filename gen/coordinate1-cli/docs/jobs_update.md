Updates a job. Fields that are set in the job state will be updated.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/coordinate* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/coordinate*.
You can set the scope for this method like this: `coordinate1 --scope <scope> jobs update ...`
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
Job:
  id: string
  kind: string
  state:
    assignee: string
    custom-fields:
      kind: string
    customer-name: string
    customer-phone-number: string
    kind: string
    location:
      address-line: [string]
      kind: string
      lat: number
      lng: number
    note: [string]
    progress: string
    title: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    id=et`
    - Job id.
* `kind=duo`
    - Identifies this object as a job.
* `state    assignee=aliquyam`
    - Email address of the assignee, or the string &#34;DELETED_USER&#34; if the account is no longer available.
* `custom-fields    kind=sea`
    - Identifies this object as a collection of custom fields.

* `..    customer-name=lorem`
    - Customer name.
* `customer-phone-number=eos`
    - Customer phone number.
* `kind=erat`
    - Identifies this object as a job state.
* `location    address-line=sadipscing`
    - Address.
    - Each invocation of this argument appends the given value to the array.
* `kind=dolor`
    - Identifies this object as a location.
* `lat=0.618673220052`
    - Latitude.
* `lng=0.577297745448`
    - Longitude.

* `..    note=amet`
    - Note added to the job.
    - Each invocation of this argument appends the given value to the array.
* `progress=no`
    - Job progress.
* `title=labore`
    - Job title.



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

* **-p lng=number**
    - The longitude coordinate of this job&#39;s location.

* **-p assignee=string**
    - Assignee email address, or empty string to unassign.

* **-p progress=string**
    - Job progress

* **-p lat=number**
    - The latitude coordinate of this job&#39;s location.

* **-p custom-field=string**
    - Sets the value of custom fields. To set a custom field, pass the field id (from /team/teamId/custom_fields), a URL escaped &#39;=&#39; character, and the desired value as a parameter. For example, customField=12%3DAlice. Repeat the parameter for each custom field. Note that &#39;=&#39; cannot appear in the parameter value. Specifying an invalid, or inactive enum field will result in an error 500.

* **-p note=string**
    - Job note as newline (Unix) separated string

* **-p customer-name=string**
    - Customer name

* **-p title=string**
    - Job title

* **-p address=string**
    - Job address as newline (Unix) separated string

* **-p customer-phone-number=string**
    - Customer phone number

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