Updates an existing account.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3 --scope <scope> accounts update ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Account:
  account-permission-ids: [string]
  account-profile: string
  active: boolean
  active-ads-limit-tier: string
  active-view-opt-out: boolean
  available-permission-ids: [string]
  country-id: string
  currency-id: string
  default-creative-size-id: string
  description: string
  id: string
  kind: string
  locale: string
  maximum-image-size: string
  name: string
  nielsen-ocr-enabled: boolean
  reports-configuration:
    exposure-to-conversion-enabled: boolean
    lookback-configuration:
      click-duration: integer
      post-impression-activities-duration: integer
    report-generation-time-zone-id: string
  share-reports-with-twitter: boolean
  teaser-size-limit: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-permission-ids=labore`
    - Account permissions assigned to this account.
    - Each invocation of this argument appends the given value to the array.
* `account-profile=invidunt`
    - Profile for this account. This is a read-only field that can be left blank.
* `active=false`
    - Whether this account is active.
* `active-ads-limit-tier=sadipscing`
    - Maximum number of active ads allowed for this account.
* `active-view-opt-out=false`
    - Whether to serve creatives with Active View tags. If disabled, viewability data will not be available for any impressions.
* `available-permission-ids=dolore`
    - User role permissions available to the user roles of this account.
    - Each invocation of this argument appends the given value to the array.
* `country-id=nonumy`
    - ID of the country associated with this account.
* `currency-id=sed`
    - ID of currency associated with this account. This is a required field.
        Acceptable values are: 
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
* `default-creative-size-id=aliquyam`
    - Default placement dimensions for this account.
* `description=sit`
    - Description of this account.
* `id=eirmod`
    - ID of this account. This is a read-only, auto-generated field.
* `kind=consetetur`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#account&#34;.
* `locale=labore`
    - Locale of this account.
        Acceptable values are: 
        - &#34;cs&#34; (Czech) 
        - &#34;de&#34; (German) 
        - &#34;en&#34; (English) 
        - &#34;en-GB&#34; (English United Kingdom) 
        - &#34;es&#34; (Spanish) 
        - &#34;fr&#34; (French) 
        - &#34;it&#34; (Italian) 
        - &#34;ja&#34; (Japanese) 
        - &#34;ko&#34; (Korean) 
        - &#34;pl&#34; (Polish) 
        - &#34;pt-BR&#34; (Portuguese Brazil) 
        - &#34;ru&#34; (Russian) 
        - &#34;sv&#34; (Swedish) 
        - &#34;tr&#34; (Turkish) 
        - &#34;zh-CN&#34; (Chinese Simplified) 
        - &#34;zh-TW&#34; (Chinese Traditional)
* `maximum-image-size=sed`
    - Maximum image size allowed for this account, in kilobytes. Value must be greater than or equal to 1.
* `name=ea`
    - Name of this account. This is a required field, and must be less than 128 characters long and be globally unique.
* `nielsen-ocr-enabled=false`
    - Whether campaigns created in this account will be enabled for Nielsen OCR reach ratings by default.
* `reports-configuration    exposure-to-conversion-enabled=false`
    - Whether the exposure to conversion report is enabled. This report shows detailed pathway information on up to 10 of the most recent ad exposures seen by a user before converting.
* `lookback-configuration    click-duration=77`
    - Lookback window, in days, from the last time a given user clicked on one of your ads. If you enter 0, clicks will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive.
* `post-impression-activities-duration=63`
    - Lookback window, in days, from the last time a given user viewed one of your ads. If you enter 0, impressions will not be considered as triggering events for floodlight tracking. If you leave this field blank, the default value for your account will be used. Acceptable values are 0 to 90, inclusive.

* `..    report-generation-time-zone-id=sea`
    - Report generation time zone ID of this account. This is a required field that can only be changed by a superuser.
        Acceptable values are:
        
        - &#34;1&#34; for &#34;America/New_York&#34; 
        - &#34;2&#34; for &#34;Europe/London&#34; 
        - &#34;3&#34; for &#34;Europe/Paris&#34; 
        - &#34;4&#34; for &#34;Africa/Johannesburg&#34; 
        - &#34;5&#34; for &#34;Asia/Jerusalem&#34; 
        - &#34;6&#34; for &#34;Asia/Shanghai&#34; 
        - &#34;7&#34; for &#34;Asia/Hong_Kong&#34; 
        - &#34;8&#34; for &#34;Asia/Tokyo&#34; 
        - &#34;9&#34; for &#34;Australia/Sydney&#34; 
        - &#34;10&#34; for &#34;Asia/Dubai&#34; 
        - &#34;11&#34; for &#34;America/Los_Angeles&#34; 
        - &#34;12&#34; for &#34;Pacific/Auckland&#34; 
        - &#34;13&#34; for &#34;America/Sao_Paulo&#34;

* `..    share-reports-with-twitter=false`
    - Share Path to Conversion reports with Twitter.
* `teaser-size-limit=ipsum`
    - File size limit in kilobytes of Rich Media teaser creatives. Acceptable values are 1 to 10240, inclusive.


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
