Processes a natural language query and returns structured, actionable data
as a result. This method is not idempotent, because it may cause contexts
and session entity types to be updated, which in turn might affect
results of future queries.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dialogflow2 --scope <scope> projects agent-sessions-detect-intent ...`
# Required Scalar Argument
* **&lt;session&gt;** *(string)*
    - Required. The name of the session this query is sent to. Format:
        `projects/&lt;Project ID&gt;/agent/sessions/&lt;Session ID&gt;`. It&#39;s up to the API
        caller to choose an appropriate session ID. It can be a random number or
        some type of user identifier (preferably hashed). The length of the session
        ID must not exceed 36 bytes.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GoogleCloudDialogflowV2DetectIntentRequest:
  input-audio: string
  query-input:
    audio-config:
      audio-encoding: string
      language-code: string
      phrase-hints: [string]
      sample-rate-hertz: integer
    event:
      language-code: string
      name: string
    text:
      language-code: string
      text: string
  query-params:
    geo-location:
      latitude: number
      longitude: number
    reset-contexts: boolean
    time-zone: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    input-audio=et`
    - Optional. The natural language speech audio to be processed. This field
        should be populated iff `query_input` is set to an input audio config.
        A single request can contain up to 1 minute of speech audio data.
* `query-input.audio-config    audio-encoding=consetetur`
    - Required. Audio encoding of the audio content to process.
* `language-code=ut`
    - Required. The language of the supplied audio. Dialogflow does not do
        translations. See [Language
        Support](https://dialogflow.com/docs/languages) for a list of the
        currently supported language codes. Note that queries in the same session
        do not necessarily need to specify the same language.
* `phrase-hints=ea`
    - Optional. The collection of phrase hints which are used to boost accuracy
        of speech recognition.
        Refer to
        [Cloud Speech API
        documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints)
        for more details.
    - Each invocation of this argument appends the given value to the array.
* `sample-rate-hertz=21`
    - Required. Sample rate (in Hertz) of the audio content sent in the query.
        Refer to
        [Cloud Speech API
        documentation](https://cloud.google.com/speech-to-text/docs/basics) for
        more details.

* `..event    language-code=dolor`
    - Required. The language of this query. See [Language
        Support](https://dialogflow.com/docs/languages) for a list of the
        currently supported language codes. Note that queries in the same session
        do not necessarily need to specify the same language.
* `name=dolor`
    - Required. The unique identifier of the event.

* `..text    language-code=dolor`
    - Required. The language of this conversational query. See [Language
        Support](https://dialogflow.com/docs/languages) for a list of the
        currently supported language codes. Note that queries in the same session
        do not necessarily need to specify the same language.
* `text=et`
    - Required. The UTF-8 encoded natural language text to be processed.
        Text length must not exceed 256 bytes.


* `...query-params.geo-location    latitude=0.0504342224178`
    - The latitude in degrees. It must be in the range [-90.0, +90.0].
* `longitude=0.494659563181`
    - The longitude in degrees. It must be in the range [-180.0, +180.0].

* `..    reset-contexts=true`
    - Optional. Specifies whether to delete all contexts in the current session
        before the new ones are activated.
* `time-zone=lorem`
    - Optional. The time zone of this conversational query from the
        [time zone database](https://www.iana.org/time-zones), e.g.,
        America/New_York, Europe/Paris. If not provided, the time zone specified in
        agent settings is used.



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
