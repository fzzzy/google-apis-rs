Processes a natural language query and returns structured, actionable data
as a result. This method is not idempotent, because it may cause contexts
and session entity types to be updated, which in turn might affect
results of future queries.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `dialogflow2-beta1 --scope <scope> projects agent-environments-users-sessions-detect-intent ...`
# Required Scalar Argument
* **&lt;session&gt;** *(string)*
    - Required. The name of the session this query is sent to. Format:
        `projects/&lt;Project ID&gt;/agent/sessions/&lt;Session ID&gt;`, or
        `projects/&lt;Project ID&gt;/agent/environments/&lt;Environment ID&gt;/users/&lt;User
        ID&gt;/sessions/&lt;Session ID&gt;`. If `Environment ID` is not specified, we assume
        default &#39;draft&#39; environment. If `User ID` is not specified, we are using
        &#34;-&#34;. It’s up to the API caller to choose an appropriate `Session ID` and
        `User Id`. They can be a random numbers or some type of user and session
        identifiers (preferably hashed). The length of the `Session ID` and
        `User ID` must not exceed 36 characters.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
GoogleCloudDialogflowV2beta1DetectIntentRequest:
  input-audio: string
  output-audio-config:
    audio-encoding: string
    sample-rate-hertz: integer
    synthesize-speech-config:
      effects-profile-id: [string]
      pitch: number
      speaking-rate: number
      voice:
        name: string
        ssml-gender: string
      volume-gain-db: number
  query-input:
    audio-config:
      audio-encoding: string
      language-code: string
      model: string
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
    knowledge-base-names: [string]
    reset-contexts: boolean
    sentiment-analysis-request-config:
      analyze-query-text-sentiment: boolean
    time-zone: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    input-audio=justo`
    - Optional. The natural language speech audio to be processed. This field
        should be populated iff `query_input` is set to an input audio config.
        A single request can contain up to 1 minute of speech audio data.
* `output-audio-config    audio-encoding=justo`
    - Required. Audio encoding of the synthesized audio content.
* `sample-rate-hertz=67`
    - Optional. The synthesis sample rate (in hertz) for this audio. If not
        provided, then the synthesizer will use the default sample rate based on
        the audio encoding. If this is different from the voice&#39;s natural sample
        rate, then the synthesizer will honor this request by converting to the
        desired sample rate (which might result in worse audio quality).
* `synthesize-speech-config    effects-profile-id=et`
    - Optional. An identifier which selects &#39;audio effects&#39; profiles that are
        applied on (post synthesized) text to speech. Effects are applied on top of
        each other in the order they are given.
    - Each invocation of this argument appends the given value to the array.
* `pitch=0.594456807662`
    - Optional. Speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20
        semitones from the original pitch. -20 means decrease 20 semitones from the
        original pitch.
* `speaking-rate=0.460933679688`
    - Optional. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal
        native speed supported by the specific voice. 2.0 is twice as fast, and
        0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any
        other values &lt; 0.25 or &gt; 4.0 will return an error.
* `voice    name=lorem`
    - Optional. The name of the voice. If not set, the service will choose a
        voice based on the other parameters such as language_code and gender.
* `ssml-gender=et`
    - Optional. The preferred gender of the voice. If not set, the service will
        choose a voice based on the other parameters such as language_code and
        name. Note that this is only a preference, not requirement. If a
        voice of the appropriate gender is not available, the synthesizer should
        substitute a voice with a different gender rather than failing the request.

* `..    volume-gain-db=0.313727897996`
    - Optional. Volume gain (in dB) of the normal native volume supported by the
        specific voice, in the range [-96.0, 16.0]. If unset, or set to a value of
        0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB)
        will play at approximately half the amplitude of the normal native signal
        amplitude. A value of +6.0 (dB) will play at approximately twice the
        amplitude of the normal native signal amplitude. We strongly recommend not
        to exceed +10 (dB) as there&#39;s usually no effective increase in loudness for
        any value greater than that.


* `...query-input.audio-config    audio-encoding=aliquyam`
    - Required. Audio encoding of the audio content to process.
* `language-code=sea`
    - Required. The language of the supplied audio. Dialogflow does not do
        translations. See [Language
        Support](https://dialogflow.com/docs/languages) for a list of the
        currently supported language codes. Note that queries in the same session
        do not necessarily need to specify the same language.
* `model=lorem`
    - Optional. Which Speech model to select for the given request. Select the
        model best suited to your domain to get best results. If a model is not
        explicitly specified, then we auto-select a model based on the parameters
        in the InputAudioConfig.
        If enhanced speech model is enabled for the agent and an enhanced
        version of the specified model for the language does not exist, then the
        speech is recognized using the standard version of the specified model.
        Refer to
        [Cloud Speech API
        documentation](https://cloud.google.com/speech-to-text/docs/basics#select-model)
        for more details.
* `phrase-hints=eos`
    - Optional. The collection of phrase hints which are used to boost accuracy
        of speech recognition.
        Refer to
        [Cloud Speech API
        documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints)
        for more details.
    - Each invocation of this argument appends the given value to the array.
* `sample-rate-hertz=20`
    - Required. Sample rate (in Hertz) of the audio content sent in the query.
        Refer to
        [Cloud Speech API
        documentation](https://cloud.google.com/speech-to-text/docs/basics) for
        more details.

* `..event    language-code=sadipscing`
    - Required. The language of this query. See [Language
        Support](https://dialogflow.com/docs/languages) for a list of the
        currently supported language codes. Note that queries in the same session
        do not necessarily need to specify the same language.
* `name=dolor`
    - Required. The unique identifier of the event.

* `..text    language-code=eirmod`
    - Required. The language of this conversational query. See [Language
        Support](https://dialogflow.com/docs/languages) for a list of the
        currently supported language codes. Note that queries in the same session
        do not necessarily need to specify the same language.
* `text=elitr`
    - Required. The UTF-8 encoded natural language text to be processed.
        Text length must not exceed 256 bytes.


* `...query-params.geo-location    latitude=0.0438804035801`
    - The latitude in degrees. It must be in the range [-90.0, +90.0].
* `longitude=0.407470002491`
    - The longitude in degrees. It must be in the range [-180.0, +180.0].

* `..    knowledge-base-names=labore`
    - Optional. KnowledgeBases to get alternative results from. If not set, the
        KnowledgeBases enabled in the agent (through UI) will be used.
        Format:  `projects/&lt;Project ID&gt;/knowledgeBases/&lt;Knowledge Base ID&gt;`.
    - Each invocation of this argument appends the given value to the array.
* `reset-contexts=true`
    - Optional. Specifies whether to delete all contexts in the current session
        before the new ones are activated.
* `sentiment-analysis-request-config    analyze-query-text-sentiment=true`
    - Optional. Instructs the service to perform sentiment analysis on
        `query_text`. If not provided, sentiment analysis is not performed on
        `query_text`.

* `..    time-zone=invidunt`
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
