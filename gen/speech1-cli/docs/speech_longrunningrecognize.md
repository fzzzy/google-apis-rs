Performs asynchronous speech recognition: receive results via the
google.longrunning.Operations interface. Returns either an
`Operation.error` or an `Operation.response` which contains
a `LongRunningRecognizeResponse` message.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `speech1 --scope <scope> speech longrunningrecognize ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
LongRunningRecognizeRequest:
  audio:
    content: string
    uri: string
  config:
    enable-automatic-punctuation: boolean
    enable-word-time-offsets: boolean
    encoding: string
    language-code: string
    max-alternatives: integer
    model: string
    profanity-filter: boolean
    sample-rate-hertz: integer
    use-enhanced: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .audio    content=eirmod`
    - The audio data bytes encoded as specified in
        `RecognitionConfig`. Note: as with all bytes fields, protobuffers use a
        pure binary representation, whereas JSON representations use base64.
* `uri=sit`
    - URI that points to a file that contains audio data bytes as specified in
        `RecognitionConfig`. The file must not be compressed (for example, gzip).
        Currently, only Google Cloud Storage URIs are
        supported, which must be specified in the following format:
        `gs://bucket_name/object_name` (other URI formats return
        google.rpc.Code.INVALID_ARGUMENT). For more information, see
        [Request URIs](https://cloud.google.com/storage/docs/reference-uris).

* `..config    enable-automatic-punctuation=false`
    - *Optional* If &#39;true&#39;, adds punctuation to recognition result hypotheses.
        This feature is only available in select languages. Setting this for
        requests in other languages has no effect at all.
        The default &#39;false&#39; value does not add punctuation to result hypotheses.
        Note: This is currently offered as an experimental service, complimentary
        to all users. In the future this may be exclusively available as a
        premium feature.
* `enable-word-time-offsets=true`
    - *Optional* If `true`, the top result includes a list of words and
        the start and end time offsets (timestamps) for those words. If
        `false`, no word-level time offset information is returned. The default is
        `false`.
* `encoding=et`
    - Encoding of audio data sent in all `RecognitionAudio` messages.
        This field is optional for `FLAC` and `WAV` audio files and required
        for all other audio formats. For details, see AudioEncoding.
* `language-code=dolores`
    - *Required* The language of the supplied audio as a
        [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag.
        Example: &#34;en-US&#34;.
        See [Language Support](/speech-to-text/docs/languages)
        for a list of the currently supported language codes.
* `max-alternatives=38`
    - *Optional* Maximum number of recognition hypotheses to be returned.
        Specifically, the maximum number of `SpeechRecognitionAlternative` messages
        within each `SpeechRecognitionResult`.
        The server may return fewer than `max_alternatives`.
        Valid values are `0`-`30`. A value of `0` or `1` will return a maximum of
        one. If omitted, will return a maximum of one.
* `model=accusam`
    - *Optional* Which model to select for the given request. Select the model
        best suited to your domain to get best results. If a model is not
        explicitly specified, then we auto-select a model based on the parameters
        in the RecognitionConfig.
        &lt;table&gt;
          &lt;tr&gt;
            &lt;td&gt;&lt;b&gt;Model&lt;/b&gt;&lt;/td&gt;
            &lt;td&gt;&lt;b&gt;Description&lt;/b&gt;&lt;/td&gt;
          &lt;/tr&gt;
          &lt;tr&gt;
            &lt;td&gt;&lt;code&gt;command_and_search&lt;/code&gt;&lt;/td&gt;
            &lt;td&gt;Best for short queries such as voice commands or voice search.&lt;/td&gt;
          &lt;/tr&gt;
          &lt;tr&gt;
            &lt;td&gt;&lt;code&gt;phone_call&lt;/code&gt;&lt;/td&gt;
            &lt;td&gt;Best for audio that originated from a phone call (typically
            recorded at an 8khz sampling rate).&lt;/td&gt;
          &lt;/tr&gt;
          &lt;tr&gt;
            &lt;td&gt;&lt;code&gt;video&lt;/code&gt;&lt;/td&gt;
            &lt;td&gt;Best for audio that originated from from video or includes multiple
                speakers. Ideally the audio is recorded at a 16khz or greater
                sampling rate. This is a premium model that costs more than the
                standard rate.&lt;/td&gt;
          &lt;/tr&gt;
          &lt;tr&gt;
            &lt;td&gt;&lt;code&gt;default&lt;/code&gt;&lt;/td&gt;
            &lt;td&gt;Best for audio that is not one of the specific audio models.
                For example, long-form audio. Ideally the audio is high-fidelity,
                recorded at a 16khz or greater sampling rate.&lt;/td&gt;
          &lt;/tr&gt;
        &lt;/table&gt;
* `profanity-filter=true`
    - *Optional* If set to `true`, the server will attempt to filter out
        profanities, replacing all but the initial character in each filtered word
        with asterisks, e.g. &#34;f***&#34;. If set to `false` or omitted, profanities
        won&#39;t be filtered out.
* `sample-rate-hertz=31`
    - Sample rate in Hertz of the audio data sent in all
        `RecognitionAudio` messages. Valid values are: 8000-48000.
        16000 is optimal. For best results, set the sampling rate of the audio
        source to 16000 Hz. If that&#39;s not possible, use the native sample rate of
        the audio source (instead of re-sampling).
        This field is optional for `FLAC` and `WAV` audio files and required
        for all other audio formats. For details, see AudioEncoding.
* `use-enhanced=true`
    - *Optional* Set to true to use an enhanced model for speech recognition.
        If `use_enhanced` is set to true and the `model` field is not set, then
        an appropriate enhanced model is chosen if:
        1. project is eligible for requesting enhanced models
        2. an enhanced model exists for the audio
        
        If `use_enhanced` is true and an enhanced version of the specified model
        does not exist, then the speech is recognized using the standard version
        of the specified model.
        
        Enhanced speech models require that you opt-in to data logging using
        instructions in the
        [documentation](/speech-to-text/docs/enable-data-logging). If you set
        `use_enhanced` to true and you have not enabled audio logging, then you
        will receive an error.



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
