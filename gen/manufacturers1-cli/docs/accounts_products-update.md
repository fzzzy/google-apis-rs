Inserts or updates the attributes of the product in a Manufacturer Center
account.

Creates a product with the provided attributes. If the product already
exists, then all attributes are replaced with the new ones. The checks at
upload time are minimal. All required attributes need to be present for a
product to be valid. Issues may show up later after the API has accepted a
new upload for a product and it is possible to overwrite an existing valid
product with an invalid product. To detect this, you should retrieve the
product and check it for issues once the new version is available.

Uploaded attributes first need to be processed before they can be
retrieved. Until then, new products will be unavailable, and retrieval
of previously uploaded products will return the original state of the
product.
# Scopes

You will need authorization for the *https://www.googleapis.com/auth/manufacturercenter* scope to make a valid call.

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/manufacturercenter*.
You can set the scope for this method like this: `manufacturers1 --scope <scope> accounts products-update ...`
# Required Scalar Arguments
* **&lt;parent&gt;** *(string)*
    - Parent ID in the format `accounts/{account_id}`.
        
        `account_id` - The ID of the Manufacturer Center account.
* **&lt;name&gt;** *(string)*
    - Name in the format `{target_country}:{content_language}:{product_id}`.
        
        `target_country`   - The target country of the product as a CLDR territory
                             code (for example, US).
        
        `content_language` - The content language of the product as a two-letter
                             ISO 639-1 language code (for example, en).
        
        `product_id`     -   The ID of the product. For more information, see
                             https://support.google.com/manufacturers/answer/6124116#id.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Attributes:
  age-group: string
  brand: string
  capacity:
    unit: string
    value: string
  color: string
  count:
    unit: string
    value: string
  description: string
  disclosure-date: string
  excluded-destination: [string]
  flavor: string
  format: string
  gender: string
  gtin: [string]
  image-link:
    image-url: string
    status: string
    type: string
  included-destination: [string]
  item-group-id: string
  material: string
  mpn: string
  pattern: string
  product-line: string
  product-name: string
  product-page-url: string
  product-type: [string]
  release-date: string
  scent: string
  size: string
  size-system: string
  size-type: string
  suggested-retail-price:
    amount: string
    currency: string
  target-client-id: string
  theme: string
  title: string
  video-link: [string]

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .    age-group=eirmod`
    - The target age group of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#agegroup.
* `brand=sit`
    - The brand name of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#brand.
* `capacity    unit=stet`
    - The unit of the capacity, i.e., MB, GB, or TB.
* `value=sed`
    - The numeric value of the capacity.

* `..    color=et`
    - The color of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#color.
* `count    unit=dolores`
    - The unit in which these products are counted.
* `value=kasd`
    - The numeric value of the number of products in a package.

* `..    description=accusam`
    - The description of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#description.
* `disclosure-date=takimata`
    - The disclosure date of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#disclosure.
* `excluded-destination=justo`
    - A list of excluded destinations.
    - Each invocation of this argument appends the given value to the array.
* `flavor=amet.`
    - The flavor of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#flavor.
* `format=erat`
    - The format of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#format.
* `gender=labore`
    - The target gender of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#gender.
* `gtin=sea`
    - The Global Trade Item Number (GTIN) of the product. For more information,
        see https://support.google.com/manufacturers/answer/6124116#gtin.
    - Each invocation of this argument appends the given value to the array.
* `image-link    image-url=nonumy`
    - The URL of the image. For crawled images, this is the provided URL. For
        uploaded images, this is a serving URL from Google if the image has been
        processed successfully.
* `status=dolores`
    - The status of the image.
        @OutputOnly
* `type=gubergren`
    - The type of the image, i.e., crawled or uploaded.
        @OutputOnly

* `..    included-destination=sadipscing`
    - A list of included destinations.
    - Each invocation of this argument appends the given value to the array.
* `item-group-id=aliquyam`
    - The item group id of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#itemgroupid.
* `material=ea`
    - The material of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#material.
* `mpn=no`
    - The Manufacturer Part Number (MPN) of the product. For more information,
        see https://support.google.com/manufacturers/answer/6124116#mpn.
* `pattern=justo`
    - The pattern of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#pattern.
* `product-line=justo`
    - The name of the group of products related to the product. For more
        information, see
        https://support.google.com/manufacturers/answer/6124116#productline.
* `product-name=et`
    - The canonical name of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#productname.
* `product-page-url=et`
    - The URL of the detail page of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#productpage.
* `product-type=diam`
    - The type or category of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#producttype.
    - Each invocation of this argument appends the given value to the array.
* `release-date=ipsum`
    - The release date of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#release.
* `scent=lorem`
    - The scent of the product. For more information, see
         https://support.google.com/manufacturers/answer/6124116#scent.
* `size=et`
    - The size of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#size.
* `size-system=duo`
    - The size system of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#sizesystem.
* `size-type=aliquyam`
    - The size type of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#sizetype.
* `suggested-retail-price    amount=sea`
    - The numeric value of the price.
* `currency=lorem`
    - The currency in which the price is denoted.

* `..    target-client-id=eos`
    - The target client id. Should only be used in the accounts of the data
        partners.
* `theme=erat`
    - The theme of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#theme.
* `title=sadipscing`
    - The title of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#title.
* `video-link=dolor`
    - The videos of the product. For more information, see
        https://support.google.com/manufacturers/answer/6124116#video.
    - Each invocation of this argument appends the given value to the array.


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
