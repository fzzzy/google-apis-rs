Creates a conversation profile in the specified project.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dialogflow2-beta1 --scope <scope> projects conversation-profiles-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The project to create a conversation profile for.
        Format: `projects/&lt;Project ID&gt;`.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GoogleCloudDialogflowV2beta1ConversationProfile:
  automated-agent-config:
    agent: string
  display-name: string
  human-agent-assistant-config:
    name: string
    notification-config:
      topic: string
  name: string
  notification-config:
    topic: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .automated-agent-config    agent=clita`
    - Required. ID of the Dialogflow agent environment to use.
        
        This project needs to either be the same project as the conversation or you
        need to grant `service-&lt;Conversation Project
        Number&gt;@gcp-sa-dialogflow.iam.gserviceaccount.com` the `Dialogflow API
        Service Agent` role in this project.
        
        Format: `projects/&lt;Project ID&gt;/agent/environments/&lt;Environment ID or &#39;-&#39;&gt;`
        If environment is not specified, the default `draft` environment is
        used. Refer to
        [DetectIntentRequest](/dialogflow-enterprise/docs/reference/rpc/google.cloud.dialogflow.v2beta1#google.cloud.dialogflow.v2beta1.DetectIntentRequest)
        for more details.

* `..    display-name=invidunt`
    - Required. Human readable name for this profile. Max length 1024 bytes.
* `human-agent-assistant-config    name=ut`
    - Required. ID of the agent assistant to use.
        Format: `projects/&lt;Project ID&gt;/humanAgentAssistants/&lt;Human Agent Assistant
        ID&gt;`.
* `notification-config    topic=dolores`
    - Optional. Name of the Cloud Pub/Sub topic to publish conversation
        events like
        CONVERSATION_STARTED as
        serialized ConversationEvent protos.
        
        If enable_notifications is
        `true` and no topic is supplied, a new topic is created and listed
        here.
        
        Notification works for phone calls, if this topic either is in the same
        project as the conversation or you grant `service-&lt;Conversation Project
        Number&gt;@gcp-sa-dialogflow.iam.gserviceaccount.com` the `Dialogflow Service
        Agent` role in the topic project.
        
        Format: `projects/&lt;Project ID&gt;/topics/&lt;Topic ID&gt;`.


* `...    name=eos`
    - Required for all methods except `create` (`create` populates the name
        automatically).
        The unique identifier of this conversation profile.
        Format: `projects/&lt;Project ID&gt;/conversationProfiles/&lt;Conversation Profile
        ID&gt;`.
* `notification-config    topic=voluptua.`
    - Optional. Name of the Cloud Pub/Sub topic to publish conversation
        events like
        CONVERSATION_STARTED as
        serialized ConversationEvent protos.
        
        If enable_notifications is
        `true` and no topic is supplied, a new topic is created and listed
        here.
        
        Notification works for phone calls, if this topic either is in the same
        project as the conversation or you grant `service-&lt;Conversation Project
        Number&gt;@gcp-sa-dialogflow.iam.gserviceaccount.com` the `Dialogflow Service
        Agent` role in the topic project.
        
        Format: `projects/&lt;Project ID&gt;/topics/&lt;Topic ID&gt;`.



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
