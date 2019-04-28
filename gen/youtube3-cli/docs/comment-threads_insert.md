Creates a new top-level comment. To add a reply to an existing comment, use the comments.insert method instead.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/youtube.force-ssl* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube.force-ssl*.
You can set the scope for this method like this: `youtube3 --scope <scope> comment-threads insert ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CommentThread:
  etag: string
  id: string
  kind: string
  snippet:
    can-reply: boolean
    channel-id: string
    is-public: boolean
    top-level-comment:
      etag: string
      id: string
      kind: string
      snippet:
        author-channel-url: string
        author-display-name: string
        author-profile-image-url: string
        can-rate: boolean
        channel-id: string
        like-count: integer
        moderation-status: string
        parent-id: string
        published-at: string
        text-display: string
        text-original: string
        updated-at: string
        video-id: string
        viewer-rating: string
    total-reply-count: integer
    video-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    etag=magna`
    - Etag of this resource.
* `id=ipsum`
    - The ID that YouTube uses to uniquely identify the comment thread.
* `kind=invidunt`
    - Identifies what kind of resource this is. Value: the fixed string &#34;youtube#commentThread&#34;.
* `snippet    can-reply=true`
    - Whether the current viewer of the thread can reply to it. This is viewer specific - other viewers may see a different value for this field.
* `channel-id=labore`
    - The YouTube channel the comments in the thread refer to or the channel with the video the comments refer to. If video_id isn&#39;t set the comments refer to the channel itself.
* `is-public=false`
    - Whether the thread (and therefore all its comments) is visible to all YouTube users.
* `top-level-comment    etag=nonumy`
    - Etag of this resource.
* `id=sed`
    - The ID that YouTube uses to uniquely identify the comment.
* `kind=diam`
    - Identifies what kind of resource this is. Value: the fixed string &#34;youtube#comment&#34;.
* `snippet    author-channel-url=magna`
    - Link to the author&#39;s YouTube channel, if any.
* `author-display-name=dolor`
    - The name of the user who posted the comment.
* `author-profile-image-url=lorem`
    - The URL for the avatar of the user who posted the comment.
* `can-rate=true`
    - Whether the current viewer can rate this comment.
* `channel-id=vero`
    - The id of the corresponding YouTube channel. In case of a channel comment this is the channel the comment refers to. In case of a video comment it&#39;s the video&#39;s channel.
* `like-count=10`
    - The total number of likes this comment has received.
* `moderation-status=takimata`
    - The comment&#39;s moderation status. Will not be set if the comments were requested through the id filter.
* `parent-id=dolores`
    - The unique id of the parent comment, only set for replies.
* `published-at=consetetur`
    - The date and time when the comment was orignally published. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `text-display=erat`
    - The comment&#39;s text. The format is either plain text or HTML dependent on what has been requested. Even the plain text representation may differ from the text originally posted in that it may replace video links with video titles etc.
* `text-original=amet.`
    - The comment&#39;s original raw text as initially posted or last updated. The original text will only be returned if it is accessible to the viewer, which is only guaranteed if the viewer is the comment&#39;s author.
* `updated-at=dolores`
    - The date and time when was last updated . The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `video-id=dolores`
    - The ID of the video the comment refers to, if any.
* `viewer-rating=et`
    - The rating the viewer has given to this comment. For the time being this will never return RATE_TYPE_DISLIKE and instead return RATE_TYPE_NONE. This may change in the future.


* `...    total-reply-count=59`
    - The total number of replies (not including the top level comment).
* `video-id=et`
    - The ID of the video the comments refer to, if any. No video_id implies a channel discussion comment.



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