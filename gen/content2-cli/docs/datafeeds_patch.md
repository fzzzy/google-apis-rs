Updates a datafeed configuration of your Merchant Center account. This method supports patch semantics.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/content* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/content*.
You can set the scope for this method like this: `content2 --scope <scope> datafeeds patch ...`
# Required Scalar Arguments
* **&lt;merchant-id&gt;** *(string)*
    - The ID of the account that manages the datafeed. This account cannot be a multi-client account.
* **&lt;datafeed-id&gt;** *(string)*
    - The ID of the datafeed.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Datafeed:
  attribute-language: string
  content-language: string
  content-type: string
  fetch-schedule:
    day-of-month: integer
    fetch-url: string
    hour: integer
    minute-of-hour: integer
    password: string
    paused: boolean
    time-zone: string
    username: string
    weekday: string
  file-name: string
  format:
    column-delimiter: string
    file-encoding: string
    quoting-mode: string
  id: string
  intended-destinations: [string]
  kind: string
  name: string
  target-country: int64

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    attribute-language=duo`
    - The two-letter ISO 639-1 language in which the attributes are defined in the data feed.
* `content-language=aliquyam`
    - [DEPRECATED] Please use targets[].language instead. The two-letter ISO 639-1 language of the items in the feed. Must be a valid language for targetCountry.
* `content-type=lorem`
    - The type of data feed. For product inventory feeds, only feeds for local stores, not online stores, are supported.
* `fetch-schedule    day-of-month=84`
    - The day of the month the feed file should be fetched (1-31).
* `fetch-url=clita`
    - The URL where the feed file can be fetched. Google Merchant Center will support automatic scheduled uploads using the HTTP, HTTPS, FTP, or SFTP protocols, so the value will need to be a valid link using one of those four protocols.
* `hour=56`
    - The hour of the day the feed file should be fetched (0-23).
* `minute-of-hour=43`
    - The minute of the hour the feed file should be fetched (0-59). Read-only.
* `password=nonumy`
    - An optional password for fetch_url.
* `paused=true`
    - Whether the scheduled fetch is paused or not.
* `time-zone=sanctus`
    - Time zone used for schedule. UTC by default. E.g., &#34;America/Los_Angeles&#34;.
* `username=takimata`
    - An optional user name for fetch_url.
* `weekday=at`
    - The day of the week the feed file should be fetched.

* `..    file-name=labore`
    - The filename of the feed. All feeds must have a unique file name.
* `format    column-delimiter=invidunt`
    - Delimiter for the separation of values in a delimiter-separated values feed. If not specified, the delimiter will be auto-detected. Ignored for non-DSV data feeds.
* `file-encoding=ea`
    - Character encoding scheme of the data feed. If not specified, the encoding will be auto-detected.
* `quoting-mode=sadipscing`
    - Specifies how double quotes are interpreted. If not specified, the mode will be auto-detected. Ignored for non-DSV data feeds.

* `..    id=rebum.`
    - The ID of the data feed.
* `intended-destinations=dolore`
    - [DEPRECATED] Please use targets[].includedDestinations instead. The list of intended destinations (corresponds to checked check boxes in Merchant Center).
    - Each invocation of this argument appends the given value to the array.
* `kind=nonumy`
    - Identifies what kind of resource this is. Value: the fixed string &#34;content#datafeed&#34;.
* `name=sed`
    - A descriptive name of the data feed.
* `target-country=-82`
    - [DEPRECATED] Please use targets[].country instead. The country where the items in the feed will be included in the search index, represented as a CLDR territory code.


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

* **-p dry-run=boolean**
    - Flag to run the request in dry-run mode.

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
