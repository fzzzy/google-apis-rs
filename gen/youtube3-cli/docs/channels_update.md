Updates a channel&#39;s metadata. Note that this method currently only supports updates to the channel resource&#39;s brandingSettings and invideoPromotion objects and their child properties.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.force-ssl*
* *https://www.googleapis.com/auth/youtubepartner*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube*.
You can set the scope for this method like this: `youtube3 --scope <scope> channels update ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Channel:
  audit-details:
    community-guidelines-good-standing: boolean
    content-id-claims-good-standing: boolean
    copyright-strikes-good-standing: boolean
    overall-good-standing: boolean
  branding-settings:
    channel:
      country: string
      default-language: string
      default-tab: string
      description: string
      featured-channels-title: string
      featured-channels-urls: [string]
      keywords: string
      moderate-comments: boolean
      profile-color: string
      show-browse-view: boolean
      show-related-channels: boolean
      title: string
      tracking-analytics-account-id: string
      unsubscribed-trailer: string
    image:
      background-image-url:
        default: string
        default-language:
          value: string
      banner-external-url: string
      banner-image-url: string
      banner-mobile-extra-hd-image-url: string
      banner-mobile-hd-image-url: string
      banner-mobile-image-url: string
      banner-mobile-low-image-url: string
      banner-mobile-medium-hd-image-url: string
      banner-tablet-extra-hd-image-url: string
      banner-tablet-hd-image-url: string
      banner-tablet-image-url: string
      banner-tablet-low-image-url: string
      banner-tv-high-image-url: string
      banner-tv-image-url: string
      banner-tv-low-image-url: string
      banner-tv-medium-image-url: string
      large-branded-banner-image-imap-script:
        default: string
        default-language:
          value: string
      large-branded-banner-image-url:
        default: string
        default-language:
          value: string
      small-branded-banner-image-imap-script:
        default: string
        default-language:
          value: string
      small-branded-banner-image-url:
        default: string
        default-language:
          value: string
      tracking-image-url: string
      watch-icon-image-url: string
    watch:
      background-color: string
      featured-playlist-id: string
      text-color: string
  content-details:
    related-playlists:
      favorites: string
      likes: string
      uploads: string
      watch-history: string
      watch-later: string
  content-owner-details:
    content-owner: string
    time-linked: string
  etag: string
  id: string
  invideo-promotion:
    default-timing:
      duration-ms: string
      offset-ms: string
      type: string
    position:
      corner-position: string
      type: string
    use-smart-timing: boolean
  kind: string
  snippet:
    country: string
    custom-url: string
    default-language: string
    description: string
    localized:
      description: string
      title: string
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
  statistics:
    comment-count: int64
    hidden-subscriber-count: boolean
    subscriber-count: int64
    video-count: int64
    view-count: int64
  status:
    is-linked: boolean
    long-uploads-status: string
    privacy-status: string
  topic-details:
    topic-categories: [string]
    topic-ids: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .audit-details    community-guidelines-good-standing=true`
    - Whether or not the channel respects the community guidelines.
* `content-id-claims-good-standing=false`
    - Whether or not the channel has any unresolved claims.
* `copyright-strikes-good-standing=false`
    - Whether or not the channel has any copyright strikes.
* `overall-good-standing=false`
    - Describes the general state of the channel. This field will always show if there are any issues whatsoever with the channel. Currently this field represents the result of the logical and operation over the community guidelines good standing, the copyright strikes good standing and the content ID claims good standing, but this may change in the future.

* `..branding-settings.channel    country=et`
    - The country of the channel.
* `default-language=sed`
    - No description provided.
* `default-tab=sit`
    - Which content tab users should see when viewing the channel.
* `description=takimata`
    - Specifies the channel description.
* `featured-channels-title=elitr`
    - Title for the featured channels tab.
* `featured-channels-urls=nonumy`
    - The list of featured channels.
    - Each invocation of this argument appends the given value to the array.
* `keywords=rebum.`
    - Lists keywords associated with the channel, comma-separated.
* `moderate-comments=true`
    - Whether user-submitted comments left on the channel page need to be approved by the channel owner to be publicly visible.
* `profile-color=lorem`
    - A prominent color that can be rendered on this channel page.
* `show-browse-view=true`
    - Whether the tab to browse the videos should be displayed.
* `show-related-channels=true`
    - Whether related channels should be proposed.
* `title=ut`
    - Specifies the channel title.
* `tracking-analytics-account-id=amet.`
    - The ID for a Google Analytics account to track and measure traffic to the channels.
* `unsubscribed-trailer=ipsum`
    - The trailer of the channel, for users that are not subscribers.

* `..image.background-image-url    default=ut`
    - No description provided.
* `default-language    value=dolor`
    - No description provided.


* `...    banner-external-url=sea`
    - This is used only in update requests; if it&#39;s set, we use this URL to generate all of the above banner URLs.
* `banner-image-url=ut`
    - Banner image. Desktop size (1060x175).
* `banner-mobile-extra-hd-image-url=eirmod`
    - Banner image. Mobile size high resolution (1440x395).
* `banner-mobile-hd-image-url=sanctus`
    - Banner image. Mobile size high resolution (1280x360).
* `banner-mobile-image-url=voluptua.`
    - Banner image. Mobile size (640x175).
* `banner-mobile-low-image-url=dolor`
    - Banner image. Mobile size low resolution (320x88).
* `banner-mobile-medium-hd-image-url=et`
    - Banner image. Mobile size medium/high resolution (960x263).
* `banner-tablet-extra-hd-image-url=et`
    - Banner image. Tablet size extra high resolution (2560x424).
* `banner-tablet-hd-image-url=vero`
    - Banner image. Tablet size high resolution (2276x377).
* `banner-tablet-image-url=ut`
    - Banner image. Tablet size (1707x283).
* `banner-tablet-low-image-url=sed`
    - Banner image. Tablet size low resolution (1138x188).
* `banner-tv-high-image-url=et`
    - Banner image. TV size high resolution (1920x1080).
* `banner-tv-image-url=ipsum`
    - Banner image. TV size extra high resolution (2120x1192).
* `banner-tv-low-image-url=justo`
    - Banner image. TV size low resolution (854x480).
* `banner-tv-medium-image-url=dolore`
    - Banner image. TV size medium resolution (1280x720).
* `large-branded-banner-image-imap-script    default=vero`
    - No description provided.
* `default-language    value=dolor`
    - No description provided.


* `...large-branded-banner-image-url    default=takimata`
    - No description provided.
* `default-language    value=et`
    - No description provided.


* `...small-branded-banner-image-imap-script    default=nonumy`
    - No description provided.
* `default-language    value=et`
    - No description provided.


* `...small-branded-banner-image-url    default=sed`
    - No description provided.
* `default-language    value=no`
    - No description provided.


* `...    tracking-image-url=invidunt`
    - The URL for a 1px by 1px tracking pixel that can be used to collect statistics for views of the channel or video pages.
* `watch-icon-image-url=rebum.`
    - The URL for the image that appears above the top-left corner of the video player. This is a 25-pixel-high image with a flexible width that cannot exceed 170 pixels.

* `..watch    background-color=labore`
    - The text color for the video watch page&#39;s branded area.
* `featured-playlist-id=aliquyam`
    - An ID that uniquely identifies a playlist that displays next to the video player.
* `text-color=elitr`
    - The background color for the video watch page&#39;s branded area.


* `...content-details.related-playlists    favorites=consetetur`
    - The ID of the playlist that contains the channel&#34;s favorite videos. Use the  playlistItems.insert and  playlistItems.delete to add or remove items from that list.
* `likes=sea`
    - The ID of the playlist that contains the channel&#34;s liked videos. Use the   playlistItems.insert and  playlistItems.delete to add or remove items from that list.
* `uploads=elitr`
    - The ID of the playlist that contains the channel&#34;s uploaded videos. Use the  videos.insert method to upload new videos and the videos.delete method to delete previously uploaded videos.
* `watch-history=at`
    - The ID of the playlist that contains the channel&#34;s watch history. Use the  playlistItems.insert and  playlistItems.delete to add or remove items from that list.
* `watch-later=sea`
    - The ID of the playlist that contains the channel&#34;s watch later playlist. Use the playlistItems.insert and  playlistItems.delete to add or remove items from that list.


* `...content-owner-details    content-owner=consetetur`
    - The ID of the content owner linked to the channel.
* `time-linked=diam`
    - The date and time of when the channel was linked to the content owner. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.

* `..    etag=accusam`
    - Etag of this resource.
* `id=dolores`
    - The ID that YouTube uses to uniquely identify the channel.
* `invideo-promotion.default-timing    duration-ms=consetetur`
    - Defines the duration in milliseconds for which the promotion should be displayed. If missing, the client should use the default.
* `offset-ms=dolor`
    - Defines the time at which the promotion will appear. Depending on the value of type the value of the offsetMs field will represent a time offset from the start or from the end of the video, expressed in milliseconds.
* `type=aliquyam`
    - Describes a timing type. If the value is offsetFromStart, then the offsetMs field represents an offset from the start of the video. If the value is offsetFromEnd, then the offsetMs field represents an offset from the end of the video.

* `..position    corner-position=elitr`
    - Describes in which corner of the video the visual widget will appear.
* `type=ea`
    - Defines the position type.

* `..    use-smart-timing=true`
    - Indicates whether the channel&#39;s promotional campaign uses &#34;smart timing.&#34; This feature attempts to show promotions at a point in the video when they are more likely to be clicked and less likely to disrupt the viewing experience. This feature also picks up a single promotion to show on each video.

* `..    kind=stet`
    - Identifies what kind of resource this is. Value: the fixed string &#34;youtube#channel&#34;.
* `snippet    country=sed`
    - The country of the channel.
* `custom-url=dolor`
    - The custom url of the channel.
* `default-language=sanctus`
    - The language of the channel&#39;s default title and description.
* `description=dolore`
    - The description of the channel.
* `localized    description=lorem`
    - The localized strings for channel&#39;s description.
* `title=consetetur`
    - The localized strings for channel&#39;s title.

* `..    published-at=consetetur`
    - The date and time that the channel was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `thumbnails.default    height=11`
    - (Optional) Height of the thumbnail image.
* `url=labore`
    - The thumbnail image&#39;s URL.
* `width=40`
    - (Optional) Width of the thumbnail image.

* `..high    height=28`
    - (Optional) Height of the thumbnail image.
* `url=sadipscing`
    - The thumbnail image&#39;s URL.
* `width=28`
    - (Optional) Width of the thumbnail image.

* `..maxres    height=69`
    - (Optional) Height of the thumbnail image.
* `url=lorem`
    - The thumbnail image&#39;s URL.
* `width=85`
    - (Optional) Width of the thumbnail image.

* `..medium    height=30`
    - (Optional) Height of the thumbnail image.
* `url=clita`
    - The thumbnail image&#39;s URL.
* `width=77`
    - (Optional) Width of the thumbnail image.

* `..standard    height=32`
    - (Optional) Height of the thumbnail image.
* `url=vero`
    - The thumbnail image&#39;s URL.
* `width=5`
    - (Optional) Width of the thumbnail image.


* `...    title=vero`
    - The channel&#39;s title.

* `..statistics    comment-count=-96`
    - The number of comments for the channel.
* `hidden-subscriber-count=true`
    - Whether or not the number of subscribers is shown for this user.
* `subscriber-count=-21`
    - The number of subscribers that the channel has.
* `video-count=-39`
    - The number of videos uploaded to the channel.
* `view-count=-61`
    - The number of times the channel has been viewed.

* `..status    is-linked=false`
    - If true, then the user is linked to either a YouTube username or G+ account. Otherwise, the user doesn&#39;t have a public YouTube identity.
* `long-uploads-status=amet.`
    - The long uploads status of this channel. See
* `privacy-status=dolore`
    - Privacy status of the channel.

* `..topic-details    topic-categories=magna`
    - A list of Wikipedia URLs that describe the channel&#39;s content.
    - Each invocation of this argument appends the given value to the array.
* `topic-ids=elitr`
    - A list of Freebase topic IDs associated with the channel. You can retrieve information about each topic using the Freebase Topic API.
    - Each invocation of this argument appends the given value to the array.



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
    - The onBehalfOfContentOwner parameter indicates that the authenticated user is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with needs to be linked to the specified YouTube content owner.

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
