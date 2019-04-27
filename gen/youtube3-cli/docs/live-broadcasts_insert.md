Creates a broadcast.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.force-ssl*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube*.
You can set the scope for this method like this: `youtube3 --scope <scope> live-broadcasts insert ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
LiveBroadcast:
  content-details:
    bound-stream-id: string
    bound-stream-last-update-time-ms: string
    closed-captions-type: string
    enable-auto-start: boolean
    enable-closed-captions: boolean
    enable-content-encryption: boolean
    enable-dvr: boolean
    enable-embed: boolean
    enable-low-latency: boolean
    latency-preference: string
    mesh: string
    monitor-stream:
      broadcast-stream-delay-ms: integer
      embed-html: string
      enable-monitor-stream: boolean
    projection: string
    record-from-start: boolean
    start-with-slate: boolean
    stereo-layout: string
  etag: string
  id: string
  kind: string
  snippet:
    actual-end-time: string
    actual-start-time: string
    channel-id: string
    description: string
    is-default-broadcast: boolean
    live-chat-id: string
    published-at: string
    scheduled-end-time: string
    scheduled-start-time: string
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
    concurrent-viewers: string
    total-chat-count: int64
  status:
    life-cycle-status: string
    live-broadcast-priority: string
    privacy-status: string
    recording-status: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .content-details    bound-stream-id=no`
    - This value uniquely identifies the live stream bound to the broadcast.
* `bound-stream-last-update-time-ms=aliquyam`
    - The date and time that the live stream referenced by boundStreamId was last updated.
* `closed-captions-type=magna`
    - No description provided.
* `enable-auto-start=true`
    - This setting indicates whether auto start is enabled for this broadcast.
* `enable-closed-captions=true`
    - This setting indicates whether HTTP POST closed captioning is enabled for this broadcast. The ingestion URL of the closed captions is returned through the liveStreams API. This is mutually exclusive with using the closed_captions_type property, and is equivalent to setting closed_captions_type to CLOSED_CAPTIONS_HTTP_POST.
* `enable-content-encryption=true`
    - This setting indicates whether YouTube should enable content encryption for the broadcast.
* `enable-dvr=true`
    - This setting determines whether viewers can access DVR controls while watching the video. DVR controls enable the viewer to control the video playback experience by pausing, rewinding, or fast forwarding content. The default value for this property is true.
        
        
        
        Important: You must set the value to true and also set the enableArchive property&#39;s value to true if you want to make playback available immediately after the broadcast ends.
* `enable-embed=true`
    - This setting indicates whether the broadcast video can be played in an embedded player. If you choose to archive the video (using the enableArchive property), this setting will also apply to the archived video.
* `enable-low-latency=false`
    - Indicates whether this broadcast has low latency enabled.
* `latency-preference=sea`
    - If both this and enable_low_latency are set, they must match. LATENCY_NORMAL should match enable_low_latency=false LATENCY_LOW should match enable_low_latency=true LATENCY_ULTRA_LOW should have enable_low_latency omitted.
* `mesh=nonumy`
    - No description provided.
* `monitor-stream    broadcast-stream-delay-ms=56`
    - If you have set the enableMonitorStream property to true, then this property determines the length of the live broadcast delay.
* `embed-html=accusam`
    - HTML code that embeds a player that plays the monitor stream.
* `enable-monitor-stream=true`
    - This value determines whether the monitor stream is enabled for the broadcast. If the monitor stream is enabled, then YouTube will broadcast the event content on a special stream intended only for the broadcaster&#39;s consumption. The broadcaster can use the stream to review the event content and also to identify the optimal times to insert cuepoints.
        
        You need to set this value to true if you intend to have a broadcast delay for your event.
        
        Note: This property cannot be updated once the broadcast is in the testing or live state.

* `..    projection=sea`
    - The projection format of this broadcast. This defaults to rectangular.
* `record-from-start=false`
    - Automatically start recording after the event goes live. The default value for this property is true.
        
        
        
        Important: You must also set the enableDvr property&#39;s value to true if you want the playback to be available immediately after the broadcast ends. If you set this property&#39;s value to true but do not also set the enableDvr property to true, there may be a delay of around one day before the archived video will be available for playback.
* `start-with-slate=false`
    - This setting indicates whether the broadcast should automatically begin with an in-stream slate when you update the broadcast&#39;s status to live. After updating the status, you then need to send a liveCuepoints.insert request that sets the cuepoint&#39;s eventState to end to remove the in-stream slate and make your broadcast stream visible to viewers.
* `stereo-layout=magna`
    - No description provided.

* `..    etag=ut`
    - Etag of this resource.
* `id=amet`
    - The ID that YouTube assigns to uniquely identify the broadcast.
* `kind=sed`
    - Identifies what kind of resource this is. Value: the fixed string &#34;youtube#liveBroadcast&#34;.
* `snippet    actual-end-time=sit`
    - The date and time that the broadcast actually ended. This information is only available once the broadcast&#39;s state is complete. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `actual-start-time=sit`
    - The date and time that the broadcast actually started. This information is only available once the broadcast&#39;s state is live. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `channel-id=dolores`
    - The ID that YouTube uses to uniquely identify the channel that is publishing the broadcast.
* `description=et`
    - The broadcast&#39;s description. As with the title, you can set this field by modifying the broadcast resource or by setting the description field of the corresponding video resource.
* `is-default-broadcast=true`
    - No description provided.
* `live-chat-id=takimata`
    - The id of the live chat for this broadcast.
* `published-at=kasd`
    - The date and time that the broadcast was added to YouTube&#39;s live broadcast schedule. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `scheduled-end-time=ut`
    - The date and time that the broadcast is scheduled to end. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `scheduled-start-time=sadipscing`
    - The date and time that the broadcast is scheduled to start. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `thumbnails.default    height=17`
    - (Optional) Height of the thumbnail image.
* `url=clita`
    - The thumbnail image&#39;s URL.
* `width=52`
    - (Optional) Width of the thumbnail image.

* `..high    height=2`
    - (Optional) Height of the thumbnail image.
* `url=elitr`
    - The thumbnail image&#39;s URL.
* `width=19`
    - (Optional) Width of the thumbnail image.

* `..maxres    height=70`
    - (Optional) Height of the thumbnail image.
* `url=kasd`
    - The thumbnail image&#39;s URL.
* `width=31`
    - (Optional) Width of the thumbnail image.

* `..medium    height=80`
    - (Optional) Height of the thumbnail image.
* `url=sit`
    - The thumbnail image&#39;s URL.
* `width=61`
    - (Optional) Width of the thumbnail image.

* `..standard    height=41`
    - (Optional) Height of the thumbnail image.
* `url=lorem`
    - The thumbnail image&#39;s URL.
* `width=72`
    - (Optional) Width of the thumbnail image.


* `...    title=tempor`
    - The broadcast&#39;s title. Note that the broadcast represents exactly one YouTube video. You can set this field by modifying the broadcast resource or by setting the title field of the corresponding video resource.

* `..statistics    concurrent-viewers=clita`
    - The number of viewers currently watching the broadcast. The property and its value will be present if the broadcast has current viewers and the broadcast owner has not hidden the viewcount for the video. Note that YouTube stops tracking the number of concurrent viewers for a broadcast when the broadcast ends. So, this property would not identify the number of viewers watching an archived video of a live broadcast that already ended.
* `total-chat-count=-62`
    - The total number of live chat messages currently on the broadcast. The property and its value will be present if the broadcast is public, has the live chat feature enabled, and has at least one message. Note that this field will not be filled after the broadcast ends. So this property would not identify the number of chat messages for an archived video of a completed live broadcast.

* `..status    life-cycle-status=elitr`
    - The broadcast&#39;s status. The status can be updated using the API&#39;s liveBroadcasts.transition method.
* `live-broadcast-priority=et`
    - Priority of the live broadcast event (internal state).
* `privacy-status=vero`
    - The broadcast&#39;s privacy status. Note that the broadcast represents exactly one YouTube video, so the privacy settings are identical to those supported for videos. In addition, you can set this field by modifying the broadcast resource or by setting the privacyStatus field of the corresponding video resource.
* `recording-status=sadipscing`
    - The broadcast&#39;s recording status.



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
