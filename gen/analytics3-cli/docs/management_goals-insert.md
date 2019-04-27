Create a new goal.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/analytics.edit* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/analytics.edit*.
You can set the scope for this method like this: `analytics3 --scope <scope> management goals-insert ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - Account ID to create the goal for.
* **&lt;web-property-id&gt;** *(string)*
    - Web property ID to create the goal for.
* **&lt;profile-id&gt;** *(string)*
    - View (Profile) ID to create the goal for.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Goal:
  account-id: string
  active: boolean
  created: string
  event-details:
    use-event-value: boolean
  id: string
  internal-web-property-id: string
  kind: string
  name: string
  parent-link:
    href: string
    type: string
  profile-id: string
  self-link: string
  type: string
  updated: string
  url-destination-details:
    case-sensitive: boolean
    first-step-required: boolean
    match-type: string
    url: string
  value: number
  visit-num-pages-details:
    comparison-type: string
    comparison-value: string
  visit-time-on-site-details:
    comparison-type: string
    comparison-value: string
  web-property-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=kasd`
    - Account ID to which this goal belongs.
* `active=true`
    - Determines whether this goal is active.
* `created=sadipscing`
    - Time this goal was created.
* `event-details    use-event-value=false`
    - Determines if the event value should be used as the value for this goal.

* `..    id=clita`
    - Goal ID.
* `internal-web-property-id=ipsum`
    - Internal ID for the web property to which this goal belongs.
* `kind=dolor`
    - Resource type for an Analytics goal.
* `name=elitr`
    - Goal name.
* `parent-link    href=magna`
    - Link to the view (profile) to which this goal belongs.
* `type=aliquyam`
    - Value is &#34;analytics#profile&#34;.

* `..    profile-id=kasd`
    - View (Profile) ID to which this goal belongs.
* `self-link=duo`
    - Link for this goal.
* `type=et`
    - Goal type. Possible values are URL_DESTINATION, VISIT_TIME_ON_SITE, VISIT_NUM_PAGES, AND EVENT.
* `updated=sit`
    - Time this goal was last modified.
* `url-destination-details    case-sensitive=true`
    - Determines if the goal URL must exactly match the capitalization of visited URLs.
* `first-step-required=false`
    - Determines if the first step in this goal is required.
* `match-type=lorem`
    - Match type for the goal URL. Possible values are HEAD, EXACT, or REGEX.
* `url=sed`
    - URL for this goal.

* `..    value=0.12771503919`
    - Goal value.
* `visit-num-pages-details    comparison-type=clita`
    - Type of comparison. Possible values are LESS_THAN, GREATER_THAN, or EQUAL.
* `comparison-value=kasd`
    - Value used for this comparison.

* `..visit-time-on-site-details    comparison-type=elitr`
    - Type of comparison. Possible values are LESS_THAN or GREATER_THAN.
* `comparison-value=et`
    - Value used for this comparison.

* `..    web-property-id=vero`
    - Web property ID to which this goal belongs. The web property ID is of the form UA-XXXXX-YY.


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
