Adds a channelSection for the authenticated user&#39;s channel.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.force-ssl*
* *https://www.googleapis.com/auth/youtubepartner*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube*.
You can set the scope for this method like this: `youtube3 --scope <scope> channel-sections insert ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
ChannelSection:
  content-details:
    channels: [string]
    playlists: [string]
  etag: string
  id: string
  kind: string
  snippet:
    channel-id: string
    default-language: string
    localized:
      title: string
    position: integer
    style: string
    title: string
    type: string
  targeting:
    countries: [string]
    languages: [string]
    regions: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .content-details    channels=sit`
    - The channel ids for type multiple_channels.
    - Each invocation of this argument appends the given value to the array.
* `playlists=diam`
    - The playlist ids for type single_playlist and multiple_playlists. For singlePlaylist, only one playlistId is allowed.
    - Each invocation of this argument appends the given value to the array.

* `..    etag=ut`
    - Etag of this resource.
* `id=justo`
    - The ID that YouTube uses to uniquely identify the channel section.
* `kind=est`
    - Identifies what kind of resource this is. Value: the fixed string &#34;youtube#channelSection&#34;.
* `snippet    channel-id=amet`
    - The ID that YouTube uses to uniquely identify the channel that published the channel section.
* `default-language=accusam`
    - The language of the channel section&#39;s default title and description.
* `localized    title=clita`
    - The localized strings for channel section&#39;s title.

* `..    position=22`
    - The position of the channel section in the channel.
* `style=justo`
    - The style of the channel section.
* `title=est`
    - The channel section&#39;s title for multiple_playlists and multiple_channels.
* `type=clita`
    - The type of the channel section.

* `..targeting    countries=invidunt`
    - The country the channel section is targeting.
    - Each invocation of this argument appends the given value to the array.
* `languages=ut`
    - The language the channel section is targeting.
    - Each invocation of this argument appends the given value to the array.
* `regions=dolores`
    - The region the channel section is targeting.
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

* **-p on-behalf-of-content-owner-channel=string**
    - This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
        
        The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
        
        This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.

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
