Creates an account ticket.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/analytics.provision* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/analytics.provision*.
You can set the scope for this method like this: `analytics3 --scope <scope> provisioning create-account-ticket ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
AccountTicket:
  account:
    child-link:
      href: string
      type: string
    created: string
    id: string
    kind: string
    name: string
    permissions:
      effective: [string]
    self-link: string
    starred: boolean
    updated: string
  id: string
  kind: string
  profile:
    account-id: string
    bot-filtering-enabled: boolean
    child-link:
      href: string
      type: string
    created: string
    currency: string
    default-page: string
    e-commerce-tracking: boolean
    enhanced-e-commerce-tracking: boolean
    exclude-query-parameters: string
    id: string
    internal-web-property-id: string
    kind: string
    name: string
    parent-link:
      href: string
      type: string
    permissions:
      effective: [string]
    self-link: string
    site-search-category-parameters: string
    site-search-query-parameters: string
    starred: boolean
    strip-site-search-category-parameters: boolean
    strip-site-search-query-parameters: boolean
    timezone: string
    type: string
    updated: string
    web-property-id: string
    website-url: string
  redirect-uri: string
  webproperty:
    account-id: string
    child-link:
      href: string
      type: string
    created: string
    data-retention-reset-on-new-activity: boolean
    data-retention-ttl: string
    default-profile-id: string
    id: string
    industry-vertical: string
    internal-web-property-id: string
    kind: string
    level: string
    name: string
    parent-link:
      href: string
      type: string
    permissions:
      effective: [string]
    profile-count: integer
    self-link: string
    starred: boolean
    updated: string
    website-url: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .account.child-link    href=clita`
    - Link to the list of web properties for this account.
* `type=vero`
    - Type of the child link. Its value is &#34;analytics#webproperties&#34;.

* `..    created=ut`
    - Time the account was created.
* `id=est`
    - Account ID.
* `kind=diam`
    - Resource type for Analytics account.
* `name=sit`
    - Account name.
* `permissions    effective=amet`
    - All the permissions that the user has for this account. These include any implied permissions (e.g., EDIT implies VIEW).
    - Each invocation of this argument appends the given value to the array.

* `..    self-link=eirmod`
    - Link for this account.
* `starred=false`
    - Indicates whether this account is starred or not.
* `updated=voluptua.`
    - Time the account was last modified.

* `..    id=invidunt`
    - Account ticket ID used to access the account ticket.
* `kind=clita`
    - Resource type for account ticket.
* `profile    account-id=justo`
    - Account ID to which this view (profile) belongs.
* `bot-filtering-enabled=true`
    - Indicates whether bot filtering is enabled for this view (profile).
* `child-link    href=lorem`
    - Link to the list of goals for this view (profile).
* `type=eirmod`
    - Value is &#34;analytics#goals&#34;.

* `..    created=diam`
    - Time this view (profile) was created.
* `currency=est`
    - The currency type associated with this view (profile), defaults to USD. The supported values are:
        USD, JPY, EUR, GBP, AUD, KRW, BRL, CNY, DKK, RUB, SEK, NOK, PLN, TRY, TWD, HKD, THB, IDR, ARS, MXN, VND, PHP, INR, CHF, CAD, CZK, NZD, HUF, BGN, LTL, ZAR, UAH, AED, BOB, CLP, COP, EGP, HRK, ILS, MAD, MYR, PEN, PKR, RON, RSD, SAR, SGD, VEF, LVL
* `default-page=vero`
    - Default page for this view (profile).
* `e-commerce-tracking=true`
    - Indicates whether ecommerce tracking is enabled for this view (profile).
* `enhanced-e-commerce-tracking=true`
    - Indicates whether enhanced ecommerce tracking is enabled for this view (profile). This property can only be enabled if ecommerce tracking is enabled.
* `exclude-query-parameters=rebum.`
    - The query parameters that are excluded from this view (profile).
* `id=clita`
    - View (Profile) ID.
* `internal-web-property-id=magna`
    - Internal ID for the web property to which this view (profile) belongs.
* `kind=stet`
    - Resource type for Analytics view (profile).
* `name=dolore`
    - Name of this view (profile).
* `parent-link    href=diam`
    - Link to the web property to which this view (profile) belongs.
* `type=ut`
    - Value is &#34;analytics#webproperty&#34;.

* `..permissions    effective=stet`
    - All the permissions that the user has for this view (profile). These include any implied permissions (e.g., EDIT implies VIEW) or inherited permissions from the parent web property.
    - Each invocation of this argument appends the given value to the array.

* `..    self-link=et`
    - Link for this view (profile).
* `site-search-category-parameters=sed`
    - Site search category parameters for this view (profile).
* `site-search-query-parameters=aliquyam`
    - The site search query parameters for this view (profile).
* `starred=false`
    - Indicates whether this view (profile) is starred or not.
* `strip-site-search-category-parameters=true`
    - Whether or not Analytics will strip search category parameters from the URLs in your reports.
* `strip-site-search-query-parameters=true`
    - Whether or not Analytics will strip search query parameters from the URLs in your reports.
* `timezone=sanctus`
    - Time zone for which this view (profile) has been configured. Time zones are identified by strings from the TZ database.
* `type=sanctus`
    - View (Profile) type. Supported types: WEB or APP.
* `updated=sadipscing`
    - Time this view (profile) was last modified.
* `web-property-id=ipsum`
    - Web property ID of the form UA-XXXXX-YY to which this view (profile) belongs.
* `website-url=takimata`
    - Website URL for this view (profile).

* `..    redirect-uri=voluptua.`
    - Redirect URI where the user will be sent after accepting Terms of Service. Must be configured in APIs console as a callback URL.
* `webproperty    account-id=labore`
    - Account ID to which this web property belongs.
* `child-link    href=et`
    - Link to the list of views (profiles) for this web property.
* `type=no`
    - Type of the parent link. Its value is &#34;analytics#profiles&#34;.

* `..    created=sed`
    - Time this web property was created.
* `data-retention-reset-on-new-activity=true`
    - Set to true to reset the retention period of the user identifier with each new event from that user (thus setting the expiration date to current time plus retention period).
        Set to false to delete data associated with the user identifer automatically after the rentention period.
        This property cannot be set on insert.
* `data-retention-ttl=diam`
    - The length of time for which user and event data is retained.
        This property cannot be set on insert.
* `default-profile-id=lorem`
    - Default view (profile) ID.
* `id=amet.`
    - Web property ID of the form UA-XXXXX-YY.
* `industry-vertical=sed`
    - The industry vertical/category selected for this web property.
* `internal-web-property-id=eos`
    - Internal ID for this web property.
* `kind=sadipscing`
    - Resource type for Analytics WebProperty.
* `level=erat`
    - Level for this web property. Possible values are STANDARD or PREMIUM.
* `name=dolores`
    - Name of this web property.
* `parent-link    href=invidunt`
    - Link to the account for this web property.
* `type=ipsum`
    - Type of the parent link. Its value is &#34;analytics#account&#34;.

* `..permissions    effective=rebum.`
    - All the permissions that the user has for this web property. These include any implied permissions (e.g., EDIT implies VIEW) or inherited permissions from the parent account.
    - Each invocation of this argument appends the given value to the array.

* `..    profile-count=2`
    - View (Profile) count for this web property.
* `self-link=nonumy`
    - Link for this web property.
* `starred=true`
    - Indicates whether this web property is starred or not.
* `updated=et`
    - Time this web property was last modified.
* `website-url=sanctus`
    - Website url for this web property.



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
