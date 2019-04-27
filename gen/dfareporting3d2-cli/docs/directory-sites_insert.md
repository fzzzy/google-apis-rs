Inserts a new directory site.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3d2 --scope <scope> directory-sites insert ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
DirectorySite:
  active: boolean
  country-id: string
  currency-id: string
  description: string
  id: string
  id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  inpage-tag-formats: [string]
  interstitial-tag-formats: [string]
  kind: string
  name: string
  parent-id: string
  settings:
    active-view-opt-out: boolean
    dfp-settings:
      dfp-network-code: string
      dfp-network-name: string
      programmatic-placement-accepted: boolean
      pub-paid-placement-accepted: boolean
      publisher-portal-only: boolean
    instream-video-placement-accepted: boolean
    interstitial-placement-accepted: boolean
    nielsen-ocr-opt-out: boolean
    verification-tag-opt-out: boolean
    video-active-view-opt-out: boolean
  url: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    active=true`
    - Whether this directory site is active.
* `country-id=et`
    - Country ID of this directory site. This is a read-only field.
* `currency-id=diam`
    - Currency ID of this directory site. This is a read-only field.
        Possible values are: 
        - &#34;1&#34; for USD 
        - &#34;2&#34; for GBP 
        - &#34;3&#34; for ESP 
        - &#34;4&#34; for SEK 
        - &#34;5&#34; for CAD 
        - &#34;6&#34; for JPY 
        - &#34;7&#34; for DEM 
        - &#34;8&#34; for AUD 
        - &#34;9&#34; for FRF 
        - &#34;10&#34; for ITL 
        - &#34;11&#34; for DKK 
        - &#34;12&#34; for NOK 
        - &#34;13&#34; for FIM 
        - &#34;14&#34; for ZAR 
        - &#34;15&#34; for IEP 
        - &#34;16&#34; for NLG 
        - &#34;17&#34; for EUR 
        - &#34;18&#34; for KRW 
        - &#34;19&#34; for TWD 
        - &#34;20&#34; for SGD 
        - &#34;21&#34; for CNY 
        - &#34;22&#34; for HKD 
        - &#34;23&#34; for NZD 
        - &#34;24&#34; for MYR 
        - &#34;25&#34; for BRL 
        - &#34;26&#34; for PTE 
        - &#34;27&#34; for MXP 
        - &#34;28&#34; for CLP 
        - &#34;29&#34; for TRY 
        - &#34;30&#34; for ARS 
        - &#34;31&#34; for PEN 
        - &#34;32&#34; for ILS 
        - &#34;33&#34; for CHF 
        - &#34;34&#34; for VEF 
        - &#34;35&#34; for COP 
        - &#34;36&#34; for GTQ 
        - &#34;37&#34; for PLN 
        - &#34;39&#34; for INR 
        - &#34;40&#34; for THB 
        - &#34;41&#34; for IDR 
        - &#34;42&#34; for CZK 
        - &#34;43&#34; for RON 
        - &#34;44&#34; for HUF 
        - &#34;45&#34; for RUB 
        - &#34;46&#34; for AED 
        - &#34;47&#34; for BGN 
        - &#34;48&#34; for HRK 
        - &#34;49&#34; for MXN 
        - &#34;50&#34; for NGN
* `description=amet.`
    - Description of this directory site. This is a read-only field.
* `id=justo`
    - ID of this directory site. This is a read-only, auto-generated field.
* `id-dimension-value    dimension-name=et`
    - The name of the dimension.
* `etag=rebum.`
    - The eTag of this response for caching purposes.
* `id=eos`
    - The ID associated with the value if available.
* `kind=diam`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=eos`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=dolore`
    - The value of the dimension.

* `..    inpage-tag-formats=sea`
    - Tag types for regular placements.
        
        Acceptable values are:
        - &#34;STANDARD&#34;
        - &#34;IFRAME_JAVASCRIPT_INPAGE&#34;
        - &#34;INTERNAL_REDIRECT_INPAGE&#34;
        - &#34;JAVASCRIPT_INPAGE&#34;
    - Each invocation of this argument appends the given value to the array.
* `interstitial-tag-formats=lorem`
    - Tag types for interstitial placements.
        
        Acceptable values are:
        - &#34;IFRAME_JAVASCRIPT_INTERSTITIAL&#34;
        - &#34;INTERNAL_REDIRECT_INTERSTITIAL&#34;
        - &#34;JAVASCRIPT_INTERSTITIAL&#34;
    - Each invocation of this argument appends the given value to the array.
* `kind=et`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#directorySite&#34;.
* `name=tempor`
    - Name of this directory site.
* `parent-id=erat`
    - Parent directory site ID.
* `settings    active-view-opt-out=true`
    - Whether this directory site has disabled active view creatives.
* `dfp-settings    dfp-network-code=kasd`
    - Ad Manager network code for this directory site.
* `dfp-network-name=invidunt`
    - Ad Manager network name for this directory site.
* `programmatic-placement-accepted=false`
    - Whether this directory site accepts programmatic placements.
* `pub-paid-placement-accepted=true`
    - Whether this directory site accepts publisher-paid tags.
* `publisher-portal-only=false`
    - Whether this directory site is available only via Publisher Portal.

* `..    instream-video-placement-accepted=false`
    - Whether this site accepts in-stream video ads.
* `interstitial-placement-accepted=true`
    - Whether this site accepts interstitial ads.
* `nielsen-ocr-opt-out=true`
    - Whether this directory site has disabled Nielsen OCR reach ratings.
* `verification-tag-opt-out=false`
    - Whether this directory site has disabled generation of Verification ins tags.
* `video-active-view-opt-out=false`
    - Whether this directory site has disabled active view for in-stream video creatives. This is a read-only field.

* `..    url=dolore`
    - URL of this directory site.


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

* **-p alt=string**
    - Data format for the response.

* **-p fields=string**
    - Selector specifying which fields to include in a partial response.

* **-p key=string**
    - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.

* **-p oauth-token=string**
    - OAuth 2.0 token for the current user.

* **-p pretty-print=boolean**
    - Returns response with indentations and line breaks.

* **-p quota-user=string**
    - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.

* **-p user-ip=string**
    - Deprecated. Please use quotaUser instead.
