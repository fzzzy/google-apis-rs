Creates a GTM Tag.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/tagmanager.edit.containers* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/tagmanager.edit.containers*.
You can set the scope for this method like this: `tagmanager2 --scope <scope> accounts containers-workspaces-tags-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - GTM Workspace&#39;s API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Tag:
  account-id: string
  blocking-rule-id: [string]
  blocking-trigger-id: [string]
  container-id: string
  fingerprint: string
  firing-rule-id: [string]
  firing-trigger-id: [string]
  live-only: boolean
  name: string
  notes: string
  parent-folder-id: string
  path: string
  paused: boolean
  priority:
    key: string
    type: string
    value: string
  schedule-end-ms: string
  schedule-start-ms: string
  tag-firing-option: string
  tag-id: string
  tag-manager-url: string
  type: string
  workspace-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=nonumy`
    - GTM Account ID.
* `blocking-rule-id=sed`
    - Blocking rule IDs. If any of the listed rules evaluate to true, the tag will not fire.
    - Each invocation of this argument appends the given value to the array.
* `blocking-trigger-id=diam`
    - Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire.
    - Each invocation of this argument appends the given value to the array.
* `container-id=magna`
    - GTM Container ID.
* `fingerprint=dolor`
    - The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified.
* `firing-rule-id=lorem`
    - Firing rule IDs. A tag will fire when any of the listed rules are true and all of its blockingRuleIds (if any specified) are false.
    - Each invocation of this argument appends the given value to the array.
* `firing-trigger-id=dolor`
    - Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false.
    - Each invocation of this argument appends the given value to the array.
* `live-only=true`
    - If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode).
* `name=nonumy`
    - Tag display name.
* `notes=takimata`
    - User notes on how to apply this tag in the container.
* `parent-folder-id=dolores`
    - Parent folder id.
* `path=consetetur`
    - GTM Tag&#39;s API relative path.
* `paused=true`
    - Indicates whether the tag is paused, which prevents the tag from firing.
* `priority    key=amet.`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=dolores`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=dolores`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    schedule-end-ms=et`
    - The end timestamp in milliseconds to schedule a tag.
* `schedule-start-ms=sed`
    - The start timestamp in milliseconds to schedule a tag.
* `tag-firing-option=et`
    - Option to fire this tag.
* `tag-id=aliquyam`
    - The Tag ID uniquely identifies the GTM Tag.
* `tag-manager-url=nonumy`
    - Auto generated link to the tag manager UI
* `type=sit`
    - GTM Tag Type.
* `workspace-id=aliquyam`
    - GTM Workspace ID.


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
