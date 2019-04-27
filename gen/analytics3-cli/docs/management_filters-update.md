Updates an existing filter.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/analytics.edit* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/analytics.edit*.
You can set the scope for this method like this: `analytics3 --scope <scope> management filters-update ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - Account ID to which the filter belongs.
* **&lt;filter-id&gt;** *(string)*
    - ID of the filter to be updated.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Filter:
  account-id: string
  advanced-details:
    case-sensitive: boolean
    extract-a: string
    extract-b: string
    field-a: string
    field-a-index: integer
    field-a-required: boolean
    field-b: string
    field-b-index: integer
    field-b-required: boolean
    output-constructor: string
    output-to-field: string
    output-to-field-index: integer
    override-output-field: boolean
  created: string
  exclude-details:
    case-sensitive: boolean
    expression-value: string
    field: string
    field-index: integer
    kind: string
    match-type: string
  id: string
  include-details:
    case-sensitive: boolean
    expression-value: string
    field: string
    field-index: integer
    kind: string
    match-type: string
  kind: string
  lowercase-details:
    field: string
    field-index: integer
  name: string
  parent-link:
    href: string
    type: string
  search-and-replace-details:
    case-sensitive: boolean
    field: string
    field-index: integer
    replace-string: string
    search-string: string
  self-link: string
  type: string
  updated: string
  uppercase-details:
    field: string
    field-index: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=labore`
    - Account ID to which this filter belongs.
* `advanced-details    case-sensitive=false`
    - Indicates if the filter expressions are case sensitive.
* `extract-a=erat`
    - Expression to extract from field A.
* `extract-b=gubergren`
    - Expression to extract from field B.
* `field-a=erat`
    - Field A.
* `field-a-index=16`
    - The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION.
* `field-a-required=false`
    - Indicates if field A is required to match.
* `field-b=lorem`
    - Field B.
* `field-b-index=23`
    - The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION.
* `field-b-required=false`
    - Indicates if field B is required to match.
* `output-constructor=justo`
    - Expression used to construct the output value.
* `output-to-field=labore`
    - Output field.
* `output-to-field-index=73`
    - The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION.
* `override-output-field=false`
    - Indicates if the existing value of the output field, if any, should be overridden by the output expression.

* `..    created=dolor`
    - Time this filter was created.
* `exclude-details    case-sensitive=true`
    - Determines if the filter is case sensitive.
* `expression-value=voluptua.`
    - Filter expression value
* `field=no`
    - Field to filter. Possible values:  
        - Content and Traffic  
        - PAGE_REQUEST_URI, 
        - PAGE_HOSTNAME, 
        - PAGE_TITLE, 
        - REFERRAL, 
        - COST_DATA_URI (Campaign target URL), 
        - HIT_TYPE, 
        - INTERNAL_SEARCH_TERM, 
        - INTERNAL_SEARCH_TYPE, 
        - SOURCE_PROPERTY_TRACKING_ID,   
        - Campaign or AdGroup  
        - CAMPAIGN_SOURCE, 
        - CAMPAIGN_MEDIUM, 
        - CAMPAIGN_NAME, 
        - CAMPAIGN_AD_GROUP, 
        - CAMPAIGN_TERM, 
        - CAMPAIGN_CONTENT, 
        - CAMPAIGN_CODE, 
        - CAMPAIGN_REFERRAL_PATH,   
        - E-Commerce  
        - TRANSACTION_COUNTRY, 
        - TRANSACTION_REGION, 
        - TRANSACTION_CITY, 
        - TRANSACTION_AFFILIATION (Store or order location), 
        - ITEM_NAME, 
        - ITEM_CODE, 
        - ITEM_VARIATION, 
        - TRANSACTION_ID, 
        - TRANSACTION_CURRENCY_CODE, 
        - PRODUCT_ACTION_TYPE,   
        - Audience/Users  
        - BROWSER, 
        - BROWSER_VERSION, 
        - BROWSER_SIZE, 
        - PLATFORM, 
        - PLATFORM_VERSION, 
        - LANGUAGE, 
        - SCREEN_RESOLUTION, 
        - SCREEN_COLORS, 
        - JAVA_ENABLED (Boolean Field), 
        - FLASH_VERSION, 
        - GEO_SPEED (Connection speed), 
        - VISITOR_TYPE, 
        - GEO_ORGANIZATION (ISP organization), 
        - GEO_DOMAIN, 
        - GEO_IP_ADDRESS, 
        - GEO_IP_VERSION,   
        - Location  
        - GEO_COUNTRY, 
        - GEO_REGION, 
        - GEO_CITY,   
        - Event  
        - EVENT_CATEGORY, 
        - EVENT_ACTION, 
        - EVENT_LABEL,   
        - Other  
        - CUSTOM_FIELD_1, 
        - CUSTOM_FIELD_2, 
        - USER_DEFINED_VALUE,   
        - Application  
        - APP_ID, 
        - APP_INSTALLER_ID, 
        - APP_NAME, 
        - APP_VERSION, 
        - SCREEN, 
        - IS_APP (Boolean Field), 
        - IS_FATAL_EXCEPTION (Boolean Field), 
        - EXCEPTION_DESCRIPTION,   
        - Mobile device  
        - IS_MOBILE (Boolean Field, Deprecated. Use DEVICE_CATEGORY=mobile), 
        - IS_TABLET (Boolean Field, Deprecated. Use DEVICE_CATEGORY=tablet), 
        - DEVICE_CATEGORY, 
        - MOBILE_HAS_QWERTY_KEYBOARD (Boolean Field), 
        - MOBILE_HAS_NFC_SUPPORT (Boolean Field), 
        - MOBILE_HAS_CELLULAR_RADIO (Boolean Field), 
        - MOBILE_HAS_WIFI_SUPPORT (Boolean Field), 
        - MOBILE_BRAND_NAME, 
        - MOBILE_MODEL_NAME, 
        - MOBILE_MARKETING_NAME, 
        - MOBILE_POINTING_METHOD,   
        - Social  
        - SOCIAL_NETWORK, 
        - SOCIAL_ACTION, 
        - SOCIAL_ACTION_TARGET,   
        - Custom dimension  
        - CUSTOM_DIMENSION (See accompanying field index),
* `field-index=20`
    - The Index of the custom dimension. Set only if the field is a is CUSTOM_DIMENSION.
* `kind=magna`
    - Kind value for filter expression
* `match-type=et`
    - Match type for this filter. Possible values are BEGINS_WITH, EQUAL, ENDS_WITH, CONTAINS, or MATCHES. GEO_DOMAIN, GEO_IP_ADDRESS, PAGE_REQUEST_URI, or PAGE_HOSTNAME filters can use any match type; all other filters must use MATCHES.

* `..    id=sed`
    - Filter ID.
* `include-details    case-sensitive=true`
    - Determines if the filter is case sensitive.
* `expression-value=sit`
    - Filter expression value
* `field=et`
    - Field to filter. Possible values:  
        - Content and Traffic  
        - PAGE_REQUEST_URI, 
        - PAGE_HOSTNAME, 
        - PAGE_TITLE, 
        - REFERRAL, 
        - COST_DATA_URI (Campaign target URL), 
        - HIT_TYPE, 
        - INTERNAL_SEARCH_TERM, 
        - INTERNAL_SEARCH_TYPE, 
        - SOURCE_PROPERTY_TRACKING_ID,   
        - Campaign or AdGroup  
        - CAMPAIGN_SOURCE, 
        - CAMPAIGN_MEDIUM, 
        - CAMPAIGN_NAME, 
        - CAMPAIGN_AD_GROUP, 
        - CAMPAIGN_TERM, 
        - CAMPAIGN_CONTENT, 
        - CAMPAIGN_CODE, 
        - CAMPAIGN_REFERRAL_PATH,   
        - E-Commerce  
        - TRANSACTION_COUNTRY, 
        - TRANSACTION_REGION, 
        - TRANSACTION_CITY, 
        - TRANSACTION_AFFILIATION (Store or order location), 
        - ITEM_NAME, 
        - ITEM_CODE, 
        - ITEM_VARIATION, 
        - TRANSACTION_ID, 
        - TRANSACTION_CURRENCY_CODE, 
        - PRODUCT_ACTION_TYPE,   
        - Audience/Users  
        - BROWSER, 
        - BROWSER_VERSION, 
        - BROWSER_SIZE, 
        - PLATFORM, 
        - PLATFORM_VERSION, 
        - LANGUAGE, 
        - SCREEN_RESOLUTION, 
        - SCREEN_COLORS, 
        - JAVA_ENABLED (Boolean Field), 
        - FLASH_VERSION, 
        - GEO_SPEED (Connection speed), 
        - VISITOR_TYPE, 
        - GEO_ORGANIZATION (ISP organization), 
        - GEO_DOMAIN, 
        - GEO_IP_ADDRESS, 
        - GEO_IP_VERSION,   
        - Location  
        - GEO_COUNTRY, 
        - GEO_REGION, 
        - GEO_CITY,   
        - Event  
        - EVENT_CATEGORY, 
        - EVENT_ACTION, 
        - EVENT_LABEL,   
        - Other  
        - CUSTOM_FIELD_1, 
        - CUSTOM_FIELD_2, 
        - USER_DEFINED_VALUE,   
        - Application  
        - APP_ID, 
        - APP_INSTALLER_ID, 
        - APP_NAME, 
        - APP_VERSION, 
        - SCREEN, 
        - IS_APP (Boolean Field), 
        - IS_FATAL_EXCEPTION (Boolean Field), 
        - EXCEPTION_DESCRIPTION,   
        - Mobile device  
        - IS_MOBILE (Boolean Field, Deprecated. Use DEVICE_CATEGORY=mobile), 
        - IS_TABLET (Boolean Field, Deprecated. Use DEVICE_CATEGORY=tablet), 
        - DEVICE_CATEGORY, 
        - MOBILE_HAS_QWERTY_KEYBOARD (Boolean Field), 
        - MOBILE_HAS_NFC_SUPPORT (Boolean Field), 
        - MOBILE_HAS_CELLULAR_RADIO (Boolean Field), 
        - MOBILE_HAS_WIFI_SUPPORT (Boolean Field), 
        - MOBILE_BRAND_NAME, 
        - MOBILE_MODEL_NAME, 
        - MOBILE_MARKETING_NAME, 
        - MOBILE_POINTING_METHOD,   
        - Social  
        - SOCIAL_NETWORK, 
        - SOCIAL_ACTION, 
        - SOCIAL_ACTION_TARGET,   
        - Custom dimension  
        - CUSTOM_DIMENSION (See accompanying field index),
* `field-index=5`
    - The Index of the custom dimension. Set only if the field is a is CUSTOM_DIMENSION.
* `kind=sea`
    - Kind value for filter expression
* `match-type=nonumy`
    - Match type for this filter. Possible values are BEGINS_WITH, EQUAL, ENDS_WITH, CONTAINS, or MATCHES. GEO_DOMAIN, GEO_IP_ADDRESS, PAGE_REQUEST_URI, or PAGE_HOSTNAME filters can use any match type; all other filters must use MATCHES.

* `..    kind=consetetur`
    - Resource type for Analytics filter.
* `lowercase-details    field=accusam`
    - Field to use in the filter.
* `field-index=88`
    - The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION.

* `..    name=sea`
    - Name of this filter.
* `parent-link    href=vero`
    - Link to the account to which this filter belongs.
* `type=dolores`
    - Value is &#34;analytics#account&#34;.

* `..search-and-replace-details    case-sensitive=true`
    - Determines if the filter is case sensitive.
* `field=ut`
    - Field to use in the filter.
* `field-index=54`
    - The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION.
* `replace-string=sed`
    - Term to replace the search term with.
* `search-string=sit`
    - Term to search.

* `..    self-link=sit`
    - Link for this filter.
* `type=dolores`
    - Type of this filter. Possible values are INCLUDE, EXCLUDE, LOWERCASE, UPPERCASE, SEARCH_AND_REPLACE and ADVANCED.
* `updated=et`
    - Time this filter was last modified.
* `uppercase-details    field=sanctus`
    - Field to use in the filter.
* `field-index=43`
    - The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION.



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
