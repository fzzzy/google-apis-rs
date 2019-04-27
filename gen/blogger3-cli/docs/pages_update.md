Update a page.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/blogger* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/blogger*.
You can set the scope for this method like this: `blogger3 --scope <scope> pages update ...`
# Required Scalar Arguments
* **&lt;blog-id&gt;** *(string)*
    - The ID of the Blog.
* **&lt;page-id&gt;** *(string)*
    - The ID of the Page.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Page:
  author:
    display-name: string
    id: string
    image:
      url: string
    url: string
  blog:
    id: string
  content: string
  etag: string
  id: string
  kind: string
  published: string
  self-link: string
  status: string
  title: string
  updated: string
  url: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .author    display-name=aliquyam`
    - The display name.
* `id=sea`
    - The identifier of the Page creator.
* `image    url=lorem`
    - The page author&#39;s avatar URL.

* `..    url=eos`
    - The URL of the Page creator&#39;s Profile page.

* `..blog    id=erat`
    - The identifier of the blog containing this page.

* `..    content=sadipscing`
    - The body content of this Page, in HTML.
* `etag=dolor`
    - Etag of the resource.
* `id=eirmod`
    - The identifier for this resource.
* `kind=elitr`
    - The kind of this entity. Always blogger#page
* `published=amet`
    - RFC 3339 date-time when this Page was published.
* `self-link=no`
    - The API REST URL to fetch this resource from.
* `status=labore`
    - The status of the page for admin resources (either LIVE or DRAFT).
* `title=eirmod`
    - The title of this entity. This is the name displayed in the Admin user interface.
* `updated=dolore`
    - RFC 3339 date-time when this Page was last updated.
* `url=invidunt`
    - The URL that this Page is displayed at.


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

* **-p revert=boolean**
    - Whether a revert action should be performed when the page is updated (default: false).

* **-p publish=boolean**
    - Whether a publish action should be performed when the page is updated (default: false).

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
