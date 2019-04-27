Inserts a new placement group.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3 --scope <scope> placement-groups insert ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
PlacementGroup:
  account-id: string
  advertiser-id: string
  advertiser-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  archived: boolean
  campaign-id: string
  campaign-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  child-placement-ids: [string]
  comment: string
  content-category-id: string
  create-info:
    time: string
  directory-site-id: string
  directory-site-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  external-id: string
  id: string
  id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  kind: string
  last-modified-info:
    time: string
  name: string
  placement-group-type: string
  placement-strategy-id: string
  pricing-schedule:
    cap-cost-option: string
    disregard-overdelivery: boolean
    end-date: string
    flighted: boolean
    floodlight-activity-id: string
    pricing-type: string
    start-date: string
    testing-start-date: string
  primary-placement-id: string
  primary-placement-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  site-id: string
  site-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  subaccount-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=elitr`
    - Account ID of this placement group. This is a read-only field that can be left blank.
* `advertiser-id=sed`
    - Advertiser ID of this placement group. This is a required field on insertion.
* `advertiser-id-dimension-value    dimension-name=sit`
    - The name of the dimension.
* `etag=lorem`
    - The eTag of this response for caching purposes.
* `id=stet`
    - The ID associated with the value if available.
* `kind=clita`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=est`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=aliquyam`
    - The value of the dimension.

* `..    archived=false`
    - Whether this placement group is archived.
* `campaign-id=diam`
    - Campaign ID of this placement group. This field is required on insertion.
* `campaign-id-dimension-value    dimension-name=est`
    - The name of the dimension.
* `etag=labore`
    - The eTag of this response for caching purposes.
* `id=rebum.`
    - The ID associated with the value if available.
* `kind=stet`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=dolor`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=erat`
    - The value of the dimension.

* `..    child-placement-ids=sed`
    - IDs of placements which are assigned to this placement group. This is a read-only, auto-generated field.
    - Each invocation of this argument appends the given value to the array.
* `comment=dolore`
    - Comments for this placement group.
* `content-category-id=no`
    - ID of the content category assigned to this placement group.
* `create-info    time=amet`
    - Timestamp of the last change in milliseconds since epoch.

* `..    directory-site-id=et`
    - Directory site ID associated with this placement group. On insert, you must set either this field or the site_id field to specify the site associated with this placement group. This is a required field that is read-only after insertion.
* `directory-site-id-dimension-value    dimension-name=ipsum`
    - The name of the dimension.
* `etag=sea`
    - The eTag of this response for caching purposes.
* `id=elitr`
    - The ID associated with the value if available.
* `kind=aliquyam`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=vero`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=voluptua.`
    - The value of the dimension.

* `..    external-id=diam`
    - External ID for this placement.
* `id=aliquyam`
    - ID of this placement group. This is a read-only, auto-generated field.
* `id-dimension-value    dimension-name=labore`
    - The name of the dimension.
* `etag=ipsum`
    - The eTag of this response for caching purposes.
* `id=vero`
    - The ID associated with the value if available.
* `kind=aliquyam`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=dolor`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=no`
    - The value of the dimension.

* `..    kind=magna`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#placementGroup&#34;.
* `last-modified-info    time=dolor`
    - Timestamp of the last change in milliseconds since epoch.

* `..    name=sit`
    - Name of this placement group. This is a required field and must be less than 256 characters long.
* `placement-group-type=takimata`
    - Type of this placement group. A package is a simple group of placements that acts as a single pricing point for a group of tags. A roadblock is a group of placements that not only acts as a single pricing point, but also assumes that all the tags in it will be served at the same time. A roadblock requires one of its assigned placements to be marked as primary for reporting. This field is required on insertion.
* `placement-strategy-id=amet.`
    - ID of the placement strategy assigned to this placement group.
* `pricing-schedule    cap-cost-option=ipsum`
    - Placement cap cost option.
* `disregard-overdelivery=false`
    - Whether cap costs are ignored by ad serving.
* `end-date=tempor`
    - Placement end date. This date must be later than, or the same day as, the placement start date, but not later than the campaign end date. If, for example, you set 6/25/2015 as both the start and end dates, the effective placement date is just that day only, 6/25/2015. The hours, minutes, and seconds of the end date should not be set, as doing so will result in an error. This field is required on insertion.
* `flighted=false`
    - Whether this placement is flighted. If true, pricing periods will be computed automatically.
* `floodlight-activity-id=sit`
    - Floodlight activity ID associated with this placement. This field should be set when placement pricing type is set to PRICING_TYPE_CPA.
* `pricing-type=labore`
    - Placement pricing type. This field is required on insertion.
* `start-date=vero`
    - Placement start date. This date must be later than, or the same day as, the campaign start date. The hours, minutes, and seconds of the start date should not be set, as doing so will result in an error. This field is required on insertion.
* `testing-start-date=dolor`
    - Testing start date of this placement. The hours, minutes, and seconds of the start date should not be set, as doing so will result in an error.

* `..    primary-placement-id=lorem`
    - ID of the primary placement, used to calculate the media cost of a roadblock (placement group). Modifying this field will automatically modify the primary field on all affected roadblock child placements.
* `primary-placement-id-dimension-value    dimension-name=tempor`
    - The name of the dimension.
* `etag=vero`
    - The eTag of this response for caching purposes.
* `id=at`
    - The ID associated with the value if available.
* `kind=justo`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=sanctus`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=eirmod`
    - The value of the dimension.

* `..    site-id=sadipscing`
    - Site ID associated with this placement group. On insert, you must set either this field or the directorySiteId field to specify the site associated with this placement group. This is a required field that is read-only after insertion.
* `site-id-dimension-value    dimension-name=labore`
    - The name of the dimension.
* `etag=tempor`
    - The eTag of this response for caching purposes.
* `id=dolor`
    - The ID associated with the value if available.
* `kind=magna`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=eirmod`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=ea`
    - The value of the dimension.

* `..    subaccount-id=sadipscing`
    - Subaccount ID of this placement group. This is a read-only field that can be left blank.


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
