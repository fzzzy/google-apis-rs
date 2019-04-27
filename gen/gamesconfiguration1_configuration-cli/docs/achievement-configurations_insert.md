Insert a new achievement configuration in this application.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/androidpublisher* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/androidpublisher*.
You can set the scope for this method like this: `gamesconfiguration1-configuration --scope <scope> achievement-configurations insert ...`
# Required Scalar Argument
* **&lt;application-id&gt;** *(string)*
    - The application ID from the Google Play developer console.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
AchievementConfiguration:
  achievement-type: string
  draft:
    description:
      kind: string
    icon-url: string
    kind: string
    name:
      kind: string
    point-value: integer
    sort-rank: integer
  id: string
  initial-state: string
  kind: string
  published:
    description:
      kind: string
    icon-url: string
    kind: string
    name:
      kind: string
    point-value: integer
    sort-rank: integer
  steps-to-unlock: integer
  token: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    achievement-type=eirmod`
    - The type of the achievement.
        Possible values are:  
        - &#34;STANDARD&#34; - Achievement is either locked or unlocked. 
        - &#34;INCREMENTAL&#34; - Achievement is incremental.
* `draft.description    kind=sit`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..    icon-url=stet`
    - The icon url of this achievement. Writes to this field are ignored.
* `kind=sed`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#achievementConfigurationDetail.
* `name    kind=et`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..    point-value=83`
    - Point value for the achievement.
* `sort-rank=38`
    - The sort rank of this achievement. Writes to this field are ignored.

* `..    id=accusam`
    - The ID of the achievement.
* `initial-state=takimata`
    - The initial state of the achievement.
        Possible values are:  
        - &#34;HIDDEN&#34; - Achievement is hidden. 
        - &#34;REVEALED&#34; - Achievement is revealed. 
        - &#34;UNLOCKED&#34; - Achievement is unlocked.
* `kind=justo`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#achievementConfiguration.
* `published.description    kind=amet.`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..    icon-url=erat`
    - The icon url of this achievement. Writes to this field are ignored.
* `kind=labore`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#achievementConfigurationDetail.
* `name    kind=sea`
    - Uniquely identifies the type of this resource. Value is always the fixed string gamesConfiguration#localizedStringBundle.

* `..    point-value=11`
    - Point value for the achievement.
* `sort-rank=82`
    - The sort rank of this achievement. Writes to this field are ignored.

* `..    steps-to-unlock=40`
    - Steps to unlock. Only applicable to incremental achievements.
* `token=sadipscing`
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
