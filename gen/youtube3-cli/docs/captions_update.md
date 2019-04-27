Updates a caption track. When updating a caption track, you can change the track&#39;s draft status, upload a new caption file for the track, or both.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube.force-ssl*
* *https://www.googleapis.com/auth/youtubepartner*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube.force-ssl*.
You can set the scope for this method like this: `youtube3 --scope <scope> captions update ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Caption:
  etag: string
  id: string
  kind: string
  snippet:
    audio-track-type: string
    failure-reason: string
    is-auto-synced: boolean
    is-cc: boolean
    is-draft: boolean
    is-easy-reader: boolean
    is-large: boolean
    language: string
    last-updated: string
    name: string
    status: string
    track-kind: string
    video-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    etag=dolore`
    - Etag of this resource.
* `id=nonumy`
    - The ID that YouTube uses to uniquely identify the caption track.
* `kind=sed`
    - Identifies what kind of resource this is. Value: the fixed string &#34;youtube#caption&#34;.
* `snippet    audio-track-type=aliquyam`
    - The type of audio track associated with the caption track.
* `failure-reason=sit`
    - The reason that YouTube failed to process the caption track. This property is only present if the state property&#39;s value is failed.
* `is-auto-synced=true`
    - Indicates whether YouTube synchronized the caption track to the audio track in the video. The value will be true if a sync was explicitly requested when the caption track was uploaded. For example, when calling the captions.insert or captions.update methods, you can set the sync parameter to true to instruct YouTube to sync the uploaded track to the video. If the value is false, YouTube uses the time codes in the uploaded caption track to determine when to display captions.
* `is-cc=true`
    - Indicates whether the track contains closed captions for the deaf and hard of hearing. The default value is false.
* `is-draft=false`
    - Indicates whether the caption track is a draft. If the value is true, then the track is not publicly visible. The default value is false.
* `is-easy-reader=true`
    - Indicates whether caption track is formatted for &#34;easy reader,&#34; meaning it is at a third-grade level for language learners. The default value is false.
* `is-large=true`
    - Indicates whether the caption track uses large text for the vision-impaired. The default value is false.
* `language=gubergren`
    - The language of the caption track. The property value is a BCP-47 language tag.
* `last-updated=aliquyam`
    - The date and time when the caption track was last updated. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `name=eos`
    - The name of the caption track. The name is intended to be visible to the user as an option during playback.
* `status=tempor`
    - The caption track&#39;s status.
* `track-kind=sea`
    - The caption track&#39;s type.
* `video-id=labore`
    - The ID that YouTube uses to uniquely identify the video associated with the caption track.



### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.

# Required Upload Flags

This method supports the upload of data, which *requires* all of the following flags to be set:

* **-u (simple|resumable)**
    - **simple** - Upload media all at once.
    - **resumable** - Upload media in a resumable fashion.
* **-f file**
    - Path to file to upload. It must be seekable.

The following flag *may* be set: 

* **-m mime**
    - the mime type, like 'application/octet-stream', which is the default


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

* **-p on-behalf-of=string**
    - ID of the Google+ Page for the channel that the request is be on behalf of

* **-p on-behalf-of-content-owner=string**
    - Note: This parameter is intended exclusively for YouTube content partners.
        
        The onBehalfOfContentOwner parameter indicates that the request&#39;s authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.

* **-p sync=boolean**
    - Note: The API server only processes the parameter value if the request contains an updated caption file.
        
        The sync parameter indicates whether YouTube should automatically synchronize the caption file with the audio track of the video. If you set the value to true, YouTube will automatically synchronize the caption track with the audio track.

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
