Creates a GTM Trigger.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/tagmanager.edit.containers* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/tagmanager.edit.containers*.
You can set the scope for this method like this: `tagmanager1 --scope <scope> accounts containers-triggers-create ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - The GTM Account ID.
* **&lt;container-id&gt;** *(string)*
    - The GTM Container ID.
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
  parent-folder-id: string
  selector:
    key: string
    type: string
    value: string
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

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=labore`
    - GTM Account ID.
* `check-validation    key=ipsum`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=aliquyam`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=dolores`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    container-id=sit`
    - GTM Container ID.
* `continuous-time-min-milliseconds    key=diam`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=ut`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=justo`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..event-name    key=est`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=amet`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=accusam`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    fingerprint=clita`
    - The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified.
* `horizontal-scroll-percentage-list    key=diam`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=justo`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=est`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..interval    key=clita`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=invidunt`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=ut`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..interval-seconds    key=dolores`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=eos`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=voluptua.`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..limit    key=duo`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=sed`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=aliquyam`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..max-timer-length-seconds    key=ea`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=ea`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=et`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    name=dolor`
    - Trigger display name.
* `parent-folder-id=diam`
    - Parent folder id.
* `selector    key=kasd`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=invidunt`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=rebum.`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..total-time-min-milliseconds    key=lorem`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=clita`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=invidunt`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..    trigger-id=eirmod`
    - The Trigger ID uniquely identifies the GTM Trigger.
* `type=at`
    - Defines the data layer event that causes this trigger.
* `unique-trigger-id    key=consetetur`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=et`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=sed`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..vertical-scroll-percentage-list    key=sit`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=takimata`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=elitr`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..visibility-selector    key=nonumy`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=rebum.`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=lorem`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..visible-percentage-max    key=lorem`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=diam`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=ut`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..visible-percentage-min    key=ut`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=amet.`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=ipsum`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..wait-for-tags    key=ut`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=dolor`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=sea`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.

* `..wait-for-tags-timeout    key=ut`
    - The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values.
* `type=eirmod`
    - The parameter type. Valid values are: 
        - boolean: The value represents a boolean, represented as &#39;true&#39; or &#39;false&#39; 
        - integer: The value represents a 64-bit signed integer value, in base 10 
        - list: A list of parameters should be specified 
        - map: A map of parameters should be specified 
        - template: The value represents any text; this can include variable references (even variable references that might return non-string types)
* `value=sanctus`
    - A parameter&#39;s value (may contain variable references such as &#34;{{myVariable}}&#34;) as appropriate to the specified type.



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
