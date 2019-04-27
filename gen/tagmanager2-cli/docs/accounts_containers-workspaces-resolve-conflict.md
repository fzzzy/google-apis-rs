Resolves a merge conflict for a workspace entity by updating it to the resolved entity passed in the request.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/tagmanager.edit.containers* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/tagmanager.edit.containers*.
You can set the scope for this method like this: `tagmanager2 --scope <scope> accounts containers-workspaces-resolve-conflict ...`
# Required Scalar Argument
* **&lt;path&gt;** *(string)*
    - GTM Workspace&#39;s API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Entity:
  change-status: string
  folder:
    account-id: string
    container-id: string
    fingerprint: string
    folder-id: string
    name: string
    notes: string
    path: string
    tag-manager-url: string
    workspace-id: string
  tag:
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
  trigger:
    account-id: string
    check-validation:
      key: string
      type: string
      value: string
    container-id: string
    continuous-time-min-milliseconds:
      key: string
      type: string
      value: string
    event-name:
      key: string
      type: string
      value: string
    fingerprint: string
    horizontal-scroll-percentage-list:
      key: string
      type: string
      value: string
    interval:
      key: string
      type: string
      value: string
    interval-seconds:
      key: string
      type: string
      value: string
    limit:
      key: string
      type: string
      value: string
    max-timer-length-seconds:
      key: string
      type: string
      value: string
    name: string
    notes: string
    parent-folder-id: string
    path: string
    selector:
      key: string
      type: string
      value: string
    tag-manager-url: string
    total-time-min-milliseconds:
      key: string
      type: string
      value: string
    trigger-id: string
    type: string
    unique-trigger-id:
      key: string
      type: string
      value: string
    vertical-scroll-percentage-list:
      key: string
      type: string
      value: string
    visibility-selector:
      key: string
      type: string
      value: string
    visible-percentage-max:
      key: string
      type: string
      value: string
    visible-percentage-min:
      key: string
      type: string
      value: string
    wait-for-tags:
      key: string
      type: string
      value: string
    wait-for-tags-timeout:
      key: string
      type: string
      value: string
    workspace-id: string
  variable:
    account-id: string
    container-id: string
    disabling-trigger-id: [string]
    enabling-trigger-id: [string]
    fingerprint: string
    name: string
    notes: string
    parent-folder-id: string
    path: string
    schedule-end-ms: string
    schedule-start-ms: string
    tag-manager-url: string
    type: string
    variable-id: string
    workspace-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    change-status=dolor`
    - Represents how the entity has been changed in the workspace.
* `folder    account-id=diam`
    - GTM Account ID.
* `container-id=kasd`
    - GTM Container ID.
* `fingerprint=invidunt`
    - The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified.
* `folder-id=rebum.`
    - The Folder ID uniquely identifies the GTM Folder.
* `name=lorem`
    - Folder display name.
* `notes=clita`
    - User notes on how to apply this folder in the container.
* `path=invidunt`
    - GTM Folder&#39;s API relative path.
* `tag-manager-url=eirmod`
    - Auto generated link to the tag manager UI
* `workspace-id=at`
    - GTM Workspace ID.

* `..tag    account-id=consetetur`
    - GTM Account ID.
* `blocking-rule-id=et`
    - Blocking rule IDs. If any of the listed rules evaluate to true, the tag will not fire.
    - Each invocation of this argument appends the given value to the array.
* `blocking-trigger-id=sed`
    - Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire.
    - Each invocation of this argument appends the given value to the array.
* `container-id=sit`
    - GTM Container ID.
* `fingerprint=takimata`
    - The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified.
* `firing-rule-id=elitr`
    - Firing rule IDs. A tag will fire when any of the listed rules are true and all of its blockingRuleIds (if any specified) are false.
    - Each invocation of this argument appends the given value to the array.
* `firing-trigger-id=nonumy`
    - Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false.
    - Each invocation of this argument appends the given value to the array.
* `live-only=true`
    - If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode).
* `name=lorem`
    - Tag display name.
* `notes=lorem`
    - User notes on how to apply this tag in the container.
* `parent-folder-id=diam`
    - Parent folder id.
* `path=ut`
    - GTM Tag&#39;s API relative path.
* `paused=true`
    - Indicates whether the tag is paused, which prevents the tag from firing.
* `priority    key=amet.`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=ipsum`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=ut`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    schedule-end-ms=dolor`
    - The end timestamp in milliseconds to schedule a tag.
* `schedule-start-ms=sea`
    - The start timestamp in milliseconds to schedule a tag.
* `tag-firing-option=ut`
    - Option to fire this tag.
* `tag-id=eirmod`
    - The Tag ID uniquely identifies the GTM Tag.
* `tag-manager-url=sanctus`
    - Auto generated link to the tag manager UI
* `type=voluptua.`
    - GTM Tag Type.
* `workspace-id=dolor`
    - GTM Workspace ID.

* `..trigger    account-id=et`
    - GTM Account ID.
* `check-validation    key=et`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=vero`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=ut`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    container-id=sed`
    - GTM Container ID.
* `continuous-time-min-milliseconds    key=et`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=ipsum`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=justo`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..event-name    key=dolore`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=vero`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=dolor`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    fingerprint=takimata`
    - The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified.
* `horizontal-scroll-percentage-list    key=et`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=nonumy`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=et`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..interval    key=sed`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=no`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=invidunt`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..interval-seconds    key=rebum.`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=labore`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=aliquyam`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..limit    key=elitr`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=consetetur`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=sea`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..max-timer-length-seconds    key=elitr`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=at`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=sea`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    name=consetetur`
    - Trigger display name.
* `notes=diam`
    - User notes on how to apply this trigger in the container.
* `parent-folder-id=accusam`
    - Parent folder id.
* `path=dolores`
    - GTM Trigger&#39;s API relative path.
* `selector    key=consetetur`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=dolor`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=aliquyam`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    tag-manager-url=elitr`
    - Auto generated link to the tag manager UI
* `total-time-min-milliseconds    key=ea`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=et`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=stet`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    trigger-id=sed`
    - The Trigger ID uniquely identifies the GTM Trigger.
* `type=dolor`
    - Defines the data layer event that causes this trigger.
* `unique-trigger-id    key=sanctus`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=dolore`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=lorem`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..vertical-scroll-percentage-list    key=consetetur`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=consetetur`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=eirmod`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..visibility-selector    key=labore`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=gubergren`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=et`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..visible-percentage-max    key=sadipscing`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=accusam`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=magna`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..visible-percentage-min    key=lorem`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=rebum.`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=et`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..wait-for-tags    key=clita`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=eos`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=dolores`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..wait-for-tags-timeout    key=vero`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=consetetur`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=vero`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    workspace-id=consetetur`
    - GTM Workspace ID.

* `..variable    account-id=eos`
    - GTM Account ID.
* `container-id=justo`
    - GTM Container ID.
* `disabling-trigger-id=tempor`
    - For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set.
    - Each invocation of this argument appends the given value to the array.
* `enabling-trigger-id=gubergren`
    - For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set.
    - Each invocation of this argument appends the given value to the array.
* `fingerprint=dolore`
    - The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified.
* `name=amet.`
    - Variable display name.
* `notes=dolore`
    - User notes on how to apply this variable in the container.
* `parent-folder-id=magna`
    - Parent folder id.
* `path=elitr`
    - GTM Variable&#39;s API relative path.
* `schedule-end-ms=magna`
    - The end timestamp in milliseconds to schedule a variable.
* `schedule-start-ms=ipsum`
    - The start timestamp in milliseconds to schedule a variable.
* `tag-manager-url=invidunt`
    - Auto generated link to the tag manager UI
* `type=accusam`
    - GTM Variable Type.
* `variable-id=labore`
    - The Variable ID uniquely identifies the GTM Variable.
* `workspace-id=diam`
    - GTM Workspace ID.



### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.

# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p fingerprint=string**
    - When provided, this fingerprint must match the fingerprint of the entity_in_workspace in the merge conflict.

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
