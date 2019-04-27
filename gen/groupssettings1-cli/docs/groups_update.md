Updates an existing resource.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/apps.groups.settings* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/apps.groups.settings*.
You can set the scope for this method like this: `groupssettings1 --scope <scope> groups update ...`
# Required Scalar Argument
* **&lt;group-unique-id&gt;** *(string)*
    - The resource ID
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Groups:
  allow-external-members: string
  allow-google-communication: string
  allow-web-posting: string
  archive-only: string
  custom-footer-text: string
  custom-reply-to: string
  default-message-deny-notification-text: string
  description: string
  email: string
  favorite-replies-on-top: string
  include-custom-footer: string
  include-in-global-address-list: string
  is-archived: string
  kind: string
  max-message-bytes: integer
  members-can-post-as-the-group: string
  message-display-font: string
  message-moderation-level: string
  name: string
  primary-language: string
  reply-to: string
  send-message-deny-notification: string
  show-in-group-directory: string
  spam-moderation-level: string
  who-can-add: string
  who-can-add-references: string
  who-can-assign-topics: string
  who-can-contact-owner: string
  who-can-enter-free-form-tags: string
  who-can-invite: string
  who-can-join: string
  who-can-leave-group: string
  who-can-mark-duplicate: string
  who-can-mark-favorite-reply-on-any-topic: string
  who-can-mark-favorite-reply-on-own-topic: string
  who-can-mark-no-response-needed: string
  who-can-modify-tags-and-categories: string
  who-can-post-message: string
  who-can-take-topics: string
  who-can-unassign-topic: string
  who-can-unmark-favorite-reply-on-any-topic: string
  who-can-view-group: string
  who-can-view-membership: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    allow-external-members=dolore`
    - Are external members allowed to join the group.
* `allow-google-communication=invidunt`
    - Is google allowed to contact admins.
* `allow-web-posting=aliquyam`
    - If posting from web is allowed.
* `archive-only=accusam`
    - If the group is archive only
* `custom-footer-text=lorem`
    - Custom footer text.
* `custom-reply-to=sea`
    - Default email to which reply to any message should go.
* `default-message-deny-notification-text=et`
    - Default message deny notification message
* `description=duo`
    - Description of the group
* `email=et`
    - Email id of the group
* `favorite-replies-on-top=eirmod`
    - If favorite replies should be displayed above other replies.
* `include-custom-footer=sanctus`
    - Whether to include custom footer.
* `include-in-global-address-list=et`
    - If this groups should be included in global address list or not.
* `is-archived=amet`
    - If the contents of the group are archived.
* `kind=et`
    - The type of the resource.
* `max-message-bytes=56`
    - Maximum message size allowed.
* `members-can-post-as-the-group=ut`
    - Can members post using the group email address.
* `message-display-font=ea`
    - Default message display font. Possible values are: DEFAULT_FONT FIXED_WIDTH_FONT
* `message-moderation-level=sed`
    - Moderation level for messages. Possible values are: MODERATE_ALL_MESSAGES MODERATE_NON_MEMBERS MODERATE_NEW_MEMBERS MODERATE_NONE
* `name=dolor`
    - Name of the Group
* `primary-language=dolor`
    - Primary language for the group.
* `reply-to=dolor`
    - Whome should the default reply to a message go to. Possible values are: REPLY_TO_CUSTOM REPLY_TO_SENDER REPLY_TO_LIST REPLY_TO_OWNER REPLY_TO_IGNORE REPLY_TO_MANAGERS
* `send-message-deny-notification=et`
    - Should the member be notified if his message is denied by owner.
* `show-in-group-directory=consetetur`
    - Is the group listed in groups directory
* `spam-moderation-level=amet.`
    - Moderation level for messages detected as spam. Possible values are: ALLOW MODERATE SILENTLY_MODERATE REJECT
* `who-can-add=voluptua.`
    - Permissions to add members. Possible values are: ALL_MANAGERS_CAN_ADD ALL_OWNERS_CAN_ADD ALL_MEMBERS_CAN_ADD NONE_CAN_ADD
* `who-can-add-references=lorem`
    - Permission to add references to a topic. Possible values are: NONE OWNERS_ONLY MANAGERS_ONLY OWNERS_AND_MANAGERS ALL_MEMBERS
* `who-can-assign-topics=gubergren`
    - Permission to assign topics in a forum to another user. Possible values are: NONE OWNERS_ONLY MANAGERS_ONLY OWNERS_AND_MANAGERS ALL_MEMBERS
* `who-can-contact-owner=justo`
    - Permission to contact owner of the group via web UI. Possible values are: ANYONE_CAN_CONTACT ALL_IN_DOMAIN_CAN_CONTACT ALL_MEMBERS_CAN_CONTACT ALL_MANAGERS_CAN_CONTACT
* `who-can-enter-free-form-tags=sit`
    - Permission to enter free form tags for topics in a forum. Possible values are: NONE OWNERS_ONLY MANAGERS_ONLY OWNERS_AND_MANAGERS ALL_MEMBERS
* `who-can-invite=vero`
    - Permissions to invite members. Possible values are: ALL_MEMBERS_CAN_INVITE ALL_MANAGERS_CAN_INVITE ALL_OWNERS_CAN_INVITE NONE_CAN_INVITE
* `who-can-join=diam`
    - Permissions to join the group. Possible values are: ANYONE_CAN_JOIN ALL_IN_DOMAIN_CAN_JOIN INVITED_CAN_JOIN CAN_REQUEST_TO_JOIN
* `who-can-leave-group=rebum.`
    - Permission to leave the group. Possible values are: ALL_MANAGERS_CAN_LEAVE ALL_OWNERS_CAN_LEAVE ALL_MEMBERS_CAN_LEAVE NONE_CAN_LEAVE
* `who-can-mark-duplicate=consetetur`
    - Permission to mark a topic as a duplicate of another topic. Possible values are: NONE OWNERS_ONLY MANAGERS_ONLY OWNERS_AND_MANAGERS ALL_MEMBERS
* `who-can-mark-favorite-reply-on-any-topic=sadipscing`
    - Permission to mark any other user&#39;s post as a favorite reply. Possible values are: NONE OWNERS_ONLY MANAGERS_ONLY OWNERS_AND_MANAGERS ALL_MEMBERS
* `who-can-mark-favorite-reply-on-own-topic=vero`
    - Permission to mark a post for a topic they started as a favorite reply. Possible values are: NONE OWNERS_ONLY MANAGERS_ONLY OWNERS_AND_MANAGERS ALL_MEMBERS
* `who-can-mark-no-response-needed=sadipscing`
    - Permission to mark a topic as not needing a response. Possible values are: NONE OWNERS_ONLY MANAGERS_ONLY OWNERS_AND_MANAGERS ALL_MEMBERS
* `who-can-modify-tags-and-categories=invidunt`
    - Permission to change tags and categories. Possible values are: NONE OWNERS_ONLY MANAGERS_ONLY OWNERS_AND_MANAGERS ALL_MEMBERS
* `who-can-post-message=consetetur`
    - Permissions to post messages to the group. Possible values are: NONE_CAN_POST ALL_MANAGERS_CAN_POST ALL_MEMBERS_CAN_POST ALL_OWNERS_CAN_POST ALL_IN_DOMAIN_CAN_POST ANYONE_CAN_POST
* `who-can-take-topics=dolore`
    - Permission to take topics in a forum. Possible values are: NONE OWNERS_ONLY MANAGERS_ONLY OWNERS_AND_MANAGERS ALL_MEMBERS
* `who-can-unassign-topic=duo`
    - Permission to unassign any topic in a forum. Possible values are: NONE OWNERS_ONLY MANAGERS_ONLY OWNERS_AND_MANAGERS ALL_MEMBERS
* `who-can-unmark-favorite-reply-on-any-topic=aliquyam`
    - Permission to unmark any post from a favorite reply. Possible values are: NONE OWNERS_ONLY MANAGERS_ONLY OWNERS_AND_MANAGERS ALL_MEMBERS
* `who-can-view-group=lorem`
    - Permissions to view group. Possible values are: ANYONE_CAN_VIEW ALL_IN_DOMAIN_CAN_VIEW ALL_MEMBERS_CAN_VIEW ALL_MANAGERS_CAN_VIEW ALL_OWNERS_CAN_VIEW
* `who-can-view-membership=et`
    - Permissions to view membership. Possible values are: ALL_IN_DOMAIN_CAN_VIEW ALL_MEMBERS_CAN_VIEW ALL_MANAGERS_CAN_VIEW


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
