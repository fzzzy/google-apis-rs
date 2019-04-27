Updates the specified intent.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dialogflow2 --scope <scope> projects agent-intents-patch ...`
# Required Scalar Argument
* **&lt;name&gt;** *(string)*
    - Required for all methods except `create` (`create` populates the name
        automatically.
        The unique identifier of this intent.
        Format: `projects/&lt;Project ID&gt;/agent/intents/&lt;Intent ID&gt;`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GoogleCloudDialogflowV2Intent:
  action: string
  default-response-platforms: [string]
  display-name: string
  events: [string]
  input-context-names: [string]
  is-fallback: boolean
  ml-disabled: boolean
  name: string
  parent-followup-intent-name: string
  priority: integer
  reset-contexts: boolean
  root-followup-intent-name: string
  webhook-state: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    action=eirmod`
    - Optional. The name of the action associated with the intent.
        Note: The action name must not contain whitespaces.
* `default-response-platforms=elitr`
    - Optional. The list of platforms for which the first response will be
        taken from among the messages assigned to the DEFAULT_PLATFORM.
    - Each invocation of this argument appends the given value to the array.
* `display-name=amet`
    - Required. The name of this intent.
* `events=no`
    - Optional. The collection of event names that trigger the intent.
        If the collection of input contexts is not empty, all of the contexts must
        be present in the active user session for an event to trigger this intent.
    - Each invocation of this argument appends the given value to the array.
* `input-context-names=labore`
    - Optional. The list of context names required for this intent to be
        triggered.
        Format: `projects/&lt;Project ID&gt;/agent/sessions/-/contexts/&lt;Context ID&gt;`.
    - Each invocation of this argument appends the given value to the array.
* `is-fallback=true`
    - Optional. Indicates whether this is a fallback intent.
* `ml-disabled=true`
    - Optional. Indicates whether Machine Learning is disabled for the intent.
        Note: If `ml_diabled` setting is set to true, then this intent is not
        taken into account during inference in `ML ONLY` match mode. Also,
        auto-markup in the UI is turned off.
* `name=invidunt`
    - Required for all methods except `create` (`create` populates the name
        automatically.
        The unique identifier of this intent.
        Format: `projects/&lt;Project ID&gt;/agent/intents/&lt;Intent ID&gt;`.
* `parent-followup-intent-name=aliquyam`
    - Read-only after creation. The unique identifier of the parent intent in the
        chain of followup intents. You can set this field when creating an intent,
        for example with CreateIntent or BatchUpdateIntents, in order to
        make this intent a followup intent.
        
        It identifies the parent followup intent.
        Format: `projects/&lt;Project ID&gt;/agent/intents/&lt;Intent ID&gt;`.
* `priority=28`
    - Optional. The priority of this intent. Higher numbers represent higher
        priorities. Zero or negative numbers mean that the intent is disabled.
* `reset-contexts=false`
    - Optional. Indicates whether to delete all contexts in the current
        session when this intent is matched.
* `root-followup-intent-name=sea`
    - Read-only. The unique identifier of the root intent in the chain of
        followup intents. It identifies the correct followup intents chain for
        this intent. We populate this field only in the output.
        
        Format: `projects/&lt;Project ID&gt;/agent/intents/&lt;Intent ID&gt;`.
* `webhook-state=et`
    - Optional. Indicates whether webhooks are enabled for the intent.


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

* **-p language-code=string**
    - Optional. The language of training phrases, parameters and rich messages
        defined in `intent`. If not specified, the agent&#39;s default language is
        used. [More than a dozen
        languages](https://dialogflow.com/docs/reference/language) are supported.
        Note: languages must be enabled in the agent, before they can be used.

* **-p update-mask=string**
    - Optional. The mask to control which fields get updated.

* **-p intent-view=string**
    - Optional. The resource view to apply to the returned intent.

# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p $-xgafv=string**
    - V1 error format.

* **-p access-token=string**
    - OAuth access token.

* **-p alt=string**
    - Data format for response.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).
