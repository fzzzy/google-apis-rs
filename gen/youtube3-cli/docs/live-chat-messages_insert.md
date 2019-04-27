Adds a message to a live chat.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/youtube*
* *https://www.googleapis.com/auth/youtube.force-ssl*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/youtube*.
You can set the scope for this method like this: `youtube3 --scope <scope> live-chat-messages insert ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
LiveChatMessage:
  author-details:
    channel-id: string
    channel-url: string
    display-name: string
    is-chat-moderator: boolean
    is-chat-owner: boolean
    is-chat-sponsor: boolean
    is-verified: boolean
    profile-image-url: string
  etag: string
  id: string
  kind: string
  snippet:
    author-channel-id: string
    display-message: string
    fan-funding-event-details:
      amount-display-string: string
      amount-micros: string
      currency: string
      user-comment: string
    has-display-content: boolean
    live-chat-id: string
    message-deleted-details:
      deleted-message-id: string
    message-retracted-details:
      retracted-message-id: string
    poll-closed-details:
      poll-id: string
    poll-edited-details:
      id: string
      prompt: string
    poll-opened-details:
      id: string
      prompt: string
    poll-voted-details:
      item-id: string
      poll-id: string
    published-at: string
    super-chat-details:
      amount-display-string: string
      amount-micros: string
      currency: string
      tier: integer
      user-comment: string
    text-message-details:
      message-text: string
    type: string
    user-banned-details:
      ban-duration-seconds: string
      ban-type: string
      banned-user-details:
        channel-id: string
        channel-url: string
        display-name: string
        profile-image-url: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .author-details    channel-id=invidunt`
    - The YouTube channel ID.
* `channel-url=duo`
    - The channel&#39;s URL.
* `display-name=dolor`
    - The channel&#39;s display name.
* `is-chat-moderator=false`
    - Whether the author is a moderator of the live chat.
* `is-chat-owner=false`
    - Whether the author is the owner of the live chat.
* `is-chat-sponsor=false`
    - Whether the author is a sponsor of the live chat.
* `is-verified=false`
    - Whether the author&#39;s identity has been verified by YouTube.
* `profile-image-url=amet`
    - The channels&#39;s avatar URL.

* `..    etag=ipsum`
    - Etag of this resource.
* `id=voluptua.`
    - The ID that YouTube assigns to uniquely identify the message.
* `kind=eirmod`
    - Identifies what kind of resource this is. Value: the fixed string &#34;youtube#liveChatMessage&#34;.
* `snippet    author-channel-id=sed`
    - The ID of the user that authored this message, this field is not always filled. textMessageEvent - the user that wrote the message fanFundingEvent - the user that funded the broadcast newSponsorEvent - the user that just became a sponsor messageDeletedEvent - the moderator that took the action messageRetractedEvent - the author that retracted their message userBannedEvent - the moderator that took the action superChatEvent - the user that made the purchase
* `display-message=accusam`
    - Contains a string that can be displayed to the user. If this field is not present the message is silent, at the moment only messages of type TOMBSTONE and CHAT_ENDED_EVENT are silent.
* `fan-funding-event-details    amount-display-string=sanctus`
    - A rendered string that displays the fund amount and currency to the user.
* `amount-micros=dolor`
    - The amount of the fund.
* `currency=dolor`
    - The currency in which the fund was made.
* `user-comment=dolore`
    - The comment added by the user to this fan funding event.

* `..    has-display-content=false`
    - Whether the message has display content that should be displayed to users.
* `live-chat-id=ipsum`
    - No description provided.
* `message-deleted-details    deleted-message-id=diam`
    - No description provided.

* `..message-retracted-details    retracted-message-id=sed`
    - No description provided.

* `..poll-closed-details    poll-id=et`
    - The id of the poll that was closed.

* `..poll-edited-details    id=tempor`
    - No description provided.
* `prompt=justo`
    - No description provided.

* `..poll-opened-details    id=takimata`
    - No description provided.
* `prompt=invidunt`
    - No description provided.

* `..poll-voted-details    item-id=amet`
    - The poll item the user chose.
* `poll-id=dolor`
    - The poll the user voted on.

* `..    published-at=voluptua.`
    - The date and time when the message was orignally published. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
* `super-chat-details    amount-display-string=et`
    - A rendered string that displays the fund amount and currency to the user.
* `amount-micros=elitr`
    - The amount purchased by the user, in micros (1,750,000 micros = 1.75).
* `currency=kasd`
    - The currency in which the purchase was made.
* `tier=53`
    - The tier in which the amount belongs to. Lower amounts belong to lower tiers. Starts at 1.
* `user-comment=justo`
    - The comment added by the user to this Super Chat event.

* `..text-message-details    message-text=gubergren`
    - The user&#39;s message.

* `..    type=ipsum`
    - The type of message, this will always be present, it determines the contents of the message as well as which fields will be present.
* `user-banned-details    ban-duration-seconds=amet.`
    - The duration of the ban. This property is only present if the banType is temporary.
* `ban-type=dolor`
    - The type of ban.
* `banned-user-details    channel-id=sanctus`
    - The YouTube channel ID.
* `channel-url=ut`
    - The channel&#39;s URL.
* `display-name=magna`
    - The channel&#39;s display name.
* `profile-image-url=amet.`
    - The channels&#39;s avatar URL.





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
