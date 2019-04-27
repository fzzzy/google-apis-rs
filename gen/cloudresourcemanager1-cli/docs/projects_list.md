Lists Projects that are visible to the user and satisfy the
specified filter. This method returns Projects in an unspecified order.
This method is eventually consistent with project mutations; this means
that a newly created project may not appear in the results or recent
updates to an existing project may not be reflected in the results. To
retrieve the latest state of a project, use the
GetProject method.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/cloud-platform.read-only*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `cloudresourcemanager1 --scope <scope> projects list ...`

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

* **-p filter=string**
    - An expression for filtering the results of the request.  Filter rules are
        case insensitive. The fields eligible for filtering are:
        
        + `name`
        + `id`
        + &lt;code&gt;labels.&lt;em&gt;key&lt;/em&gt;&lt;/code&gt; where *key* is the name of a label
        
        Some examples of using labels as filters:
        
        |Filter|Description|
        |------|-----------|
        |name:how*|The project&#39;s name starts with &#34;how&#34;.|
        |name:Howl|The project&#39;s name is `Howl` or `howl`.|
        |name:HOWL|Equivalent to above.|
        |NAME:howl|Equivalent to above.|
        |labels.color:*|The project has the label `color`.|
        |labels.color:red|The project&#39;s label `color` has the value `red`.|
        |labels.color:red&amp;nbsp;labels.size:big|The project&#39;s label `color` has the value `red` and its label `size` has the value `big`.
        
        If you specify a filter that has both `parent.type` and `parent.id`, then
        the `resourcemanager.projects.list` permission is checked on the parent.
        If the user has this permission, all projects under the parent will be
        returned after remaining filters have been applied. If the user lacks this
        permission, then all projects for which the user has the
        `resourcemanager.projects.get` permission will be returned after remaining
        filters have been applied. If no filter is specified, the call will return
        projects for which the user has `resourcemanager.projects.get` permissions.
        
        Optional.

* **-p page-token=string**
    - A pagination token returned from a previous call to ListProjects
        that indicates from where listing should continue.
        
        Optional.

* **-p page-size=integer**
    - The maximum number of Projects to return in the response.
        The server can return fewer Projects than requested.
        If unspecified, server picks an appropriate default.
        
        Optional.

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
