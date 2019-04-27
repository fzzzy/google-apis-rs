Inserts a new advertiser.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3 --scope <scope> advertisers insert ...`
# Required Scalar Argument
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Advertiser:
  account-id: string
  advertiser-group-id: string
  click-through-url-suffix: string
  default-click-through-event-tag-id: string
  default-email: string
  floodlight-configuration-id: string
  floodlight-configuration-id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  id: string
  id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  kind: string
  name: string
  original-floodlight-configuration-id: string
  status: string
  subaccount-id: string
  suspended: boolean

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    account-id=vero`
    - Account ID of this advertiser.This is a read-only field that can be left blank.
* `advertiser-group-id=dolores`
    - ID of the advertiser group this advertiser belongs to. You can group advertisers for reporting purposes, allowing you to see aggregated information for all advertisers in each group.
* `click-through-url-suffix=magna`
    - Suffix added to click-through URL of ad creative associations under this advertiser. Must be less than 129 characters long.
* `default-click-through-event-tag-id=ut`
    - ID of the click-through event tag to apply by default to the landing pages of this advertiser&#39;s campaigns.
* `default-email=amet`
    - Default email address used in sender field for tag emails.
* `floodlight-configuration-id=sed`
    - Floodlight configuration ID of this advertiser. The floodlight configuration ID will be created automatically, so on insert this field should be left blank. This field can be set to another advertiser&#39;s floodlight configuration ID in order to share that advertiser&#39;s floodlight configuration with this advertiser, so long as: 
        - This advertiser&#39;s original floodlight configuration is not already associated with floodlight activities or floodlight activity groups. 
        - This advertiser&#39;s original floodlight configuration is not already shared with another advertiser.
* `floodlight-configuration-id-dimension-value    dimension-name=sit`
    - The name of the dimension.
* `etag=sit`
    - The eTag of this response for caching purposes.
* `id=dolores`
    - The ID associated with the value if available.
* `kind=et`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=sanctus`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=takimata`
    - The value of the dimension.

* `..    id=kasd`
    - ID of this advertiser. This is a read-only, auto-generated field.
* `id-dimension-value    dimension-name=ut`
    - The name of the dimension.
* `etag=sadipscing`
    - The eTag of this response for caching purposes.
* `id=et`
    - The ID associated with the value if available.
* `kind=clita`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=ipsum`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=dolor`
    - The value of the dimension.

* `..    kind=elitr`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#advertiser&#34;.
* `name=magna`
    - Name of this advertiser. This is a required field and must be less than 256 characters long and unique among advertisers of the same account.
* `original-floodlight-configuration-id=aliquyam`
    - Original floodlight configuration before any sharing occurred. Set the floodlightConfigurationId of this advertiser to originalFloodlightConfigurationId to unshare the advertiser&#39;s current floodlight configuration. You cannot unshare an advertiser&#39;s floodlight configuration if the shared configuration has activities associated with any campaign or placement.
* `status=kasd`
    - Status of this advertiser.
* `subaccount-id=duo`
    - Subaccount ID of this advertiser.This is a read-only field that can be left blank.
* `suspended=true`
    - Suspension status of this advertiser.


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
