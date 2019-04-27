Updates a calendar resource.

This method supports patch semantics, meaning you only need to include the fields you wish to update. Fields that are not present in the request will be preserved. This method supports patch semantics.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/admin.directory.resource.calendar* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/admin.directory.resource.calendar*.
You can set the scope for this method like this: `admin1-directory --scope <scope> resources calendars-patch ...`
# Required Scalar Arguments
* **&lt;customer&gt;** *(string)*
    - The unique ID for the customer&#39;s G Suite account. As an account administrator, you can also use the my_customer alias to represent your account&#39;s customer ID.
* **&lt;calendar-resource-id&gt;** *(string)*
    - The unique ID of the calendar resource to update.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CalendarResource:
  building-id: string
  capacity: integer
  etags: string
  floor-name: string
  floor-section: string
  generated-resource-name: string
  kind: string
  resource-category: string
  resource-description: string
  resource-email: string
  resource-id: string
  resource-name: string
  resource-type: string
  user-visible-description: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    building-id=takimata`
    - Unique ID for the building a resource is located in.
* `capacity=32`
    - Capacity of a resource, number of seats in a room.
* `etags=consetetur`
    - ETag of the resource.
* `floor-name=erat`
    - Name of the floor a resource is located on.
* `floor-section=amet.`
    - Name of the section within a floor a resource is located in.
* `generated-resource-name=dolores`
    - The read-only auto-generated name of the calendar resource which includes metadata about the resource such as building name, floor, capacity, etc. For example, &#34;NYC-2-Training Room 1A (16)&#34;.
* `kind=dolores`
    - The type of the resource. For calendar resources, the value is admin#directory#resources#calendars#CalendarResource.
* `resource-category=et`
    - The category of the calendar resource. Either CONFERENCE_ROOM or OTHER. Legacy data is set to CATEGORY_UNKNOWN.
* `resource-description=sed`
    - Description of the resource, visible only to admins.
* `resource-email=et`
    - The read-only email for the calendar resource. Generated as part of creating a new calendar resource.
* `resource-id=aliquyam`
    - The unique ID for the calendar resource.
* `resource-name=nonumy`
    - The name of the calendar resource. For example, &#34;Training Room 1A&#34;.
* `resource-type=sit`
    - The type of the calendar resource, intended for non-room resources.
* `user-visible-description=aliquyam`
    - Description of the resource, visible to users and admins.


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
