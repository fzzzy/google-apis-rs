Update the supplied ad unit in the specified publisher AdSense account.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/adsensehost* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/adsensehost*.
You can set the scope for this method like this: `adsensehost4d1 --scope <scope> accounts adunits-update ...`
# Required Scalar Arguments
* **&lt;account-id&gt;** *(string)*
    - Account which contains the ad client.
* **&lt;ad-client-id&gt;** *(string)*
    - Ad client which contains the ad unit.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
AdUnit:
  code: string
  content-ads-settings:
    backup-option:
      color: string
      type: string
      url: string
    size: string
    type: string
  custom-style:
    colors:
      background: string
      border: string
      text: string
      title: string
      url: string
    corners: string
    font:
      family: string
      size: string
    kind: string
  id: string
  kind: string
  mobile-content-ads-settings:
    markup-language: string
    scripting-language: string
    size: string
    type: string
  name: string
  status: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    code=accusam`
    - Identity code of this ad unit, not necessarily unique across ad clients.
* `content-ads-settings.backup-option    color=lorem`
    - Color to use when type is set to COLOR. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash.
* `type=sea`
    - Type of the backup option. Possible values are BLANK, COLOR and URL.
* `url=et`
    - URL to use when type is set to URL.

* `..    size=duo`
    - Size of this ad unit. Size values are in the form SIZE_{width}_{height}.
* `type=et`
    - Type of this ad unit. Possible values are TEXT, TEXT_IMAGE, IMAGE and LINK.

* `..custom-style.colors    background=eirmod`
    - The color of the ad background.
* `border=sanctus`
    - The color of the ad border.
* `text=et`
    - The color of the ad text.
* `title=amet`
    - The color of the ad title.
* `url=et`
    - The color of the ad url.

* `..    corners=consetetur`
    - The style of the corners in the ad (deprecated: never populated, ignored).
* `font    family=ut`
    - The family of the font. Possible values are: ACCOUNT_DEFAULT_FAMILY, ADSENSE_DEFAULT_FAMILY, ARIAL, TIMES and VERDANA.
* `size=ea`
    - The size of the font. Possible values are: ACCOUNT_DEFAULT_SIZE, ADSENSE_DEFAULT_SIZE, SMALL, MEDIUM and LARGE.

* `..    kind=sed`
    - Kind this is, in this case adsensehost#adStyle.

* `..    id=dolor`
    - Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format.
* `kind=dolor`
    - Kind of resource this is, in this case adsensehost#adUnit.
* `mobile-content-ads-settings    markup-language=dolor`
    - The markup language to use for this ad unit.
* `scripting-language=et`
    - The scripting language to use for this ad unit.
* `size=consetetur`
    - Size of this ad unit.
* `type=amet.`
    - Type of this ad unit.

* `..    name=voluptua.`
    - Name of this ad unit.
* `status=lorem`
    - Status of this ad unit. Possible values are:
        NEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.
        
        ACTIVE: Indicates that there has been activity on this ad unit in the last seven days.
        
        INACTIVE: Indicates that there has been no activity on this ad unit in the last seven days.


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
