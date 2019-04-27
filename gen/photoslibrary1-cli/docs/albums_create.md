Creates an album in a user&#39;s Google Photos library.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/photoslibrary*
* *https://www.googleapis.com/auth/photoslibrary.appendonly*
* *https://www.googleapis.com/auth/photoslibrary.sharing*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/photoslibrary*.
You can set the scope for this method like this: `photoslibrary1 --scope <scope> albums create ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CreateAlbumRequest:
  album:
    cover-photo-base-url: string
    cover-photo-media-item-id: string
    id: string
    is-writeable: boolean
    media-items-count: int64
    product-url: string
    share-info:
      is-joined: boolean
      share-token: string
      shareable-url: string
      shared-album-options:
        is-collaborative: boolean
        is-commentable: boolean
    title: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .album    cover-photo-base-url=sea`
    - [Output only] A URL to the cover photo&#39;s bytes. This shouldn&#39;t be used as
        is. Parameters should be appended to this URL before use. For example,
        `&#39;=w2048-h1024&#39;` sets the dimensions of
        the cover photo to have a width of 2048 px and height of 1024 px.
* `cover-photo-media-item-id=nonumy`
    - [Output only] Identifier for the media item associated with the cover
        photo.
* `id=dolores`
    - [Ouput only] Identifier for the album. This is a persistent identifier that
        can be used between sessions to identify this album.
* `is-writeable=false`
    - [Output only] True if you can create media items in this album.
        This field is based on the scopes granted and permissions of the album. If
        the scopes are changed or permissions of the album are changed, this field
        is updated.
* `media-items-count=-95`
    - [Output only] The number of media items in the album.
* `product-url=aliquyam`
    - [Output only] Google Photos URL for the album. The user needs to be signed
        in to their Google Photos account to access this link.
* `share-info    is-joined=false`
    - True if the user has joined the album. This is always true for the owner
        of the shared album.
* `share-token=no`
    - A token that can be used by other users to join this shared album via the
        API.
* `shareable-url=justo`
    - A link to the album that&#39;s now shared on the Google Photos website and app.
        Anyone with the link can access this shared album and see all of the items
        present in the album.
* `shared-album-options    is-collaborative=true`
    - True if the shared album allows collaborators (users who have joined
        the album) to add media items to it. Defaults to false.
* `is-commentable=true`
    - True if the shared album allows the owner and the collaborators (users
        who have joined the album) to add comments to the album. Defaults to false.


* `...    title=et`
    - Name of the album displayed to the user in their Google Photos account.
        This string shouldn&#39;t be more than 500 characters.



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
