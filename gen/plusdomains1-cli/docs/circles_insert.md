Create a new circle for the authenticated user.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/plus.circles.write*
* *https://www.googleapis.com/auth/plus.login*
* *https://www.googleapis.com/auth/plus.me*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/plus.circles.write*.
You can set the scope for this method like this: `plusdomains1 --scope <scope> circles insert ...`
# Required Scalar Argument
* **&lt;user-id&gt;** *(string)*
    - The ID of the user to create the circle on behalf of. The value &#34;me&#34; can be used to indicate the authenticated user.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Circle:
  description: string
  display-name: string
  etag: string
  id: string
  kind: string
  people:
    total-items: integer
  self-link: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    description=amet`
    - The description of this circle.
* `display-name=et`
    - The circle name.
* `etag=consetetur`
    - ETag of this response for caching purposes.
* `id=ut`
    - The ID of the circle.
* `kind=ea`
    - Identifies this resource as a circle. Value: &#34;plus#circle&#34;.
* `people    total-items=21`
    - The total number of people in this circle.

* `..    self-link=dolor`
    - Link to this circle resource


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