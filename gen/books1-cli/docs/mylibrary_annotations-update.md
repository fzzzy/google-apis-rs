Updates an existing annotation.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/books* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/books*.
You can set the scope for this method like this: `books1 --scope <scope> mylibrary annotations-update ...`
# Required Scalar Argument
* **&lt;annotation-id&gt;** *(string)*
    - The ID for the annotation to update.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Annotation:
  after-selected-text: string
  before-selected-text: string
  client-version-ranges:
    cfi-range:
      end-offset: string
      end-position: string
      start-offset: string
      start-position: string
    content-version: string
    gb-image-range:
      end-offset: string
      end-position: string
      start-offset: string
      start-position: string
    gb-text-range:
      end-offset: string
      end-position: string
      start-offset: string
      start-position: string
    image-cfi-range:
      end-offset: string
      end-position: string
      start-offset: string
      start-position: string
  created: string
  current-version-ranges:
    cfi-range:
      end-offset: string
      end-position: string
      start-offset: string
      start-position: string
    content-version: string
    gb-image-range:
      end-offset: string
      end-position: string
      start-offset: string
      start-position: string
    gb-text-range:
      end-offset: string
      end-position: string
      start-offset: string
      start-position: string
    image-cfi-range:
      end-offset: string
      end-position: string
      start-offset: string
      start-position: string
  data: string
  deleted: boolean
  highlight-style: string
  id: string
  kind: string
  layer-id: string
  layer-summary:
    allowed-character-count: integer
    limit-type: string
    remaining-character-count: integer
  page-ids: [string]
  selected-text: string
  self-link: string
  updated: string
  volume-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    after-selected-text=dolor`
    - Anchor text after excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty.
* `before-selected-text=et`
    - Anchor text before excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty.
* `client-version-ranges.cfi-range    end-offset=consetetur`
    - The offset from the ending position.
* `end-position=amet.`
    - The ending position for the range.
* `start-offset=voluptua.`
    - The offset from the starting position.
* `start-position=lorem`
    - The starting position for the range.

* `..    content-version=gubergren`
    - Content version the client sent in.
* `gb-image-range    end-offset=justo`
    - The offset from the ending position.
* `end-position=sit`
    - The ending position for the range.
* `start-offset=vero`
    - The offset from the starting position.
* `start-position=diam`
    - The starting position for the range.

* `..gb-text-range    end-offset=rebum.`
    - The offset from the ending position.
* `end-position=consetetur`
    - The ending position for the range.
* `start-offset=sadipscing`
    - The offset from the starting position.
* `start-position=vero`
    - The starting position for the range.

* `..image-cfi-range    end-offset=sadipscing`
    - The offset from the ending position.
* `end-position=invidunt`
    - The ending position for the range.
* `start-offset=consetetur`
    - The offset from the starting position.
* `start-position=dolore`
    - The starting position for the range.


* `...    created=duo`
    - Timestamp for the created time of this annotation.
* `current-version-ranges.cfi-range    end-offset=aliquyam`
    - The offset from the ending position.
* `end-position=lorem`
    - The ending position for the range.
* `start-offset=et`
    - The offset from the starting position.
* `start-position=clita`
    - The starting position for the range.

* `..    content-version=consetetur`
    - Content version applicable to ranges below.
* `gb-image-range    end-offset=takimata`
    - The offset from the ending position.
* `end-position=nonumy`
    - The ending position for the range.
* `start-offset=kasd`
    - The offset from the starting position.
* `start-position=sanctus`
    - The starting position for the range.

* `..gb-text-range    end-offset=takimata`
    - The offset from the ending position.
* `end-position=at`
    - The ending position for the range.
* `start-offset=labore`
    - The offset from the starting position.
* `start-position=invidunt`
    - The starting position for the range.

* `..image-cfi-range    end-offset=ea`
    - The offset from the ending position.
* `end-position=sadipscing`
    - The ending position for the range.
* `start-offset=rebum.`
    - The offset from the starting position.
* `start-position=dolore`
    - The starting position for the range.


* `...    data=nonumy`
    - User-created data for this annotation.
* `deleted=true`
    - Indicates that this annotation is deleted.
* `highlight-style=aliquyam`
    - The highlight style for this annotation.
* `id=sit`
    - Id of this annotation, in the form of a GUID.
* `kind=eirmod`
    - Resource type.
* `layer-id=consetetur`
    - The layer this annotation is for.
* `layer-summary    allowed-character-count=16`
    - Maximum allowed characters on this layer, especially for the &#34;copy&#34; layer.
* `limit-type=sed`
    - Type of limitation on this layer. &#34;limited&#34; or &#34;unlimited&#34; for the &#34;copy&#34; layer.
* `remaining-character-count=85`
    - Remaining allowed characters on this layer, especially for the &#34;copy&#34; layer.

* `..    page-ids=gubergren`
    - Pages that this annotation spans.
    - Each invocation of this argument appends the given value to the array.
* `selected-text=aliquyam`
    - Excerpt from the volume.
* `self-link=eos`
    - URL to this resource.
* `updated=tempor`
    - Timestamp for the last time this annotation was modified.
* `volume-id=sea`
    - The volume that this annotation belongs to.


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

* **-p source=string**
    - String to identify the originator of this request.

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
