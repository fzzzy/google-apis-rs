Searches for media items in a user&#39;s Google Photos library.
If no filters are set, then all media items in the user&#39;s library are
returned.
If an album is set, all media items in the specified album are returned.
If filters are specified, media items that match the filters from the user&#39;s
library are listed.
If you set both the album and the filters, the request results in an error.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/photoslibrary*
* *https://www.googleapis.com/auth/photoslibrary.readonly*
* *https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/photoslibrary*.
You can set the scope for this method like this: `photoslibrary1 --scope <scope> media-items search ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
SearchMediaItemsRequest:
  album-id: string
  filters:
    content-filter:
      excluded-content-categories: [string]
      included-content-categories: [string]
    exclude-non-app-created-data: boolean
    include-archived-media: boolean
    media-type-filter:
      media-types: [string]
  page-size: integer
  page-token: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    album-id=sea`
    - Identifier of an album. If populated, lists all media items in
        specified album. Can&#39;t set in conjunction with any filters.
* `filters.content-filter    excluded-content-categories=lorem`
    - The set of categories which are not to be included in the media item search
        results. The items in the set are ORed. There&#39;s a maximum of 10
        `excludedContentCategories` per request.
    - Each invocation of this argument appends the given value to the array.
* `included-content-categories=eos`
    - The set of categories to be included in the media item search results.
        The items in the set are ORed. There&#39;s a maximum of 10
        `includedContentCategories` per request.
    - Each invocation of this argument appends the given value to the array.

* `..    exclude-non-app-created-data=false`
    - If set, the results exclude media items that were not created by this app.
        Defaults to false (all media items are returned). This field is ignored if
        the photoslibrary.readonly.appcreateddata scope is used.
* `include-archived-media=false`
    - If set, the results include media items that the user has archived.
        Defaults to false (archived media items aren&#39;t included).
* `media-type-filter    media-types=dolor`
    - The types of media items to be included. This field should be populated
        with only one media type. If you specify multiple media types, it results
        in an error.
    - Each invocation of this argument appends the given value to the array.


* `...    page-size=62`
    - Maximum number of media items to return in the response. The default number
        of media items to return at a time is 25. The maximum
        `pageSize` is 100.
* `page-token=elitr`
    - A continuation token to get the next page of the results. Adding this to
        the request returns the rows after the `pageToken`. The `pageToken` should
        be the value returned in the `nextPageToken` parameter in the response to
        the `searchMediaItems` request.


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
