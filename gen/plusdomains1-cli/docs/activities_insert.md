Create a new activity for the authenticated user.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/plus.login*
* *https://www.googleapis.com/auth/plus.me*
* *https://www.googleapis.com/auth/plus.stream.write*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/plus.login*.
You can set the scope for this method like this: `plusdomains1 --scope <scope> activities insert ...`
# Required Scalar Argument
* **&lt;user-id&gt;** *(string)*
    - The ID of the user to create the activity on behalf of. Its value should be &#34;me&#34;, to indicate the authenticated user.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Activity:
  access:
    description: string
    domain-restricted: boolean
    kind: string
  actor:
    client-specific-actor-info:
      youtube-actor-info:
        channel-id: string
    display-name: string
    id: string
    image:
      url: string
    name:
      family-name: string
      given-name: string
    url: string
    verification:
      ad-hoc-verified: string
  address: string
  annotation: string
  crosspost-source: string
  etag: string
  geocode: string
  id: string
  kind: string
  location:
    address:
      formatted: string
    display-name: string
    id: string
    kind: string
    position:
      latitude: number
      longitude: number
  object:
    actor:
      client-specific-actor-info:
        youtube-actor-info:
          channel-id: string
      display-name: string
      id: string
      image:
        url: string
      url: string
      verification:
        ad-hoc-verified: string
    content: string
    id: string
    object-type: string
    original-content: string
    plusoners:
      self-link: string
      total-items: integer
    replies:
      self-link: string
      total-items: integer
    resharers:
      self-link: string
      total-items: integer
    status-for-viewer:
      can-comment: boolean
      can-plusone: boolean
      can-update: boolean
      is-plus-oned: boolean
      resharing-disabled: boolean
    url: string
  place-id: string
  place-name: string
  provider:
    title: string
  published: string
  radius: string
  title: string
  updated: string
  url: string
  verb: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .access    description=eirmod`
    - Description of the access granted, suitable for display.
* `domain-restricted=true`
    - Whether access is restricted to the domain.
* `kind=stet`
    - Identifies this resource as a collection of access controls. Value: &#34;plus#acl&#34;.

* `..actor.client-specific-actor-info.youtube-actor-info    channel-id=sed`
    - ID of the YouTube channel owned by the Actor.


* `...    display-name=et`
    - The name of the actor, suitable for display.
* `id=dolores`
    - The ID of the actor&#39;s Person resource.
* `image    url=kasd`
    - The URL of the actor&#39;s profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side.

* `..name    family-name=accusam`
    - The family name (&#34;last name&#34;) of the actor.
* `given-name=takimata`
    - The given name (&#34;first name&#34;) of the actor.

* `..    url=justo`
    - The link to the actor&#39;s Google profile.
* `verification    ad-hoc-verified=amet.`
    - Verification for one-time or manual processes.


* `...    address=erat`
    - Street address where this activity occurred.
* `annotation=labore`
    - Additional content added by the person who shared this activity, applicable only when resharing an activity.
* `crosspost-source=sea`
    - If this activity is a crosspost from another system, this property specifies the ID of the original activity.
* `etag=nonumy`
    - ETag of this response for caching purposes.
* `geocode=dolores`
    - Latitude and longitude where this activity occurred. Format is latitude followed by longitude, space separated.
* `id=gubergren`
    - The ID of this activity.
* `kind=sadipscing`
    - Identifies this resource as an activity. Value: &#34;plus#activity&#34;.
* `location.address    formatted=aliquyam`
    - The formatted address for display.

* `..    display-name=ea`
    - The display name of the place.
* `id=no`
    - The id of the place.
* `kind=justo`
    - Identifies this resource as a place. Value: &#34;plus#place&#34;.
* `position    latitude=0.800065132311`
    - The latitude of this position.
* `longitude=0.663748882844`
    - The longitude of this position.


* `...object.actor.client-specific-actor-info.youtube-actor-info    channel-id=et`
    - ID of the YouTube channel owned by the Actor.


* `...    display-name=diam`
    - The original actor&#39;s name, which is suitable for display.
* `id=ipsum`
    - ID of the original actor.
* `image    url=lorem`
    - A URL that points to a thumbnail photo of the original actor.

* `..    url=et`
    - A link to the original actor&#39;s Google profile.
* `verification    ad-hoc-verified=duo`
    - Verification for one-time or manual processes.


* `...    content=aliquyam`
    - The HTML-formatted content, which is suitable for display.
* `id=sea`
    - The ID of the object. When resharing an activity, this is the ID of the activity that is being reshared.
* `object-type=lorem`
    - The type of the object. Possible values include, but are not limited to, the following values:  
        - &#34;note&#34; - Textual content. 
        - &#34;activity&#34; - A Google+ activity.
* `original-content=eos`
    - The content (text) as provided by the author, which is stored without any HTML formatting. When creating or updating an activity, this value must be supplied as plain text in the request.
* `plusoners    self-link=erat`
    - The URL for the collection of people who +1&#39;d this activity.
* `total-items=6`
    - Total number of people who +1&#39;d this activity.

* `..replies    self-link=dolor`
    - The URL for the collection of comments in reply to this activity.
* `total-items=62`
    - Total number of comments on this activity.

* `..resharers    self-link=elitr`
    - The URL for the collection of resharers.
* `total-items=4`
    - Total number of people who reshared this activity.

* `..status-for-viewer    can-comment=false`
    - Whether the viewer can comment on the activity.
* `can-plusone=true`
    - Whether the viewer can +1 the activity.
* `can-update=true`
    - Whether the viewer can edit or delete the activity.
* `is-plus-oned=true`
    - Whether the viewer has +1&#39;d the activity.
* `resharing-disabled=true`
    - Whether reshares are disabled for the activity.

* `..    url=aliquyam`
    - The URL that points to the linked resource.

* `..    place-id=accusam`
    - ID of the place where this activity occurred.
* `place-name=lorem`
    - Name of the place where this activity occurred.
* `provider    title=sea`
    - Name of the service provider.

* `..    published=et`
    - The time at which this activity was initially published. Formatted as an RFC 3339 timestamp.
* `radius=duo`
    - Radius, in meters, of the region where this activity occurred, centered at the latitude and longitude identified in geocode.
* `title=et`
    - Title of this activity.
* `updated=eirmod`
    - The time at which this activity was last updated. Formatted as an RFC 3339 timestamp.
* `url=sanctus`
    - The link to this activity.
* `verb=et`
    - This activity&#39;s verb, which indicates the action that was performed. Possible values include, but are not limited to, the following values:  
        - &#34;post&#34; - Publish content to the stream. 
        - &#34;share&#34; - Reshare an activity.


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

* **-p preview=boolean**
    - If &#34;true&#34;, extract the potential media attachments for a URL. The response will include all possible attachments for a URL, including video, photos, and articles based on the content of the page.

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
