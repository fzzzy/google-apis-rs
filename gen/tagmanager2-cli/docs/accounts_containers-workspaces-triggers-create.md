Creates a GTM Trigger.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/tagmanager.edit.containers* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/tagmanager.edit.containers*.
You can set the scope for this method like this: `tagmanager2 --scope <scope> accounts containers-workspaces-triggers-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - GTM Workspaces&#39;s API relative path. Example: accounts/{account_id}/containers/{container_id}/workspaces/{workspace_id}
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Trigger:
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

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=tempor`
    - GTM Account ID.
* `check-validation    key=et`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=erat`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=dolores`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    container-id=kasd`
    - GTM Container ID.
* `continuous-time-min-milliseconds    key=et`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=clita`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=sed`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..event-name    key=dolores`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=clita`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=eos`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    fingerprint=amet`
    - The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified.
* `horizontal-scroll-percentage-list    key=sed`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=takimata`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=sit`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..interval    key=labore`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=nonumy`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=erat`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..interval-seconds    key=gubergren`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=erat`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=et`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..limit    key=amet`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=lorem`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=voluptua.`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..max-timer-length-seconds    key=rebum.`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=justo`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=labore`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    name=voluptua.`
    - Trigger display name.
* `notes=takimata`
    - User notes on how to apply this trigger in the container.
* `parent-folder-id=dolor`
    - Parent folder id.
* `path=takimata`
    - GTM Trigger&#39;s API relative path.
* `selector    key=voluptua.`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=no`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=aliquyam`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    tag-manager-url=magna`
    - Auto generated link to the tag manager UI
* `total-time-min-milliseconds    key=et`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=sed`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=est`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    trigger-id=sit`
    - The Trigger ID uniquely identifies the GTM Trigger.
* `type=et`
    - Defines the data layer event that causes this trigger.
* `unique-trigger-id    key=consetetur`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=sea`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=nonumy`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..vertical-scroll-percentage-list    key=consetetur`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=accusam`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=clita`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..visibility-selector    key=sea`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=vero`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=dolores`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..visible-percentage-max    key=magna`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=ut`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=amet`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..visible-percentage-min    key=sed`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=sit`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=sit`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..wait-for-tags    key=dolores`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=et`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=sanctus`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..wait-for-tags-timeout    key=takimata`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=kasd`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=ut`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    workspace-id=sadipscing`
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
