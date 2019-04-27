Modifies a playlist item. For example, you could update the item&#39;s position in the playlist.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.force-ssl*
* *https://www.googleapis.com/auth/youtubepartner*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube*.
You can set the scope for this method like this: `youtube3 --scope <scope> playlist-items update ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
PlaylistItem:
  content-details:
    end-at: string
    note: string
    start-at: string
    video-id: string
    video-published-at: string
  etag: string
  id: string
  kind: string
  snippet:
    channel-id: string
    channel-title: string
    description: string
    playlist-id: string
    position: integer
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
  status:
    privacy-status: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .content-details    end-at=rebum.`
    - The time, measured in seconds from the start of the video, when the video should stop playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) By default, assume that the video.endTime is the end of the video.
* `note=sit`
    - A user-generated note for this item.
* `start-at=lorem`
    - The time, measured in seconds from the start of the video, when the video should start playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) The default value is 0.
* `video-id=sanctus`
    - The ID that YouTube uses to uniquely identify a video. To retrieve the video resource, set the id query parameter to this value in your API request.
* `video-published-at=accusam`
    - The date and time that the video was published to YouTube. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.

* `..    etag=dolore`
    - Etag of this resource.
* `id=est`
    - The ID that YouTube uses to uniquely identify the playlist item.
* `kind=invidunt`
    - Identifies what kind of resource this is. Value: the fixed string &#34;youtube#playlistItem&#34;.
* `snippet    channel-id=accusam`
    - The ID that YouTube uses to uniquely identify the user that added the item to the playlist.
* `channel-title=accusam`
    - Channel title for the channel that the playlist item belongs to.
* `description=invidunt`
    - The item&#39;s description.
* `playlist-id=eirmod`
    - The ID that YouTube uses to uniquely identify the playlist that the playlist item is in.
* `position=49`
    - The order in which the item appears in the playlist. The value uses a zero-based index, so the first item has a position of 0, the second item has a position of 1, and so forth.
* `published-at=elitr`
    - The date and time that the item was added to the playlist. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `resource-id    channel-id=amet`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
* `kind=ut`
    - The type of the API resource.
* `playlist-id=duo`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
* `video-id=et`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.

* `..thumbnails.default    height=94`
    - (Optional) Height of the thumbnail image.
* `url=et`
    - The thumbnail image&#39;s URL.
* `width=80`
    - (Optional) Width of the thumbnail image.

* `..high    height=38`
    - (Optional) Height of the thumbnail image.
* `url=consetetur`
    - The thumbnail image&#39;s URL.
* `width=18`
    - (Optional) Width of the thumbnail image.

* `..maxres    height=63`
    - (Optional) Height of the thumbnail image.
* `url=at`
    - The thumbnail image&#39;s URL.
* `width=86`
    - (Optional) Width of the thumbnail image.

* `..medium    height=54`
    - (Optional) Height of the thumbnail image.
* `url=ipsum`
    - The thumbnail image&#39;s URL.
* `width=37`
    - (Optional) Width of the thumbnail image.

* `..standard    height=88`
    - (Optional) Height of the thumbnail image.
* `url=stet`
    - The thumbnail image&#39;s URL.
* `width=64`
    - (Optional) Width of the thumbnail image.


* `...    title=et`
    - The item&#39;s title.

* `..status    privacy-status=tempor`
    - This resource&#39;s privacy status.



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

* **-p on-behalf-of-content-owner=string**
    - Note: This parameter is intended exclusively for YouTube content partners.
        
        The onBehalfOfContentOwner parameter indicates that the request&#39;s authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

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
