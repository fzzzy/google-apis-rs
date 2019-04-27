Synthesizes speech synchronously: receive results after all text input
has been processed.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/cloud-platform* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `texttospeech1 --scope <scope> text synthesize ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
SynthesizeSpeechRequest:
  audio-config:
    audio-encoding: string
    pitch: number
    sample-rate-hertz: integer
    speaking-rate: number
    volume-gain-db: number
  input:
    ssml: string
    text: string
  voice:
    language-code: string
    name: string
    ssml-gender: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .audio-config    audio-encoding=eirmod`
    - Required. The format of the requested audio byte stream.
* `pitch=0.533265573605`
    - Optional speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20
        semitones from the original pitch. -20 means decrease 20 semitones from the
        original pitch.
* `sample-rate-hertz=36`
    - The synthesis sample rate (in hertz) for this audio. Optional.  If this is
        different from the voice&#39;s natural sample rate, then the synthesizer will
        honor this request by converting to the desired sample rate (which might
        result in worse audio quality), unless the specified sample rate is not
        supported for the encoding chosen, in which case it will fail the request
        and return google.rpc.Code.INVALID_ARGUMENT.
* `speaking-rate=0.585787353902`
    - Optional speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal
        native speed supported by the specific voice. 2.0 is twice as fast, and
        0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any
        other values &lt; 0.25 or &gt; 4.0 will return an error.
* `volume-gain-db=0.165687283689`
    - Optional volume gain (in dB) of the normal native volume supported by the
        specific voice, in the range [-96.0, 16.0]. If unset, or set to a value of
        0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB)
        will play at approximately half the amplitude of the normal native signal
        amplitude. A value of +6.0 (dB) will play at approximately twice the
        amplitude of the normal native signal amplitude. Strongly recommend not to
        exceed +10 (dB) as there&#39;s usually no effective increase in loudness for
        any value greater than that.

* `..input    ssml=dolores`
    - The SSML document to be synthesized. The SSML document must be valid
        and well-formed. Otherwise the RPC will fail and return
        google.rpc.Code.INVALID_ARGUMENT. For more information, see
        [SSML](/speech/text-to-speech/docs/ssml).
* `text=kasd`
    - The raw text to be synthesized.

* `..voice    language-code=accusam`
    - The language (and optionally also the region) of the voice expressed as a
        [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag, e.g.
        &#34;en-US&#34;. Required. This should not include a script tag (e.g. use
        &#34;cmn-cn&#34; rather than &#34;cmn-Hant-cn&#34;), because the script will be inferred
        from the input provided in the SynthesisInput.  The TTS service
        will use this parameter to help choose an appropriate voice.  Note that
        the TTS service may choose a voice with a slightly different language code
        than the one selected; it may substitute a different region
        (e.g. using en-US rather than en-CA if there isn&#39;t a Canadian voice
        available), or even a different language, e.g. using &#34;nb&#34; (Norwegian
        Bokmal) instead of &#34;no&#34; (Norwegian)&#34;.
* `name=takimata`
    - The name of the voice. Optional; if not set, the service will choose a
        voice based on the other parameters such as language_code and gender.
* `ssml-gender=justo`
    - The preferred gender of the voice. Optional; if not set, the service will
        choose a voice based on the other parameters such as language_code and
        name. Note that this is only a preference, not requirement; if a
        voice of the appropriate gender is not available, the synthesizer should
        substitute a voice with a different gender rather than failing the request.



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
