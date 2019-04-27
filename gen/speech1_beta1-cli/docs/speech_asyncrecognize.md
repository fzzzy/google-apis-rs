Performs asynchronous speech recognition: receive results via the
[google.longrunning.Operations]
(/speech/reference/rest/v1beta1/operations#Operation)
interface. Returns either an
`Operation.error` or an `Operation.response` which contains
an `AsyncRecognizeResponse` message.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `speech1-beta1 --scope <scope> speech asyncrecognize ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
AsyncRecognizeRequest:
  audio:
    content: string
    uri: string
  config:
    encoding: string
    language-code: string
    max-alternatives: integer
    profanity-filter: boolean
    sample-rate: integer
    speech-context:
      phrases: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .audio    content=eirmod`
    - The audio data bytes encoded as specified in
        `RecognitionConfig`. Note: as with all bytes fields, protobuffers use a
        pure binary representation, whereas JSON representations use base64.
* `uri=sit`
    - URI that points to a file that contains audio data bytes as specified in
        `RecognitionConfig`. Currently, only Google Cloud Storage URIs are
        supported, which must be specified in the following format:
        `gs://bucket_name/object_name` (other URI formats return
        google.rpc.Code.INVALID_ARGUMENT). For more information, see
        [Request URIs](https://cloud.google.com/storage/docs/reference-uris).

* `..config    encoding=stet`
    - *Required* Encoding of audio data sent in all `RecognitionAudio` messages.
* `language-code=sed`
    - *Optional* The language of the supplied audio as a BCP-47 language tag.
        Example: &#34;en-GB&#34;  https://www.rfc-editor.org/rfc/bcp/bcp47.txt
        If omitted, defaults to &#34;en-US&#34;. See
        [Language Support](https://cloud.google.com/speech/docs/languages)
        for a list of the currently supported language codes.
* `max-alternatives=16`
    - *Optional* Maximum number of recognition hypotheses to be returned.
        Specifically, the maximum number of `SpeechRecognitionAlternative` messages
        within each `SpeechRecognitionResult`.
        The server may return fewer than `max_alternatives`.
        Valid values are `0`-`30`. A value of `0` or `1` will return a maximum of
        one. If omitted, will return a maximum of one.
* `profanity-filter=true`
    - *Optional* If set to `true`, the server will attempt to filter out
        profanities, replacing all but the initial character in each filtered word
        with asterisks, e.g. &#34;f***&#34;. If set to `false` or omitted, profanities
        won&#39;t be filtered out.
* `sample-rate=38`
    - *Required* Sample rate in Hertz of the audio data sent in all
        `RecognitionAudio` messages. Valid values are: 8000-48000.
        16000 is optimal. For best results, set the sampling rate of the audio
        source to 16000 Hz. If that&#39;s not possible, use the native sample rate of
        the audio source (instead of re-sampling).
* `speech-context    phrases=accusam`
    - *Optional* A list of strings containing words and phrases &#34;hints&#34; so that
        the speech recognition is more likely to recognize them. This can be used
        to improve the accuracy for specific words and phrases, for example, if
        specific commands are typically spoken by the user. This can also be used
        to add additional words to the vocabulary of the recognizer. See
        [usage limits](https://cloud.google.com/speech/limits#content).
    - Each invocation of this argument appends the given value to the array.




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
