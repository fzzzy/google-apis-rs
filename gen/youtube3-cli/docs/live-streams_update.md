Updates a video stream. If the properties that you want to change cannot be updated, then you need to create a new stream with the proper settings.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.force-ssl*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube*.
You can set the scope for this method like this: `youtube3 --scope <scope> live-streams update ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
LiveStream:
  cdn:
    format: string
    frame-rate: string
    ingestion-info:
      backup-ingestion-address: string
      ingestion-address: string
      stream-name: string
    ingestion-type: string
    resolution: string
  content-details:
    closed-captions-ingestion-url: string
    is-reusable: boolean
  etag: string
  id: string
  kind: string
  snippet:
    channel-id: string
    description: string
    is-default-stream: boolean
    published-at: string
    title: string
  status:
    health-status:
      last-update-time-seconds: string
      status: string
    stream-status: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .cdn    format=sadipscing`
    - The format of the video stream that you are sending to Youtube.
* `frame-rate=ipsum`
    - The frame rate of the inbound video data.
* `ingestion-info    backup-ingestion-address=sadipscing`
    - The backup ingestion URL that you should use to stream video to YouTube. You have the option of simultaneously streaming the content that you are sending to the ingestionAddress to this URL.
* `ingestion-address=ea`
    - The primary ingestion URL that you should use to stream video to YouTube. You must stream video to this URL.
        
        Depending on which application or tool you use to encode your video stream, you may need to enter the stream URL and stream name separately or you may need to concatenate them in the following format:
        
        STREAM_URL/STREAM_NAME
* `stream-name=dolor`
    - The HTTP or RTMP stream name that YouTube assigns to the video stream.

* `..    ingestion-type=consetetur`
    - The method or protocol used to transmit the video stream.
* `resolution=sadipscing`
    - The resolution of the inbound video data.

* `..content-details    closed-captions-ingestion-url=ipsum`
    - The ingestion URL where the closed captions of this stream are sent.
* `is-reusable=true`
    - Indicates whether the stream is reusable, which means that it can be bound to multiple broadcasts. It is common for broadcasters to reuse the same stream for many different broadcasts if those broadcasts occur at different times.
        
        If you set this value to false, then the stream will not be reusable, which means that it can only be bound to one broadcast. Non-reusable streams differ from reusable streams in the following ways:  
        - A non-reusable stream can only be bound to one broadcast. 
        - A non-reusable stream might be deleted by an automated process after the broadcast ends. 
        - The  liveStreams.list method does not list non-reusable streams if you call the method and set the mine parameter to true. The only way to use that method to retrieve the resource for a non-reusable stream is to use the id parameter to identify the stream.

* `..    etag=invidunt`
    - Etag of this resource.
* `id=amet.`
    - The ID that YouTube assigns to uniquely identify the stream.
* `kind=sit`
    - Identifies what kind of resource this is. Value: the fixed string &#34;youtube#liveStream&#34;.
* `snippet    channel-id=dolor`
    - The ID that YouTube uses to uniquely identify the channel that is transmitting the stream.
* `description=et`
    - The stream&#39;s description. The value cannot be longer than 10000 characters.
* `is-default-stream=true`
    - No description provided.
* `published-at=sit`
    - The date and time that the stream was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `title=vero`
    - The stream&#39;s title. The value must be between 1 and 128 characters long.

* `..status.health-status    last-update-time-seconds=nonumy`
    - The last time this status was updated (in seconds)
* `status=accusam`
    - The status code of this stream

* `..    stream-status=est`
    - No description provided.



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

* **-p on-behalf-of-content-owner-channel=string**
    - This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
        
        The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
        
        This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.

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
