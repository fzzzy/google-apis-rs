Creates a managed short Dynamic Link given either a valid long Dynamic Link
or details such as Dynamic Link domain, Android and iOS app information.
The created short Dynamic Link will not expire.

This differs from CreateShortDynamicLink in the following ways:
  - The request will also contain a name for the link (non unique name
    for the front end).
  - The response must be authenticated with an auth token (generated with
    the admin service account).
  - The link will appear in the FDL list of links in the console front end.

The Dynamic Link domain in the request must be owned by requester&#39;s
Firebase project.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/firebase* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/firebase*.
You can set the scope for this method like this: `firebasedynamiclinks1 --scope <scope> managed-short-links create ...`
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CreateManagedShortLinkRequest:
  dynamic-link-info:
    analytics-info:
      google-play-analytics:
        gclid: string
        utm-campaign: string
        utm-content: string
        utm-medium: string
        utm-source: string
        utm-term: string
      itunes-connect-analytics:
        at: string
        ct: string
        mt: string
        pt: string
    android-info:
      android-fallback-link: string
      android-link: string
      android-min-package-version-code: string
      android-package-name: string
    desktop-info:
      desktop-fallback-link: string
    domain-uri-prefix: string
    dynamic-link-domain: string
    ios-info:
      ios-app-store-id: string
      ios-bundle-id: string
      ios-custom-scheme: string
      ios-fallback-link: string
      ios-ipad-bundle-id: string
      ios-ipad-fallback-link: string
    link: string
    navigation-info:
      enable-forced-redirect: boolean
    social-meta-tag-info:
      social-description: string
      social-image-link: string
      social-title: string
  long-dynamic-link: string
  name: string
  sdk-version: string
  suffix:
    custom-suffix: string
    option: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .dynamic-link-info.analytics-info.google-play-analytics    gclid=eirmod`
    - [AdWords autotagging parameter](https://support.google.com/analytics/answer/1033981?hl=en);
        used to measure Google AdWords ads. This value is generated dynamically
        and should never be modified.
* `utm-campaign=sit`
    - Campaign name; used for keyword analysis to identify a specific product
        promotion or strategic campaign.
* `utm-content=stet`
    - Campaign content; used for A/B testing and content-targeted ads to
        differentiate ads or links that point to the same URL.
* `utm-medium=sed`
    - Campaign medium; used to identify a medium such as email or cost-per-click.
* `utm-source=et`
    - Campaign source; used to identify a search engine, newsletter, or other
        source.
* `utm-term=dolores`
    - Campaign term; used with paid search to supply the keywords for ads.

* `..itunes-connect-analytics    at=kasd`
    - Affiliate token used to create affiliate-coded links.
* `ct=accusam`
    - Campaign text that developers can optionally add to any link in order to
        track sales from a specific marketing campaign.
* `mt=takimata`
    - iTune media types, including music, podcasts, audiobooks and so on.
* `pt=justo`
    - Provider token that enables analytics for Dynamic Links from within iTunes
        Connect.


* `...android-info    android-fallback-link=amet.`
    - Link to open on Android if the app is not installed.
* `android-link=erat`
    - If specified, this overrides the ‘link’ parameter on Android.
* `android-min-package-version-code=labore`
    - Minimum version code for the Android app. If the installed app’s version
        code is lower, then the user is taken to the Play Store.
* `android-package-name=sea`
    - Android package name of the app.

* `..desktop-info    desktop-fallback-link=nonumy`
    - Link to open on desktop.

* `..    domain-uri-prefix=dolores`
    - E.g. https://maps.app.goo.gl, https://maps.page.link, https://g.co/maps
        More examples can be found in description of getNormalizedUriPrefix in
        j/c/g/firebase/dynamiclinks/uri/DdlDomain.java
* `dynamic-link-domain=gubergren`
    - Dynamic Links domain that the project owns, e.g. abcd.app.goo.gl
        [Learn more](https://firebase.google.com/docs/dynamic-links/android/receive)
        on how to set up Dynamic Link domain associated with your Firebase project.
        
        Required.
* `ios-info    ios-app-store-id=sadipscing`
    - iOS App Store ID.
* `ios-bundle-id=aliquyam`
    - iOS bundle ID of the app.
* `ios-custom-scheme=ea`
    - Custom (destination) scheme to use for iOS. By default, we’ll use the
        bundle ID as the custom scheme. Developer can override this behavior using
        this param.
* `ios-fallback-link=no`
    - Link to open on iOS if the app is not installed.
* `ios-ipad-bundle-id=justo`
    - iPad bundle ID of the app.
* `ios-ipad-fallback-link=justo`
    - If specified, this overrides the ios_fallback_link value on iPads.

* `..    link=et`
    - The link your app will open, You can specify any URL your app can handle.
        This link must be a well-formatted URL, be properly URL-encoded, and use
        the HTTP or HTTPS scheme. See &#39;link&#39; parameters in the
        [documentation](https://firebase.google.com/docs/dynamic-links/create-manually).
        
        Required.
* `navigation-info    enable-forced-redirect=true`
    - If this option is on, FDL click will be forced to redirect rather than
        show an interstitial page.

* `..social-meta-tag-info    social-description=diam`
    - A short description of the link. Optional.
* `social-image-link=ipsum`
    - An image url string. Optional.
* `social-title=lorem`
    - Title to be displayed. Optional.


* `...    long-dynamic-link=et`
    - Full long Dynamic Link URL with desired query parameters specified.
        For example,
        &#34;https://sample.app.goo.gl/?link=http://www.google.com&amp;apn=com.sample&#34;,
        [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener).
* `name=duo`
    - Link name to associate with the link. It&#39;s used for marketer to identify
        manually-created links in the Firebase console
        (https://console.firebase.google.com/).
        Links must be named to be tracked.
* `sdk-version=aliquyam`
    - Google SDK version. Version takes the form &#34;$major.$minor.$patch&#34;
* `suffix    custom-suffix=sea`
    - Only applies to Option.CUSTOM.
* `option=lorem`
    - Suffix option.



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
