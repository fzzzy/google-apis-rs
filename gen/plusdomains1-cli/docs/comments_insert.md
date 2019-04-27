Create a new comment in reply to an activity.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/plus.login*
* *https://www.googleapis.com/auth/plus.stream.write*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/plus.login*.
You can set the scope for this method like this: `plusdomains1 --scope <scope> comments insert ...`
# Required Scalar Argument
* **&lt;activity-id&gt;** *(string)*
    - The ID of the activity to reply to.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Comment:
  actor:
    client-specific-actor-info:
      youtube-actor-info:
        channel-id: string
    display-name: string
    id: string
    image:
      url: string
    url: string
    verification:
      ad-hoc-verified: string
  etag: string
  id: string
  kind: string
  object:
    content: string
    object-type: string
    original-content: string
  plusoners:
    total-items: integer
  published: string
  self-link: string
  updated: string
  verb: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .actor.client-specific-actor-info.youtube-actor-info    channel-id=sadipscing`
    - ID of the YouTube channel owned by the Actor.


* `...    display-name=vero`
    - The name of this actor, suitable for display.
* `id=sadipscing`
    - The ID of the actor.
* `image    url=invidunt`
    - The URL of the actor&#39;s profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side.

* `..    url=consetetur`
    - A link to the Person resource for this actor.
* `verification    ad-hoc-verified=dolore`
    - Verification for one-time or manual processes.


* `...    etag=duo`
    - ETag of this response for caching purposes.
* `id=aliquyam`
    - The ID of this comment.
* `kind=lorem`
    - Identifies this resource as a comment. Value: &#34;plus#comment&#34;.
* `object    content=et`
    - The HTML-formatted content, suitable for display.
* `object-type=clita`
    - The object type of this comment. Possible values are:  
        - &#34;comment&#34; - A comment in reply to an activity.
* `original-content=consetetur`
    - The content (text) as provided by the author, stored without any HTML formatting. When creating or updating a comment, this value must be supplied as plain text in the request.

* `..plusoners    total-items=43`
    - Total number of people who +1&#39;d this comment.

* `..    published=nonumy`
    - The time at which this comment was initially published. Formatted as an RFC 3339 timestamp.
* `self-link=kasd`
    - Link to this comment resource.
* `updated=sanctus`
    - The time at which this comment was last updated. Formatted as an RFC 3339 timestamp.
* `verb=takimata`
    - This comment&#39;s verb, indicating what action was performed. Possible values are:  
        - &#34;post&#34; - Publish content to the stream.


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
