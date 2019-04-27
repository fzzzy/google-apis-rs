Creates a new short URL.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/urlshortener* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/urlshortener*.
You can set the scope for this method like this: `urlshortener1 --scope <scope> url insert ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Url:
  analytics:
    all-time:
      long-url-clicks: string
      short-url-clicks: string
    day:
      long-url-clicks: string
      short-url-clicks: string
    month:
      long-url-clicks: string
      short-url-clicks: string
    two-hours:
      long-url-clicks: string
      short-url-clicks: string
    week:
      long-url-clicks: string
      short-url-clicks: string
  created: string
  id: string
  kind: string
  long-url: string
  status: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .analytics.all-time    long-url-clicks=eirmod`
    - Number of clicks on all goo.gl short URLs pointing to this long URL.
* `short-url-clicks=sit`
    - Number of clicks on this short URL.

* `..day    long-url-clicks=stet`
    - Number of clicks on all goo.gl short URLs pointing to this long URL.
* `short-url-clicks=sed`
    - Number of clicks on this short URL.

* `..month    long-url-clicks=et`
    - Number of clicks on all goo.gl short URLs pointing to this long URL.
* `short-url-clicks=dolores`
    - Number of clicks on this short URL.

* `..two-hours    long-url-clicks=kasd`
    - Number of clicks on all goo.gl short URLs pointing to this long URL.
* `short-url-clicks=accusam`
    - Number of clicks on this short URL.

* `..week    long-url-clicks=takimata`
    - Number of clicks on all goo.gl short URLs pointing to this long URL.
* `short-url-clicks=justo`
    - Number of clicks on this short URL.


* `...    created=amet.`
    - Time the short URL was created; ISO 8601 representation using the yyyy-MM-dd&#39;T&#39;HH:mm:ss.SSSZZ format, e.g. &#34;2010-10-14T19:01:24.944+00:00&#34;.
* `id=erat`
    - Short URL, e.g. &#34;http://goo.gl/l6MS&#34;.
* `kind=labore`
    - The fixed string &#34;urlshortener#url&#34;.
* `long-url=sea`
    - Long URL, e.g. &#34;http://www.google.com/&#34;. Might not be present if the status is &#34;REMOVED&#34;.
* `status=nonumy`
    - Status of the target URL. Possible values: &#34;OK&#34;, &#34;MALWARE&#34;, &#34;PHISHING&#34;, or &#34;REMOVED&#34;. A URL might be marked &#34;REMOVED&#34; if it was flagged as spam, for example.


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
