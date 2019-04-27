Translates input text, returning translated text.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-translation*
* *https://www.googleapis.com/auth/cloud-platform*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `translate2 --scope <scope> translations list ...`
# Required Scalar Arguments
* **&lt;q&gt;...** *(string)*
    - The input text to translate. Repeat this parameter to perform translation
        operations on multiple text inputs.
    - This property can be specified one or more times
* **&lt;target&gt;** *(string)*
    - The language to use for translation of the input text, set to one of the
        language codes listed in Language Support.

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

* **-p cid=string**
    - The customization id for translate

* **-p format=string**
    - The format of the source text, in either HTML (default) or plain-text. A
        value of &#34;html&#34; indicates HTML and a value of &#34;text&#34; indicates plain-text.

* **-p source=string**
    - The language of the source text, set to one of the language codes listed in
        Language Support. If the source language is not specified, the API will
        attempt to identify the source language automatically and return it within
        the response.

* **-p model=string**
    - The `model` type requested for this translation. Valid values are
        listed in public documentation.

# Optional General Properties

The following properties can configure any call, and are not specific to this method.

* **-p $-xgafv=string**
    - V1 error format.

* **-p access-token=string**
    - OAuth access token.

* **-p alt=string**
    - Data format for response.

* **-p bearer-token=string**
    - OAuth bearer token.

* **-p callback=string**
    - JSONP

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pp=boolean**
    - Pretty-print response.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.

* **-p upload-type=string**
    - Legacy upload protocol for media (e.g. &#34;media&#34;, &#34;multipart&#34;).

* **-p upload-protocol=string**
    - Upload protocol for media (e.g. &#34;raw&#34;, &#34;multipart&#34;).
