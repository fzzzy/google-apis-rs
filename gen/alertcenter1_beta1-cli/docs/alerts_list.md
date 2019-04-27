Lists all the alerts for the current user and application.

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
    - Optional. Query string for filtering alert results.
        This string must be specified as an expression or list of expressions,
        using the following grammar:
        
        ### Expressions
        
        An expression has the general form `&lt;field&gt; &lt;operator&gt; &lt;value&gt;`.
        
        A field or value which contains a space or a colon must be enclosed by
        double quotes.
        
        #### Operators
        
        Operators follow the BNF specification:
        
        `&lt;equalityOperator&gt; ::= &#34;=&#34; | &#34;:&#34;`
        
        `&lt;relationalOperator&gt; ::= &#34;&lt;&#34; | &#34;&gt;&#34; | &#34;&lt;=&#34; | &#34;&gt;=&#34;`
        
        Relational operators are defined only for timestamp fields. Equality
        operators are defined only for string fields.
        
        #### Timestamp fields
        
        The value supplied for a timestamp field must be an
        [RFC 3339](https://tools.ietf.org/html/rfc3339) date-time string.
        
        Supported timestamp fields are `create_time`, `start_time`, and `end_time`.
        
        #### String fields
        
        The value supplied for a string field may be an arbitrary string.
        
        #### Examples
        
        To query for all alerts created on or after April 5, 2018:
        
        `create_time &gt;= &#34;2018-04-05T00:00:00Z&#34;`
        
        To query for all alerts from the source &#34;Gmail phishing&#34;:
        
        `source:&#34;Gmail phishing&#34;`
        
        ### Joining expressions
        
        Expressions may be joined to form a more complex query. The BNF
        specification is:
        
        `&lt;expressionList&gt; ::= &lt;expression&gt; | &lt;expressionList&gt; &lt;conjunction&gt;
        &lt;expressionList&gt; | &lt;negation&gt; &lt;expressionList&gt;`
        `&lt;conjunction&gt; ::= &#34;AND&#34; | &#34;OR&#34; | &#34;&#34;`
        `&lt;negation&gt; ::= &#34;NOT&#34;`
        
        Using the empty string as a conjunction acts as an implicit AND.
        
        The precedence of joining operations, from highest to lowest, is NOT, AND,
        OR.
        
        #### Examples
        
        To query for all alerts which started in 2017:
        
        `start_time &gt;= &#34;2017-01-01T00:00:00Z&#34; AND start_time &lt;
        &#34;2018-01-01T00:00:00Z&#34;`
        
        To query for all user reported phishing alerts from the source
        &#34;Gmail phishing&#34;:
        
        `type:&#34;User reported phishing&#34; source:&#34;Gmail phishing&#34;`

* **-p customer-id=string**
    - Optional. The unique identifier of the Google account of the customer the
        alerts are associated with. This is obfuscated and not the plain
        customer ID as stored internally. Inferred from the caller identity if not
        provided.

* **-p page-token=string**
    - Optional. A token identifying a page of results the server should return.
        If empty, a new iteration is started. To continue an iteration, pass in
        the value from the previous ListAlertsResponse&#39;s next_page_token field.

* **-p page-size=integer**
    - Optional. Requested page size. Server may return fewer items than
        requested. If unspecified, server will pick an appropriate default.

* **-p order-by=string**
    - Optional. Sort the list results by a certain order.
        If not specified results may be returned in arbitrary order.
        You can sort the results in a descending order based on the creation
        timestamp using order_by=&#34;create_time desc&#34;.
        Currently, only sorting by create_time desc is supported.

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
