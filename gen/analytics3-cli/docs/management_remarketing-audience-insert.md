Creates a new remarketing audience.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/analytics.edit* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/analytics.edit*.
You can set the scope for this method like this: `analytics3 --scope <scope> management remarketing-audience-insert ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - The account ID for which to create the remarketing audience.
* **&lt;web-property-id&gt;** *(string)*
    - Web property ID for which to create the remarketing audience.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
RemarketingAudience:
  account-id: string
  audience-definition:
    include-conditions:
      days-to-look-back: integer
      is-smart-list: boolean
      kind: string
      membership-duration-days: integer
      segment: string
  audience-type: string
  created: string
  description: string
  id: string
  internal-web-property-id: string
  kind: string
  linked-views: [string]
  name: string
  state-based-audience-definition:
    exclude-conditions:
      exclusion-duration: string
      segment: string
    include-conditions:
      days-to-look-back: integer
      is-smart-list: boolean
      kind: string
      membership-duration-days: integer
      segment: string
  updated: string
  web-property-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=dolore`
    - Account ID to which this remarketing audience belongs.
* `audience-definition.include-conditions    days-to-look-back=87`
    - The look-back window lets you specify a time frame for evaluating the behavior that qualifies users for your audience. For example, if your filters include users from Central Asia, and Transactions Greater than 2, and you set the look-back window to 14 days, then any user from Central Asia whose cumulative transactions exceed 2 during the last 14 days is added to the audience.
* `is-smart-list=true`
    - Boolean indicating whether this segment is a smart list. https://support.google.com/analytics/answer/4628577
* `kind=labore`
    - Resource type for include conditions.
* `membership-duration-days=4`
    - Number of days (in the range 1 to 540) a user remains in the audience.
* `segment=invidunt`
    - The segment condition that will cause a user to be added to an audience.


* `...    audience-type=dolore`
    - The type of audience, either SIMPLE or STATE_BASED.
* `created=dolor`
    - Time this remarketing audience was created.
* `description=ipsum`
    - The description of this remarketing audience.
* `id=et`
    - Remarketing Audience ID.
* `internal-web-property-id=kasd`
    - Internal ID for the web property to which this remarketing audience belongs.
* `kind=sed`
    - Collection type.
* `linked-views=tempor`
    - The views (profiles) that this remarketing audience is linked to.
    - Each invocation of this argument appends the given value to the array.
* `name=diam`
    - The name of this remarketing audience.
* `state-based-audience-definition.exclude-conditions    exclusion-duration=takimata`
    - Whether to make the exclusion TEMPORARY or PERMANENT.
* `segment=lorem`
    - The segment condition that will cause a user to be removed from an audience.

* `..include-conditions    days-to-look-back=98`
    - The look-back window lets you specify a time frame for evaluating the behavior that qualifies users for your audience. For example, if your filters include users from Central Asia, and Transactions Greater than 2, and you set the look-back window to 14 days, then any user from Central Asia whose cumulative transactions exceed 2 during the last 14 days is added to the audience.
* `is-smart-list=false`
    - Boolean indicating whether this segment is a smart list. https://support.google.com/analytics/answer/4628577
* `kind=sed`
    - Resource type for include conditions.
* `membership-duration-days=92`
    - Number of days (in the range 1 to 540) a user remains in the audience.
* `segment=consetetur`
    - The segment condition that will cause a user to be added to an audience.


* `...    updated=sea`
    - Time this remarketing audience was last modified.
* `web-property-id=voluptua.`
    - Web property ID of the form UA-XXXXX-YY to which this remarketing audience belongs.


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
