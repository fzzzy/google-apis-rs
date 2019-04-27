Posts a bulletin for a specific channel. (The user submitting the request must be authorized to act on the channel&#39;s behalf.)

Note: Even though an activity resource can contain information about actions like a user rating a video or marking a video as a favorite, you need to use other API methods to generate those activity resources. For example, you would use the API&#39;s videos.rate() method to rate a video and the playlistItems.insert() method to mark a video as a favorite.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.force-ssl*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube*.
You can set the scope for this method like this: `youtube3 --scope <scope> activities insert ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Activity:
  content-details:
    bulletin:
      resource-id:
        channel-id: string
        kind: string
        playlist-id: string
        video-id: string
    channel-item:
      resource-id:
        channel-id: string
        kind: string
        playlist-id: string
        video-id: string
    comment:
      resource-id:
        channel-id: string
        kind: string
        playlist-id: string
        video-id: string
    favorite:
      resource-id:
        channel-id: string
        kind: string
        playlist-id: string
        video-id: string
    like:
      resource-id:
        channel-id: string
        kind: string
        playlist-id: string
        video-id: string
    playlist-item:
      playlist-id: string
      playlist-item-id: string
      resource-id:
        channel-id: string
        kind: string
        playlist-id: string
        video-id: string
    promoted-item:
      ad-tag: string
      click-tracking-url: string
      creative-view-url: string
      cta-type: string
      custom-cta-button-text: string
      description-text: string
      destination-url: string
      forecasting-url: [string]
      impression-url: [string]
      video-id: string
    recommendation:
      reason: string
      resource-id:
        channel-id: string
        kind: string
        playlist-id: string
        video-id: string
      seed-resource-id:
        channel-id: string
        kind: string
        playlist-id: string
        video-id: string
    social:
      author: string
      image-url: string
      reference-url: string
      resource-id:
        channel-id: string
        kind: string
        playlist-id: string
        video-id: string
      type: string
    subscription:
      resource-id:
        channel-id: string
        kind: string
        playlist-id: string
        video-id: string
    upload:
      video-id: string
  etag: string
  id: string
  kind: string
  snippet:
    channel-id: string
    channel-title: string
    description: string
    group-id: string
    published-at: string
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
    type: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .content-details.bulletin.resource-id    channel-id=eirmod`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
* `kind=sit`
    - The type of the API resource.
* `playlist-id=stet`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
* `video-id=sed`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.


* `...channel-item.resource-id    channel-id=et`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
* `kind=dolores`
    - The type of the API resource.
* `playlist-id=kasd`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
* `video-id=accusam`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.


* `...comment.resource-id    channel-id=takimata`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
* `kind=justo`
    - The type of the API resource.
* `playlist-id=amet.`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
* `video-id=erat`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.


* `...favorite.resource-id    channel-id=labore`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
* `kind=sea`
    - The type of the API resource.
* `playlist-id=nonumy`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
* `video-id=dolores`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.


* `...like.resource-id    channel-id=gubergren`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
* `kind=sadipscing`
    - The type of the API resource.
* `playlist-id=aliquyam`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
* `video-id=ea`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.


* `...playlist-item    playlist-id=no`
    - The value that YouTube uses to uniquely identify the playlist.
* `playlist-item-id=justo`
    - ID of the item within the playlist.
* `resource-id    channel-id=justo`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
* `kind=et`
    - The type of the API resource.
* `playlist-id=et`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
* `video-id=diam`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.


* `...promoted-item    ad-tag=ipsum`
    - The URL the client should fetch to request a promoted item.
* `click-tracking-url=lorem`
    - The URL the client should ping to indicate that the user clicked through on this promoted item.
* `creative-view-url=et`
    - The URL the client should ping to indicate that the user was shown this promoted item.
* `cta-type=duo`
    - The type of call-to-action, a message to the user indicating action that can be taken.
* `custom-cta-button-text=aliquyam`
    - The custom call-to-action button text. If specified, it will override the default button text for the cta_type.
* `description-text=sea`
    - The text description to accompany the promoted item.
* `destination-url=lorem`
    - The URL the client should direct the user to, if the user chooses to visit the advertiser&#39;s website.
* `forecasting-url=eos`
    - The list of forecasting URLs. The client should ping all of these URLs when a promoted item is not available, to indicate that a promoted item could have been shown.
    - Each invocation of this argument appends the given value to the array.
* `impression-url=erat`
    - The list of impression URLs. The client should ping all of these URLs to indicate that the user was shown this promoted item.
    - Each invocation of this argument appends the given value to the array.
* `video-id=sadipscing`
    - The ID that YouTube uses to uniquely identify the promoted video.

* `..recommendation    reason=dolor`
    - The reason that the resource is recommended to the user.
* `resource-id    channel-id=eirmod`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
* `kind=elitr`
    - The type of the API resource.
* `playlist-id=amet`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
* `video-id=no`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.

* `..seed-resource-id    channel-id=labore`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
* `kind=eirmod`
    - The type of the API resource.
* `playlist-id=dolore`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
* `video-id=invidunt`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.


* `...social    author=aliquyam`
    - The author of the social network post.
* `image-url=accusam`
    - An image of the post&#39;s author.
* `reference-url=lorem`
    - The URL of the social network post.
* `resource-id    channel-id=sea`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
* `kind=et`
    - The type of the API resource.
* `playlist-id=duo`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
* `video-id=et`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.

* `..    type=eirmod`
    - The name of the social network.

* `..subscription.resource-id    channel-id=sanctus`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
* `kind=et`
    - The type of the API resource.
* `playlist-id=amet`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
* `video-id=et`
    - The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.


* `...upload    video-id=consetetur`
    - The ID that YouTube uses to uniquely identify the uploaded video.


* `...    etag=ut`
    - Etag of this resource.
* `id=ea`
    - The ID that YouTube uses to uniquely identify the activity.
* `kind=sed`
    - Identifies what kind of resource this is. Value: the fixed string &#34;youtube#activity&#34;.
* `snippet    channel-id=dolor`
    - The ID that YouTube uses to uniquely identify the channel associated with the activity.
* `channel-title=dolor`
    - Channel title for the channel responsible for this activity
* `description=dolor`
    - The description of the resource primarily associated with the activity.
* `group-id=et`
    - The group ID associated with the activity. A group ID identifies user events that are associated with the same user and resource. For example, if a user rates a video and marks the same video as a favorite, the entries for those events would have the same group ID in the user&#39;s activity feed. In your user interface, you can avoid repetition by grouping events with the same groupId value.
* `published-at=consetetur`
    - The date and time that the video was uploaded. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `thumbnails.default    height=49`
    - (Optional) Height of the thumbnail image.
* `url=voluptua.`
    - The thumbnail image&#39;s URL.
* `width=45`
    - (Optional) Width of the thumbnail image.

* `..high    height=90`
    - (Optional) Height of the thumbnail image.
* `url=justo`
    - The thumbnail image&#39;s URL.
* `width=49`
    - (Optional) Width of the thumbnail image.

* `..maxres    height=75`
    - (Optional) Height of the thumbnail image.
* `url=diam`
    - The thumbnail image&#39;s URL.
* `width=35`
    - (Optional) Width of the thumbnail image.

* `..medium    height=56`
    - (Optional) Height of the thumbnail image.
* `url=sadipscing`
    - The thumbnail image&#39;s URL.
* `width=25`
    - (Optional) Width of the thumbnail image.

* `..standard    height=6`
    - (Optional) Height of the thumbnail image.
* `url=invidunt`
    - The thumbnail image&#39;s URL.
* `width=5`
    - (Optional) Width of the thumbnail image.


* `...    title=dolore`
    - The title of the resource primarily associated with the activity.
* `type=duo`
    - The type of activity that the resource describes.



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
