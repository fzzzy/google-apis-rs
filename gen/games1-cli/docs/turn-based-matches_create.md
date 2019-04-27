Create a turn-based match.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/games*
* *https://www.googleapis.com/auth/plus.login*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/games*.
You can set the scope for this method like this: `games1 --scope <scope> turn-based-matches create ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
TurnBasedMatchCreateRequest:
  auto-matching-criteria:
    exclusive-bitmask: string
    kind: string
    max-auto-matching-players: integer
    min-auto-matching-players: integer
  invited-player-ids: [string]
  kind: string
  request-id: string
  variant: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .auto-matching-criteria    exclusive-bitmask=sanctus`
    - A bitmask indicating when auto-matches are valid. When ANDed with other exclusive bitmasks, the result must be zero. Can be used to support exclusive roles within a game.
* `kind=et`
    - Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedAutoMatchingCriteria.
* `max-auto-matching-players=55`
    - The maximum number of players that should be added to the match by auto-matching.
* `min-auto-matching-players=78`
    - The minimum number of players that should be added to the match by auto-matching.

* `..    invited-player-ids=consetetur`
    - The player ids to invite to the match.
    - Each invocation of this argument appends the given value to the array.
* `kind=ut`
    - Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedMatchCreateRequest.
* `request-id=ea`
    - A randomly generated numeric ID. This number is used at the server to ensure that the request is handled correctly across retries.
* `variant=21`
    - The variant / mode of the application to be played. This can be any integer value, or left blank. You should use a small number of variants to keep the auto-matching pool as large as possible.


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

* **-p language=string**
    - The preferred language to use for strings returned by this method.

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
