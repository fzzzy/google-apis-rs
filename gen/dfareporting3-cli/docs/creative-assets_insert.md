Inserts a new creative asset.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/dfatrafficking* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/dfatrafficking*.
You can set the scope for this method like this: `dfareporting3 --scope <scope> creative-assets insert ...`
# Required Scalar Arguments
* **&lt;profile-id&gt;** *(string)*
    - User profile ID associated with this request.
* **&lt;advertiser-id&gt;** *(string)*
    - Advertiser ID of this creative. This is a required field.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CreativeAssetMetadata:
  asset-identifier:
    name: string
    type: string
  detected-features: [string]
  id: string
  id-dimension-value:
    dimension-name: string
    etag: string
    id: string
    kind: string
    match-type: string
    value: string
  kind: string
  warned-validation-rules: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .asset-identifier    name=invidunt`
    - Name of the creative asset. This is a required field while inserting an asset. After insertion, this assetIdentifier is used to identify the uploaded asset. Characters in the name must be alphanumeric or one of the following: &#34;.-_ &#34;. Spaces are allowed.
* `type=accusam`
    - Type of asset to upload. This is a required field. FLASH and IMAGE are no longer supported for new uploads. All image assets should use HTML_IMAGE.

* `..    detected-features=accusam`
    - List of feature dependencies for the creative asset that are detected by Campaign Manager. Feature dependencies are features that a browser must be able to support in order to render your HTML5 creative correctly. This is a read-only, auto-generated field.
    - Each invocation of this argument appends the given value to the array.
* `id=invidunt`
    - Numeric ID of the asset. This is a read-only, auto-generated field.
* `id-dimension-value    dimension-name=eirmod`
    - The name of the dimension.
* `etag=sit`
    - The eTag of this response for caching purposes.
* `id=elitr`
    - The ID associated with the value if available.
* `kind=amet`
    - The kind of resource this is, in this case dfareporting#dimensionValue.
* `match-type=ut`
    - Determines how the &#39;value&#39; field is matched when filtering. If not specified, defaults to EXACT. If set to WILDCARD_EXPRESSION, &#39;*&#39; is allowed as a placeholder for variable length character sequences, and it can be escaped with a backslash. Note, only paid search dimensions (&#39;dfa:paidSearch*&#39;) allow a matchType other than EXACT.
* `value=duo`
    - The value of the dimension.

* `..    kind=et`
    - Identifies what kind of resource this is. Value: the fixed string &#34;dfareporting#creativeAssetMetadata&#34;.
* `warned-validation-rules=est`
    - Rules validated during code generation that generated a warning. This is a read-only, auto-generated field.
        
        Possible values are:
        - &#34;ADMOB_REFERENCED&#34;
        - &#34;ASSET_FORMAT_UNSUPPORTED_DCM&#34;
        - &#34;ASSET_INVALID&#34;
        - &#34;CLICK_TAG_HARD_CODED&#34;
        - &#34;CLICK_TAG_INVALID&#34;
        - &#34;CLICK_TAG_IN_GWD&#34;
        - &#34;CLICK_TAG_MISSING&#34;
        - &#34;CLICK_TAG_MORE_THAN_ONE&#34;
        - &#34;CLICK_TAG_NON_TOP_LEVEL&#34;
        - &#34;COMPONENT_UNSUPPORTED_DCM&#34;
        - &#34;ENABLER_UNSUPPORTED_METHOD_DCM&#34;
        - &#34;EXTERNAL_FILE_REFERENCED&#34;
        - &#34;FILE_DETAIL_EMPTY&#34;
        - &#34;FILE_TYPE_INVALID&#34;
        - &#34;GWD_PROPERTIES_INVALID&#34;
        - &#34;HTML5_FEATURE_UNSUPPORTED&#34;
        - &#34;LINKED_FILE_NOT_FOUND&#34;
        - &#34;MAX_FLASH_VERSION_11&#34;
        - &#34;MRAID_REFERENCED&#34;
        - &#34;NOT_SSL_COMPLIANT&#34;
        - &#34;ORPHANED_ASSET&#34;
        - &#34;PRIMARY_HTML_MISSING&#34;
        - &#34;SVG_INVALID&#34;
        - &#34;ZIP_INVALID&#34;
    - Each invocation of this argument appends the given value to the array.


### About Cursors

The cursor position is key to comfortably set complex nested structures. The following rules apply:

* The cursor position is always set relative to the current one, unless the field name starts with the `.` character. Fields can be nested such as in `-r f.s.o` .
* The cursor position is set relative to the top-level structure if it starts with `.`, e.g. `-r .s.s`
* You can also set nested fields without setting the cursor explicitly. For example, to set a value relative to the current cursor position, you would specify `-r struct.sub_struct=bar`.
* You can move the cursor one level up by using `..`. Each additional `.` moves it up one additional level. E.g. `...` would go three levels up.

# Required Upload Flags

This method supports the upload of data, which *requires* all of the following flags to be set:

* **-u (simple|resumable)**
    - **simple** - Upload media all at once.
    - **resumable** - Upload media in a resumable fashion.
* **-f file**
    - Path to file to upload. It must be seekable.

The following flag *may* be set: 

* **-m mime**
    - the mime type, like 'application/octet-stream', which is the default


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
