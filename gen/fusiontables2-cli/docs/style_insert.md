Adds a new style for the table.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/fusiontables* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/fusiontables*.
You can set the scope for this method like this: `fusiontables2 --scope <scope> style insert ...`
# Required Scalar Argument
* **&lt;table-id&gt;** *(string)*
    - Table for which a new style is being added
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
StyleSetting:
  kind: string
  marker-options:
    icon-name: string
    icon-styler:
      column-name: string
      gradient:
        max: number
        min: number
      kind: string
  name: string
  polygon-options:
    fill-color: string
    fill-color-styler:
      column-name: string
      gradient:
        max: number
        min: number
      kind: string
    fill-opacity: number
    stroke-color: string
    stroke-color-styler:
      column-name: string
      gradient:
        max: number
        min: number
      kind: string
    stroke-opacity: number
    stroke-weight: integer
    stroke-weight-styler:
      column-name: string
      gradient:
        max: number
        min: number
      kind: string
  polyline-options:
    stroke-color: string
    stroke-color-styler:
      column-name: string
      gradient:
        max: number
        min: number
      kind: string
    stroke-opacity: number
    stroke-weight: integer
    stroke-weight-styler:
      column-name: string
      gradient:
        max: number
        min: number
      kind: string
  style-id: integer
  table-id: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    kind=amet`
    - The kind of item this is. A StyleSetting contains the style definitions for points, lines, and polygons in a table. Since a table can have any one or all of them, a style definition can have point, line and polygon style definitions.
* `marker-options    icon-name=no`
    - Name of the icon. Use values defined in http://www.google.com/fusiontables/DataSource?dsrcid=308519
* `icon-styler    column-name=labore`
    - Name of the column whose value is used in the style.
* `gradient    max=0.61974554549`
    - Higher-end of the interpolation range: rows with this value will be assigned to colors[n-1].
* `min=0.675598874816`
    - Lower-end of the interpolation range: rows with this value will be assigned to colors[0].

* `..    kind=invidunt`
    - Stylers can be one of three kinds: &#34;fusiontables#fromColumn if the column value is to be used as is, i.e., the column values can have colors in #RRGGBBAA format or integer line widths or icon names; fusiontables#gradient if the styling of the row is to be based on applying the gradient function on the column value; or fusiontables#buckets if the styling is to based on the bucket into which the the column value falls.


* `...    name=aliquyam`
    - Optional name for the style setting.
* `polygon-options    fill-color=accusam`
    - Color of the interior of the polygon in #RRGGBB format.
* `fill-color-styler    column-name=lorem`
    - Name of the column whose value is used in the style.
* `gradient    max=0.916530817745`
    - Higher-end of the interpolation range: rows with this value will be assigned to colors[n-1].
* `min=0.798238825565`
    - Lower-end of the interpolation range: rows with this value will be assigned to colors[0].

* `..    kind=duo`
    - Stylers can be one of three kinds: &#34;fusiontables#fromColumn if the column value is to be used as is, i.e., the column values can have colors in #RRGGBBAA format or integer line widths or icon names; fusiontables#gradient if the styling of the row is to be based on applying the gradient function on the column value; or fusiontables#buckets if the styling is to based on the bucket into which the the column value falls.

* `..    fill-opacity=0.795407428867`
    - Opacity of the interior of the polygon: 0.0 (transparent) to 1.0 (opaque).
* `stroke-color=eirmod`
    - Color of the polygon border in #RRGGBB format.
* `stroke-color-styler    column-name=sanctus`
    - Name of the column whose value is used in the style.
* `gradient    max=0.790427604532`
    - Higher-end of the interpolation range: rows with this value will be assigned to colors[n-1].
* `min=0.549609244211`
    - Lower-end of the interpolation range: rows with this value will be assigned to colors[0].

* `..    kind=et`
    - Stylers can be one of three kinds: &#34;fusiontables#fromColumn if the column value is to be used as is, i.e., the column values can have colors in #RRGGBBAA format or integer line widths or icon names; fusiontables#gradient if the styling of the row is to be based on applying the gradient function on the column value; or fusiontables#buckets if the styling is to based on the bucket into which the the column value falls.

* `..    stroke-opacity=0.555410189379`
    - Opacity of the polygon border: 0.0 (transparent) to 1.0 (opaque).
* `stroke-weight=65`
    - Width of the polyon border in pixels.
* `stroke-weight-styler    column-name=ea`
    - Name of the column whose value is used in the style.
* `gradient    max=0.213650499369`
    - Higher-end of the interpolation range: rows with this value will be assigned to colors[n-1].
* `min=0.974461005485`
    - Lower-end of the interpolation range: rows with this value will be assigned to colors[0].

* `..    kind=dolor`
    - Stylers can be one of three kinds: &#34;fusiontables#fromColumn if the column value is to be used as is, i.e., the column values can have colors in #RRGGBBAA format or integer line widths or icon names; fusiontables#gradient if the styling of the row is to be based on applying the gradient function on the column value; or fusiontables#buckets if the styling is to based on the bucket into which the the column value falls.


* `...polyline-options    stroke-color=dolor`
    - Color of the line in #RRGGBB format.
* `stroke-color-styler    column-name=et`
    - Name of the column whose value is used in the style.
* `gradient    max=0.0504342224178`
    - Higher-end of the interpolation range: rows with this value will be assigned to colors[n-1].
* `min=0.494659563181`
    - Lower-end of the interpolation range: rows with this value will be assigned to colors[0].

* `..    kind=voluptua.`
    - Stylers can be one of three kinds: &#34;fusiontables#fromColumn if the column value is to be used as is, i.e., the column values can have colors in #RRGGBBAA format or integer line widths or icon names; fusiontables#gradient if the styling of the row is to be based on applying the gradient function on the column value; or fusiontables#buckets if the styling is to based on the bucket into which the the column value falls.

* `..    stroke-opacity=0.455185502569`
    - Opacity of the line : 0.0 (transparent) to 1.0 (opaque).
* `stroke-weight=90`
    - Width of the line in pixels.
* `stroke-weight-styler    column-name=justo`
    - Name of the column whose value is used in the style.
* `gradient    max=0.487587336766`
    - Higher-end of the interpolation range: rows with this value will be assigned to colors[n-1].
* `min=0.752180990981`
    - Lower-end of the interpolation range: rows with this value will be assigned to colors[0].

* `..    kind=diam`
    - Stylers can be one of three kinds: &#34;fusiontables#fromColumn if the column value is to be used as is, i.e., the column values can have colors in #RRGGBBAA format or integer line widths or icon names; fusiontables#gradient if the styling of the row is to be based on applying the gradient function on the column value; or fusiontables#buckets if the styling is to based on the bucket into which the the column value falls.


* `...    style-id=35`
    - Identifier for the style setting (unique only within tables).
* `table-id=consetetur`
    - Identifier for the table.


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
