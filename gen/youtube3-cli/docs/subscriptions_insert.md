Adds a subscription for the authenticated user&#39;s channel.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.force-ssl*
* *https://www.googleapis.com/auth/youtubepartner*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube*.
You can set the scope for this method like this: `youtube3 --scope <scope> subscriptions insert ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Subscription:
  content-details:
    activity-type: string
    new-item-count: integer
    total-item-count: integer
  etag: string
  id: string
  kind: string
  snippet:
    channel-id: string
    channel-title: string
    description: string
    published-at: string
    resource-id:
      channel-id: string
      kind: string
      playlist-id: string
      video-id: string
    thumbnails:
      default:
        height: integer
        url: string
        width: integer
      high:
        height: integer
        url: string
        width: integer
      maxres:
        height: integer
        url: string
        width: integer
      medium:
        height: integer
        url: string
        width: integer
      standard:
        height: integer
        url: string
        width: integer
    title: string
  subscriber-snippet:
    channel-id: string
    description: string
    thumbnails:
      default:
        height: integer
        url: string
        width: integer
      high:
        height: integer
        url: string
        width: integer
      maxres:
        height: integer
        url: string
        width: integer
      medium:
        height: integer
        url: string
        width: integer
      standard:
        height: integer
        url: string
        width: integer
    title: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .content-details    activity-type=dolor`
    - The type of activity this subscription is for (only uploads, everything).
* `new-item-count=51`
    - The number of new items in the subscription since its content was last read.
* `total-item-count=4`
    - The approximate number of items that the subscription points to.

* `..    etag=lorem`
    - Etag of this resource.
* `id=kasd`
    - The ID that YouTube uses to uniquely identify the subscription.
* `kind=nonumy`
    - Identifies what kind of resource this is. Value: the fixed string &#34;youtube#subscription&#34;.
* `snippet    channel-id=dolor`
    - The ID that YouTube uses to uniquely identify the subscriber&#39;s channel.
* `channel-title=diam`
    - Channel title for the channel that the subscription belongs to.
* `description=no`
    - The subscription&#39;s details.
* `published-at=tempor`
    - The date and time that the subscription was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `resource-id    channel-id=dolor`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
* `kind=et`
    - The type of the API resource.
* `playlist-id=ipsum`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
* `video-id=gubergren`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.

* `..thumbnails.default    height=69`
    - (Optional) Height of the thumbnail image.
* `url=lorem`
    - The thumbnail image&#39;s URL.
* `width=32`
    - (Optional) Width of the thumbnail image.

* `..high    height=32`
    - (Optional) Height of the thumbnail image.
* `url=accusam`
    - The thumbnail image&#39;s URL.
* `width=55`
    - (Optional) Width of the thumbnail image.

* `..maxres    height=32`
    - (Optional) Height of the thumbnail image.
* `url=diam`
    - The thumbnail image&#39;s URL.
* `width=80`
    - (Optional) Width of the thumbnail image.

* `..medium    height=3`
    - (Optional) Height of the thumbnail image.
* `url=tempor`
    - The thumbnail image&#39;s URL.
* `width=35`
    - (Optional) Width of the thumbnail image.

* `..standard    height=59`
    - (Optional) Height of the thumbnail image.
* `url=et`
    - The thumbnail image&#39;s URL.
* `width=35`
    - (Optional) Width of the thumbnail image.


* `...    title=eos`
    - The subscription&#39;s title.

* `..subscriber-snippet    channel-id=gubergren`
    - The channel ID of the subscriber.
* `description=dolores`
    - The description of the subscriber.
* `thumbnails.default    height=14`
    - (Optional) Height of the thumbnail image.
* `url=dolore`
    - The thumbnail image&#39;s URL.
* `width=26`
    - (Optional) Width of the thumbnail image.

* `..high    height=23`
    - (Optional) Height of the thumbnail image.
* `url=takimata`
    - The thumbnail image&#39;s URL.
* `width=98`
    - (Optional) Width of the thumbnail image.

* `..maxres    height=1`
    - (Optional) Height of the thumbnail image.
* `url=lorem`
    - The thumbnail image&#39;s URL.
* `width=93`
    - (Optional) Width of the thumbnail image.

* `..medium    height=43`
    - (Optional) Height of the thumbnail image.
* `url=rebum.`
    - The thumbnail image&#39;s URL.
* `width=64`
    - (Optional) Width of the thumbnail image.

* `..standard    height=41`
    - (Optional) Height of the thumbnail image.
* `url=et`
    - The thumbnail image&#39;s URL.
* `width=55`
    - (Optional) Width of the thumbnail image.


* `...    title=aliquyam`
    - The title of the subscriber.



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
