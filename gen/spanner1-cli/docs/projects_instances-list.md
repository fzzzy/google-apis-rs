Lists all instances in the given project.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/spanner.admin*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `spanner1 --scope <scope> projects instances-list ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required. The name of the project for which a list of instances is
        requested. Values are of the form `projects/&lt;project&gt;`.

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

* **-p page-size=integer**
    - Number of instances to be returned in the response. If 0 or less, defaults
        to the server&#39;s maximum allowed page size.

* **-p filter=string**
    - An expression for filtering the results of the request. Filter rules are
        case insensitive. The fields eligible for filtering are:
        
          * `name`
          * `display_name`
          * `labels.key` where key is the name of a label
        
        Some examples of using filters are:
        
          * `name:*` --&gt; The instance has a name.
          * `name:Howl` --&gt; The instance&#39;s name contains the string &#34;howl&#34;.
          * `name:HOWL` --&gt; Equivalent to above.
          * `NAME:howl` --&gt; Equivalent to above.
          * `labels.env:*` --&gt; The instance has the label &#34;env&#34;.
          * `labels.env:dev` --&gt; The instance has the label &#34;env&#34; and the value of
                               the label contains the string &#34;dev&#34;.
          * `name:howl labels.env:dev` --&gt; The instance&#39;s name contains &#34;howl&#34; and
                                         it has the label &#34;env&#34; with its value
                                         containing &#34;dev&#34;.

* **-p page-token=string**
    - If non-empty, `page_token` should contain a
        next_page_token from a
        previous ListInstancesResponse.

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
