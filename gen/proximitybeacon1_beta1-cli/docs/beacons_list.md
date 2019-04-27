Searches the beacon registry for beacons that match the given search
criteria. Only those beacons that the client has permission to list
will be returned.

Authenticate using an [OAuth access token](https://developers.google.com/identity/protocols/OAuth2)
from a signed-in user with **viewer**, **Is owner** or **Can edit**
permissions in the Google Developers Console project.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/userlocation.beacon.registry* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/userlocation.beacon.registry*.
You can set the scope for this method like this: `proximitybeacon1-beta1 --scope <scope> beacons list ...`

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
    - The maximum number of records to return for this request, up to a
        server-defined upper limit.

* **-p q=string**
    - Filter query string that supports the following field filters:
        
        * **description:`&#34;&lt;string&gt;&#34;`**
          For example: **description:&#34;Room 3&#34;**
          Returns beacons whose description matches tokens in the string &#34;Room 3&#34;
          (not necessarily that exact string).
          The string must be double-quoted.
        * **status:`&lt;enum&gt;`**
          For example: **status:active**
          Returns beacons whose status matches the given value. Values must be
          one of the Beacon.Status enum values (case insensitive). Accepts
          multiple filters which will be combined with OR logic.
        * **stability:`&lt;enum&gt;`**
          For example: **stability:mobile**
          Returns beacons whose expected stability matches the given value.
          Values must be one of the Beacon.Stability enum values (case
          insensitive). Accepts multiple filters which will be combined with
          OR logic.
        * **place\_id:`&#34;&lt;string&gt;&#34;`**
          For example: **place\_id:&#34;ChIJVSZzVR8FdkgRXGmmm6SslKw=&#34;**
          Returns beacons explicitly registered at the given place, expressed as
          a Place ID obtained from [Google Places API](/places/place-id). Does not
          match places inside the given place. Does not consider the beacon&#39;s
          actual location (which may be different from its registered place).
          Accepts multiple filters that will be combined with OR logic. The place
          ID must be double-quoted.
        * **registration\_time`[&lt;|&gt;|&lt;=|&gt;=]&lt;integer&gt;`**
          For example: **registration\_time&gt;=1433116800**
          Returns beacons whose registration time matches the given filter.
          Supports the operators: &lt;, &gt;, &lt;=, and &gt;=. Timestamp must be expressed as
          an integer number of seconds since midnight January 1, 1970 UTC. Accepts
          at most two filters that will be combined with AND logic, to support
          &#34;between&#34; semantics. If more than two are supplied, the latter ones are
          ignored.
        * **lat:`&lt;double&gt; lng:&lt;double&gt; radius:&lt;integer&gt;`**
          For example: **lat:51.1232343 lng:-1.093852 radius:1000**
          Returns beacons whose registered location is within the given circle.
          When any of these fields are given, all are required. Latitude and
          longitude must be decimal degrees between -90.0 and 90.0 and between
          -180.0 and 180.0 respectively. Radius must be an integer number of
          meters between 10 and 1,000,000 (1000 km).
        * **property:`&#34;&lt;string&gt;=&lt;string&gt;&#34;`**
          For example: **property:&#34;battery-type=CR2032&#34;**
          Returns beacons which have a property of the given name and value.
          Supports multiple filters which will be combined with OR logic.
          The entire name=value string must be double-quoted as one string.
        * **attachment\_type:`&#34;&lt;string&gt;&#34;`**
          For example: **attachment_type:&#34;my-namespace/my-type&#34;**
          Returns beacons having at least one attachment of the given namespaced
          type. Supports &#34;any within this namespace&#34; via the partial wildcard
          syntax: &#34;my-namespace/*&#34;. Supports multiple filters which will be
          combined with OR logic. The string must be double-quoted.
        * **indoor\_level:`&#34;&lt;string&gt;&#34;`**
          For example: **indoor\_level:&#34;1&#34;**
          Returns beacons which are located on the given indoor level. Accepts
          multiple filters that will be combined with OR logic.
        
        Multiple filters on the same field are combined with OR logic (except
        registration_time which is combined with AND logic).
        Multiple filters on different fields are combined with AND logic.
        Filters should be separated by spaces.
        
        As with any HTTP query string parameter, the whole filter expression must
        be URL-encoded.
        
        Example REST request:
        `GET /v1beta1/beacons?q=status:active%20lat:51.123%20lng:-1.095%20radius:1000`

* **-p project-id=string**
    - The project id to list beacons under. If not present then the project
        credential that made the request is used as the project.
        Optional.

* **-p page-token=string**
    - A pagination token obtained from a previous request to list beacons.

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
