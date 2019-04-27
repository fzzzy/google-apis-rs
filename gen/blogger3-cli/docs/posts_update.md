Update a post.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/blogger* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/blogger*.
You can set the scope for this method like this: `blogger3 --scope <scope> posts update ...`
# Required Scalar Arguments
* **&lt;blog-id&gt;** *(string)*
    - The ID of the Blog.
* **&lt;post-id&gt;** *(string)*
    - The ID of the Post.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Post:
  author:
    display-name: string
    id: string
    image:
      url: string
    url: string
  blog:
    id: string
  content: string
  custom-meta-data: string
  etag: string
  id: string
  kind: string
  labels: [string]
  location:
    lat: number
    lng: number
    name: string
    span: string
  published: string
  reader-comments: string
  replies:
    self-link: string
    total-items: string
  self-link: string
  status: string
  title: string
  title-link: string
  updated: string
  url: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .author    display-name=invidunt`
    - The display name.
* `id=ea`
    - The identifier of the Post creator.
* `image    url=sadipscing`
    - The Post author&#39;s avatar URL.

* `..    url=rebum.`
    - The URL of the Post creator&#39;s Profile page.

* `..blog    id=dolore`
    - The identifier of the Blog that contains this Post.

* `..    content=nonumy`
    - The content of the Post. May contain HTML markup.
* `custom-meta-data=sed`
    - The JSON meta-data for the Post.
* `etag=aliquyam`
    - Etag of the resource.
* `id=sit`
    - The identifier of this Post.
* `kind=eirmod`
    - The kind of this entity. Always blogger#post
* `labels=consetetur`
    - The list of labels this Post was tagged with.
    - Each invocation of this argument appends the given value to the array.
* `location    lat=0.159726330798`
    - Location&#39;s latitude.
* `lng=0.711397230561`
    - Location&#39;s longitude.
* `name=ea`
    - Location name.
* `span=gubergren`
    - Location&#39;s viewport span. Can be used when rendering a map preview.

* `..    published=aliquyam`
    - RFC 3339 date-time when this Post was published.
* `reader-comments=eos`
    - Comment control and display setting for readers of this post.
* `replies    self-link=tempor`
    - The URL of the comments on this post.
* `total-items=sea`
    - The count of comments on this post.

* `..    self-link=labore`
    - The API REST URL to fetch this resource from.
* `status=ipsum`
    - Status of the post. Only set for admin-level requests
* `title=aliquyam`
    - The title of the Post.
* `title-link=dolores`
    - The title link URL, similar to atom&#39;s related link.
* `updated=sit`
    - RFC 3339 date-time when this Post was last updated.
* `url=diam`
    - The URL where this Post is displayed.


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

* **-p publish=boolean**
    - Whether a publish action should be performed when the post is updated (default: false).

* **-p fetch-images=boolean**
    - Whether image URL metadata for each post is included in the returned result (default: false).

* **-p fetch-body=boolean**
    - Whether the body content of the post is included with the result (default: true).

* **-p revert=boolean**
    - Whether a revert action should be performed when the post is updated (default: false).

* **-p max-comments=integer**
    - Maximum number of comments to retrieve with the returned post.

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
