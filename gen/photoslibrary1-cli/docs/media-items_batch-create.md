Creates one or more media items in a user&#39;s Google Photos library.

This is the second step for creating a media item. For details regarding
Step 1, uploading the raw bytes to a Google Server, see
&lt;a href=&#34;/photos/library/guides/upload-media&#34;&gt;Uploading media&lt;/a&gt;.

This call adds the media item to the library. If an album `id` is
specified, the call adds the media item to the album too. By default, the
media item will be added to the end of the library or album.

If an album `id` and position are both defined, the media item is
added to the album at the specified position.

If the call contains multiple media items, they&#39;re added at the specified
position.
If you are creating a media item in a shared album where you are not the
owner, you are not allowed to position the media item. Doing so will result
in a `BAD REQUEST` error.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/photoslibrary*
* *https://www.googleapis.com/auth/photoslibrary.appendonly*
* *https://www.googleapis.com/auth/photoslibrary.sharing*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/photoslibrary*.
You can set the scope for this method like this: `photoslibrary1 --scope <scope> media-items batch-create ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
BatchCreateMediaItemsRequest:
  album-id: string
  album-position:
    position: string
    relative-enrichment-item-id: string
    relative-media-item-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    album-id=lorem`
    - Identifier of the album where the media items are added. The media items
        are also added to the user&#39;s library. This is an optional field.
* `album-position    position=et`
    - Type of position, for a media or enrichment item.
* `relative-enrichment-item-id=duo`
    - The enrichment item to which the position is relative to.
        Only used when position type is AFTER_ENRICHMENT_ITEM.
* `relative-media-item-id=aliquyam`
    - The media item to which the position is relative to.
        Only used when position type is AFTER_MEDIA_ITEM.



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
