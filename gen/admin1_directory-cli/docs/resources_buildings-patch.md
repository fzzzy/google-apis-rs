Updates a building. This method supports patch semantics.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/admin.directory.resource.calendar* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/admin.directory.resource.calendar*.
You can set the scope for this method like this: `admin1-directory --scope <scope> resources buildings-patch ...`
# Required Scalar Arguments
* **&lt;customer&gt;** *(string)*
    - The unique ID for the customer&#39;s G Suite account. As an account administrator, you can also use the my_customer alias to represent your account&#39;s customer ID.
* **&lt;building-id&gt;** *(string)*
    - The ID of the building to update.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Building:
  building-id: string
  building-name: string
  coordinates:
    latitude: number
    longitude: number
  description: string
  etags: string
  floor-names: [string]
  kind: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    building-id=eos`
    - Unique identifier for the building. The maximum length is 100 characters.
* `building-name=dolores`
    - The building name as seen by users in Calendar. Must be unique for the customer. For example, &#34;NYC-CHEL&#34;. The maximum length is 100 characters.
* `coordinates    latitude=0.756086376331`
    - Latitude in decimal degrees.
* `longitude=0.056572721073`
    - Longitude in decimal degrees.

* `..    description=vero`
    - A brief description of the building. For example, &#34;Chelsea Market&#34;.
* `etags=consetetur`
    - ETag of the resource.
* `floor-names=eos`
    - The display names for all floors in this building. The floors are expected to be sorted in ascending order, from lowest floor to highest floor. For example, [&#34;B2&#34;, &#34;B1&#34;, &#34;L&#34;, &#34;1&#34;, &#34;2&#34;, &#34;2M&#34;, &#34;3&#34;, &#34;PH&#34;] Must contain at least one entry.
    - Each invocation of this argument appends the given value to the array.
* `kind=justo`
    - Kind of resource this is.


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
