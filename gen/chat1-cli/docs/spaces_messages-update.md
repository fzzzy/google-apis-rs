Updates a message.
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Resource name, in the form &#34;spaces/*/messages/*&#34;.
        
        Example: spaces/AAAAMpdlehY/messages/UMxbHmzDlr4.UMxbHmzDlr4
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Message:
  action-response:
    type: string
    url: string
  argument-text: string
  create-time: string
  fallback-text: string
  name: string
  preview-text: string
  sender:
    display-name: string
    name: string
    type: string
  space:
    display-name: string
    name: string
    type: string
  text: string
  thread:
    name: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .action-response    type=dolores`
    - The type of bot response.
* `url=gubergren`
    - URL for users to auth or config. (Only for REQUEST_CONFIG response types.)

* `..    argument-text=sadipscing`
    - Plain-text body of the message with all bot mentions stripped out.
* `create-time=aliquyam`
    - Output only. The time at which the message was created in Hangouts Chat
        server.
* `fallback-text=ea`
    - A plain-text description of the message&#39;s cards, used when the actual cards
        cannot be displayed (e.g. mobile notifications).
* `name=no`
    - Resource name, in the form &#34;spaces/*/messages/*&#34;.
        
        Example: spaces/AAAAMpdlehY/messages/UMxbHmzDlr4.UMxbHmzDlr4
* `preview-text=justo`
    - Text for generating preview chips. This text will not be displayed to the
        user, but any links to images, web pages, videos, etc. included here will
        generate preview chips.
* `sender    display-name=justo`
    - The user&#39;s display name.
* `name=et`
    - Resource name, in the format &#34;users/*&#34;.
* `type=et`
    - User type.

* `..space    display-name=diam`
    - Output only. The display name (only if the space is a room).
* `name=ipsum`
    - Resource name of the space, in the form &#34;spaces/*&#34;.
        
        Example: spaces/AAAAMpdlehYs
* `type=lorem`
    - Output only. The type of a space.

* `..    text=et`
    - Plain-text body of the message.
* `thread    name=duo`
    - Resource name, in the form &#34;spaces/*/threads/*&#34;.
        
        Example: spaces/AAAAMpdlehY/threads/UMxbHmzDlr4



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

* **-p update-mask=string**
    - Required. The field paths to be updated.
        
        Currently supported field paths: &#34;text&#34;, &#34;cards&#34;.

# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p $-xgafv=string**
    - V1 error format.

* **-p access-token=string**
    - OAuth access token.

* **-p alt=string**
    - Data format for response.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).
