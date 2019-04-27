Creates a spreadsheet, returning the newly created spreadsheet.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/drive*
* *https://www.googleapis.com/auth/drive.file*
* *https://www.googleapis.com/auth/spreadsheets*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/drive*.
You can set the scope for this method like this: `sheets4 --scope <scope> spreadsheets create ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Spreadsheet:
  properties:
    auto-recalc: string
    default-format:
      background-color:
        alpha: number
        blue: number
        green: number
        red: number
      borders:
        bottom:
          color:
            alpha: number
            blue: number
            green: number
            red: number
          style: string
          width: integer
        left:
          color:
            alpha: number
            blue: number
            green: number
            red: number
          style: string
          width: integer
        right:
          color:
            alpha: number
            blue: number
            green: number
            red: number
          style: string
          width: integer
        top:
          color:
            alpha: number
            blue: number
            green: number
            red: number
          style: string
          width: integer
      horizontal-alignment: string
      hyperlink-display-type: string
      number-format:
        pattern: string
        type: string
      padding:
        bottom: integer
        left: integer
        right: integer
        top: integer
      text-direction: string
      text-format:
        bold: boolean
        font-family: string
        font-size: integer
        foreground-color:
          alpha: number
          blue: number
          green: number
          red: number
        italic: boolean
        strikethrough: boolean
        underline: boolean
      text-rotation:
        angle: integer
        vertical: boolean
      vertical-alignment: string
      wrap-strategy: string
    iterative-calculation-settings:
      convergence-threshold: number
      max-iterations: integer
    locale: string
    time-zone: string
    title: string
  spreadsheet-id: string
  spreadsheet-url: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .properties    auto-recalc=sed`
    - The amount of time to wait before volatile functions are recalculated.
* `default-format.background-color    alpha=0.165687283689`
    - The fraction of this color that should be applied to the pixel. That is,
        the final pixel color is defined by the equation:
        
          pixel color = alpha * (this color) + (1.0 - alpha) * (background color)
        
        This means that a value of 1.0 corresponds to a solid color, whereas
        a value of 0.0 corresponds to a completely transparent color. This
        uses a wrapper message rather than a simple float scalar so that it is
        possible to distinguish between a default value and the value being unset.
        If omitted, this color object is to be rendered as a solid color
        (as if the alpha value had been explicitly given with a value of 1.0).
* `blue=0.824373746908`
    - The amount of blue in the color as a value in the interval [0, 1].
* `green=0.383704808614`
    - The amount of green in the color as a value in the interval [0, 1].
* `red=0.789612824916`
    - The amount of red in the color as a value in the interval [0, 1].

* `..borders.bottom.color    alpha=0.921726509896`
    - The fraction of this color that should be applied to the pixel. That is,
        the final pixel color is defined by the equation:
        
          pixel color = alpha * (this color) + (1.0 - alpha) * (background color)
        
        This means that a value of 1.0 corresponds to a solid color, whereas
        a value of 0.0 corresponds to a completely transparent color. This
        uses a wrapper message rather than a simple float scalar so that it is
        possible to distinguish between a default value and the value being unset.
        If omitted, this color object is to be rendered as a solid color
        (as if the alpha value had been explicitly given with a value of 1.0).
* `blue=0.30763338798`
    - The amount of blue in the color as a value in the interval [0, 1].
* `green=0.992247072586`
    - The amount of green in the color as a value in the interval [0, 1].
* `red=0.204946002078`
    - The amount of red in the color as a value in the interval [0, 1].

* `..    style=labore`
    - The style of the border.
* `width=92`
    - The width of the border, in pixels.
        Deprecated; the width is determined by the &#34;style&#34; field.

* `..left.color    alpha=0.109288397621`
    - The fraction of this color that should be applied to the pixel. That is,
        the final pixel color is defined by the equation:
        
          pixel color = alpha * (this color) + (1.0 - alpha) * (background color)
        
        This means that a value of 1.0 corresponds to a solid color, whereas
        a value of 0.0 corresponds to a completely transparent color. This
        uses a wrapper message rather than a simple float scalar so that it is
        possible to distinguish between a default value and the value being unset.
        If omitted, this color object is to be rendered as a solid color
        (as if the alpha value had been explicitly given with a value of 1.0).
* `blue=0.820437629783`
    - The amount of blue in the color as a value in the interval [0, 1].
* `green=0.398739054055`
    - The amount of green in the color as a value in the interval [0, 1].
* `red=0.0653431304201`
    - The amount of red in the color as a value in the interval [0, 1].

* `..    style=aliquyam`
    - The style of the border.
* `width=35`
    - The width of the border, in pixels.
        Deprecated; the width is determined by the &#34;style&#34; field.

* `..right.color    alpha=0.403587530578`
    - The fraction of this color that should be applied to the pixel. That is,
        the final pixel color is defined by the equation:
        
          pixel color = alpha * (this color) + (1.0 - alpha) * (background color)
        
        This means that a value of 1.0 corresponds to a solid color, whereas
        a value of 0.0 corresponds to a completely transparent color. This
        uses a wrapper message rather than a simple float scalar so that it is
        possible to distinguish between a default value and the value being unset.
        If omitted, this color object is to be rendered as a solid color
        (as if the alpha value had been explicitly given with a value of 1.0).
* `blue=0.801612546575`
    - The amount of blue in the color as a value in the interval [0, 1].
* `green=0.800065132311`
    - The amount of green in the color as a value in the interval [0, 1].
* `red=0.663748882844`
    - The amount of red in the color as a value in the interval [0, 1].

* `..    style=et`
    - The style of the border.
* `width=60`
    - The width of the border, in pixels.
        Deprecated; the width is determined by the &#34;style&#34; field.

* `..top.color    alpha=0.460933679688`
    - The fraction of this color that should be applied to the pixel. That is,
        the final pixel color is defined by the equation:
        
          pixel color = alpha * (this color) + (1.0 - alpha) * (background color)
        
        This means that a value of 1.0 corresponds to a solid color, whereas
        a value of 0.0 corresponds to a completely transparent color. This
        uses a wrapper message rather than a simple float scalar so that it is
        possible to distinguish between a default value and the value being unset.
        If omitted, this color object is to be rendered as a solid color
        (as if the alpha value had been explicitly given with a value of 1.0).
* `blue=0.957563386643`
    - The amount of blue in the color as a value in the interval [0, 1].
* `green=0.795720263212`
    - The amount of green in the color as a value in the interval [0, 1].
* `red=0.313727897996`
    - The amount of red in the color as a value in the interval [0, 1].

* `..    style=aliquyam`
    - The style of the border.
* `width=92`
    - The width of the border, in pixels.
        Deprecated; the width is determined by the &#34;style&#34; field.


* `...    horizontal-alignment=lorem`
    - The horizontal alignment of the value in the cell.
* `hyperlink-display-type=eos`
    - How a hyperlink, if it exists, should be displayed in the cell.
* `number-format    pattern=erat`
    - Pattern string used for formatting.  If not set, a default pattern based on
        the user&#39;s locale will be used if necessary for the given type.
        See the [Date and Number Formats guide](/sheets/api/guides/formats) for more
        information about the supported patterns.
* `type=sadipscing`
    - The type of the number format.
        When writing, this field must be set.

* `..padding    bottom=53`
    - The bottom padding of the cell.
* `left=62`
    - The left padding of the cell.
* `right=58`
    - The right padding of the cell.
* `top=4`
    - The top padding of the cell.

* `..    text-direction=no`
    - The direction of the text in the cell.
* `text-format    bold=true`
    - True if the text is bold.
* `font-family=eirmod`
    - The font family.
* `font-size=68`
    - The size of the font.
* `foreground-color    alpha=0.634997883415`
    - The fraction of this color that should be applied to the pixel. That is,
        the final pixel color is defined by the equation:
        
          pixel color = alpha * (this color) + (1.0 - alpha) * (background color)
        
        This means that a value of 1.0 corresponds to a solid color, whereas
        a value of 0.0 corresponds to a completely transparent color. This
        uses a wrapper message rather than a simple float scalar so that it is
        possible to distinguish between a default value and the value being unset.
        If omitted, this color object is to be rendered as a solid color
        (as if the alpha value had been explicitly given with a value of 1.0).
* `blue=0.197022051398`
    - The amount of blue in the color as a value in the interval [0, 1].
* `green=0.284925834809`
    - The amount of green in the color as a value in the interval [0, 1].
* `red=0.452328203239`
    - The amount of red in the color as a value in the interval [0, 1].

* `..    italic=true`
    - True if the text is italicized.
* `strikethrough=true`
    - True if the text has a strikethrough.
* `underline=false`
    - True if the text is underlined.

* `..text-rotation    angle=80`
    - The angle between the standard orientation and the desired orientation.
        Measured in degrees. Valid values are between -90 and 90. Positive
        angles are angled upwards, negative are angled downwards.
        
        Note: For LTR text direction positive angles are in the counterclockwise
        direction, whereas for RTL they are in the clockwise direction
* `vertical=true`
    - If true, text reads top to bottom, but the orientation of individual
        characters is unchanged.
        For example:
        
            | V |
            | e |
            | r |
            | t |
            | i |
            | c |
            | a |
            | l |

* `..    vertical-alignment=sanctus`
    - The vertical alignment of the value in the cell.
* `wrap-strategy=et`
    - The wrap strategy for the value in the cell.

* `..iterative-calculation-settings    convergence-threshold=0.549609244211`
    - When iterative calculation is enabled and successive results differ by
        less than this threshold value, the calculation rounds stop.
* `max-iterations=78`
    - When iterative calculation is enabled, the maximum number of calculation
        rounds to perform.

* `..    locale=consetetur`
    - The locale of the spreadsheet in one of the following formats:
        
        * an ISO 639-1 language code such as `en`
        
        * an ISO 639-2 language code such as `fil`, if no 639-1 code exists
        
        * a combination of the ISO language code and country code, such as `en_US`
        
        Note: when updating this field, not all locales/languages are supported.
* `time-zone=ut`
    - The time zone of the spreadsheet, in CLDR format such as
        `America/New_York`. If the time zone isn&#39;t recognized, this may
        be a custom time zone such as `GMT-07:00`.
* `title=ea`
    - The title of the spreadsheet.

* `..    spreadsheet-id=sed`
    - The ID of the spreadsheet.
        This field is read-only.
* `spreadsheet-url=dolor`
    - The url of the spreadsheet.
        This field is read-only.


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
