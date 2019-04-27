Insert a new leaderboard configuration in this application.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidpublisher* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidpublisher*.
You can set the scope for this method like this: `gamesconfiguration1-configuration --scope <scope> leaderboard-configurations insert ...`
# Required Scalar Argument
* **&lt;application-id&gt;** *(string)*
    - The application ID from the Google Play developer console.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
LeaderboardConfiguration:
  draft:
    icon-url: string
    kind: string
    name:
      kind: string
    score-format:
      currency-code: string
      num-decimal-places: integer
      number-format-type: string
      suffix:
        few:
          kind: string
        many:
          kind: string
        one:
          kind: string
        other:
          kind: string
        two:
          kind: string
        zero:
          kind: string
    sort-rank: integer
  id: string
  kind: string
  published:
    icon-url: string
    kind: string
    name:
      kind: string
    score-format:
      currency-code: string
      num-decimal-places: integer
      number-format-type: string
      suffix:
        few:
          kind: string
        many:
          kind: string
        one:
          kind: string
        other:
          kind: string
        two:
          kind: string
        zero:
          kind: string
    sort-rank: integer
  score-max: string
  score-min: string
  score-order: string
  token: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .draft    icon-url=et`
    - The icon url of this leaderboard. Writes to this field are ignored.
* `kind=amet`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#leaderboardConfigurationDetail.
* `name    kind=et`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..score-format    currency-code=consetetur`
    - The curreny code string. Only used for CURRENCY format type.
* `num-decimal-places=65`
    - The number of decimal places for number. Only used for NUMERIC format type.
* `number-format-type=ea`
    - The formatting for the number.
        Possible values are:  
        - &#34;NUMERIC&#34; - Numbers are formatted to have no digits or a fixed number of digits after the decimal point according to locale. An optional custom unit can be added.
        - &#34;TIME_DURATION&#34; - Numbers are formatted to hours, minutes and seconds.
        - &#34;CURRENCY&#34; - Numbers are formatted to currency according to locale.
* `suffix.few    kind=sed`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..many    kind=dolor`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..one    kind=dolor`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..other    kind=dolor`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..two    kind=et`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..zero    kind=consetetur`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.



* `....    sort-rank=49`
    - The sort rank of this leaderboard. Writes to this field are ignored.

* `..    id=voluptua.`
    - The ID of the leaderboard.
* `kind=lorem`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#leaderboardConfiguration.
* `published    icon-url=gubergren`
    - The icon url of this leaderboard. Writes to this field are ignored.
* `kind=justo`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#leaderboardConfigurationDetail.
* `name    kind=sit`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..score-format    currency-code=vero`
    - The curreny code string. Only used for CURRENCY format type.
* `num-decimal-places=73`
    - The number of decimal places for number. Only used for NUMERIC format type.
* `number-format-type=rebum.`
    - The formatting for the number.
        Possible values are:  
        - &#34;NUMERIC&#34; - Numbers are formatted to have no digits or a fixed number of digits after the decimal point according to locale. An optional custom unit can be added.
        - &#34;TIME_DURATION&#34; - Numbers are formatted to hours, minutes and seconds.
        - &#34;CURRENCY&#34; - Numbers are formatted to currency according to locale.
* `suffix.few    kind=consetetur`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..many    kind=sadipscing`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..one    kind=vero`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..other    kind=sadipscing`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..two    kind=invidunt`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..zero    kind=consetetur`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.



* `....    sort-rank=17`
    - The sort rank of this leaderboard. Writes to this field are ignored.

* `..    score-max=duo`
    - Maximum score that can be posted to this leaderboard.
* `score-min=aliquyam`
    - Minimum score that can be posted to this leaderboard.
* `score-order=lorem`
    - The type of the leaderboard.
        Possible values are:  
        - &#34;LARGER_IS_BETTER&#34; - Larger scores posted are ranked higher. 
        - &#34;SMALLER_IS_BETTER&#34; - Smaller scores posted are ranked higher.
* `token=et`
    - The token for this resource.


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
