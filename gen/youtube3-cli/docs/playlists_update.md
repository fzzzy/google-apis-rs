Modifies a playlist. For example, you could change a playlist&#39;s title, description, or privacy status.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.force-ssl*
* *https://www.googleapis.com/auth/youtubepartner*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube*.
You can set the scope for this method like this: `youtube3 --scope <scope> playlists update ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Playlist:
  content-details:
    item-count: integer
  etag: string
  id: string
  kind: string
  player:
    embed-html: string
  snippet:
    channel-id: string
    channel-title: string
    default-language: string
    description: string
    localized:
      description: string
      title: string
    published-at: string
    tags: [string]
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

* `-r .content-details    item-count=24`
    - The number of videos in the playlist.

* `..    etag=sanctus`
    - Etag of this resource.
* `id=sit`
    - The ID that YouTube uses to uniquely identify the playlist.
* `kind=sit`
    - Identifies what kind of resource this is. Value: the fixed string &#34;youtube#playlist&#34;.
* `player    embed-html=magna`
    - An &lt;iframe&gt; tag that embeds a player that will play the playlist.

* `..snippet    channel-id=amet.`
    - The ID that YouTube uses to uniquely identify the channel that published the playlist.
* `channel-title=clita`
    - The channel title of the channel that the video belongs to.
* `default-language=stet`
    - The language of the playlist&#39;s default title and description.
* `description=sed`
    - The playlist&#39;s description.
* `localized    description=eirmod`
    - The localized strings for playlist&#39;s description.
* `title=elitr`
    - The localized strings for playlist&#39;s title.

* `..    published-at=gubergren`
    - The date and time that the playlist was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `tags=sit`
    - Keyword tags associated with the playlist.
    - Each invocation of this argument appends the given value to the array.
* `thumbnails.default    height=27`
    - (Optional) Height of the thumbnail image.
* `url=sea`
    - The thumbnail image&#39;s URL.
* `width=80`
    - (Optional) Width of the thumbnail image.

* `..high    height=68`
    - (Optional) Height of the thumbnail image.
* `url=at`
    - The thumbnail image&#39;s URL.
* `width=47`
    - (Optional) Width of the thumbnail image.

* `..maxres    height=62`
    - (Optional) Height of the thumbnail image.
* `url=ipsum`
    - The thumbnail image&#39;s URL.
* `width=45`
    - (Optional) Width of the thumbnail image.

* `..medium    height=34`
    - (Optional) Height of the thumbnail image.
* `url=no`
    - The thumbnail image&#39;s URL.
* `width=72`
    - (Optional) Width of the thumbnail image.

* `..standard    height=38`
    - (Optional) Height of the thumbnail image.
* `url=ipsum`
    - The thumbnail image&#39;s URL.
* `width=6`
    - (Optional) Width of the thumbnail image.


* `...    title=sea`
    - The playlist&#39;s title.

* `..status    privacy-status=eos`
    - The playlist&#39;s privacy status.



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
