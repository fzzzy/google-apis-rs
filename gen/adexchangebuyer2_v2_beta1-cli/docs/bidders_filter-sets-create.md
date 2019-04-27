Creates the specified filter set for the account with the given account ID.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adexchange.buyer* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adexchange.buyer*.
You can set the scope for this method like this: `adexchangebuyer2-v2-beta1 --scope <scope> bidders filter-sets-create ...`
# Required Scalar Argument
* **&lt;owner-name&gt;** *(string)*
    - Name of the owner (bidder or account) of the filter set to be created.
        For example:
        
        - For a bidder-level filter set for bidder 123: `bidders/123`
        
        - For an account-level filter set for the buyer account representing bidder
          123: `bidders/123/accounts/123`
        
        - For an account-level filter set for the child seat buyer account 456
          whose bidder is 123: `bidders/123/accounts/456`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
FilterSet:
  absolute-date-range:
    end-date:
      day: integer
      month: integer
      year: integer
    start-date:
      day: integer
      month: integer
      year: integer
  creative-id: string
  deal-id: string
  environment: string
  formats: [string]
  name: string
  platforms: [string]
  publisher-identifiers: [string]
  realtime-time-range:
    start-timestamp: string
  relative-date-range:
    duration-days: integer
    offset-days: integer
  seller-network-ids: [integer]
  time-series-granularity: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .absolute-date-range.end-date    day=81`
    - Day of month. Must be from 1 to 31 and valid for the year and month, or 0
        if specifying a year by itself or a year and month where the day is not
        significant.
* `month=67`
    - Month of year. Must be from 1 to 12, or 0 if specifying a year without a
        month and day.
* `year=75`
    - Year of date. Must be from 1 to 9999, or 0 if specifying a date without
        a year.

* `..start-date    day=2`
    - Day of month. Must be from 1 to 31 and valid for the year and month, or 0
        if specifying a year by itself or a year and month where the day is not
        significant.
* `month=43`
    - Month of year. Must be from 1 to 12, or 0 if specifying a year without a
        month and day.
* `year=80`
    - Year of date. Must be from 1 to 9999, or 0 if specifying a date without
        a year.


* `...    creative-id=nonumy`
    - The ID of the creative on which to filter; optional. This field may be set
        only for a filter set that accesses account-level troubleshooting data,
        i.e., one whose name matches the `bidders/*/accounts/*/filterSets/*`
        pattern.
* `deal-id=et`
    - The ID of the deal on which to filter; optional. This field may be set
        only for a filter set that accesses account-level troubleshooting data,
        i.e., one whose name matches the `bidders/*/accounts/*/filterSets/*`
        pattern.
* `environment=sed`
    - The environment on which to filter; optional.
* `formats=no`
    - The list of formats on which to filter; may be empty. The filters
        represented by multiple formats are ORed together (i.e., if non-empty,
        results must match any one of the formats).
    - Each invocation of this argument appends the given value to the array.
* `name=invidunt`
    - A user-defined name of the filter set. Filter set names must be unique
        globally and match one of the patterns:
        
        - `bidders/*/filterSets/*` (for accessing bidder-level troubleshooting
        data)
        - `bidders/*/accounts/*/filterSets/*` (for accessing account-level
        troubleshooting data)
        
        This field is required in create operations.
* `platforms=rebum.`
    - The list of platforms on which to filter; may be empty. The filters
        represented by multiple platforms are ORed together (i.e., if non-empty,
        results must match any one of the platforms).
    - Each invocation of this argument appends the given value to the array.
* `publisher-identifiers=labore`
    - For Open Bidding partners only.
        The list of publisher identifiers on which to filter; may be empty.
        The filters represented by multiple publisher identifiers are ORed
        together.
    - Each invocation of this argument appends the given value to the array.
* `realtime-time-range    start-timestamp=aliquyam`
    - The start timestamp of the real-time RTB metrics aggregation.

* `..relative-date-range    duration-days=7`
    - The number of days in the requested date range, e.g., for a range spanning
        today: 1. For a range spanning the last 7 days: 7.
* `offset-days=55`
    - The end date of the filter set, specified as the number of days before
        today, e.g., for a range where the last date is today: 0.

* `..    seller-network-ids=92`
    - For Authorized Buyers only.
        The list of IDs of the seller (publisher) networks on which to filter;
        may be empty. The filters represented by multiple seller network IDs are
        ORed together (i.e., if non-empty, results must match any one of the
        publisher networks). See
        [seller-network-ids](https://developers.google.com/authorized-buyers/rtb/downloads/seller-network-ids)
        file for the set of existing seller network IDs.
    - Each invocation of this argument appends the given value to the array.
* `time-series-granularity=elitr`
    - The granularity of time intervals if a time series breakdown is desired;
        optional.


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
# Optional Method Properties

You may set the following properties to further configure the call. Please note that `-p` is followed by one 
or more key-value-pairs, and is called like this `-p k1=v1 k2=v2` even though the listing below repeats the
`-p` for completeness.

* **-p is-transient=boolean**
    - Whether the filter set is transient, or should be persisted indefinitely.
        By default, filter sets are not transient.
        If transient, it will be available for at least 1 hour after creation.

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
