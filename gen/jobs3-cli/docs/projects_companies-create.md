Creates a new company entity.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/cloud-platform*
* *https://www.googleapis.com/auth/jobs*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/cloud-platform*.
You can set the scope for this method like this: `jobs3 --scope <scope> projects companies-create ...`
# Required Scalar Argument
* **&lt;parent&gt;** *(string)*
    - Required.
        
        Resource name of the project under which the company is created.
        
        The format is &#34;projects/{project_id}&#34;, for example,
        &#34;projects/api-test-project&#34;.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
CreateCompanyRequest:
  company:
    career-site-uri: string
    derived-info:
      headquarters-location:
        lat-lng:
          latitude: number
          longitude: number
        location-type: string
        postal-address:
          address-lines: [string]
          administrative-area: string
          language-code: string
          locality: string
          organization: string
          postal-code: string
          recipients: [string]
          region-code: string
          revision: integer
          sorting-code: string
          sublocality: string
        radius-in-miles: number
    display-name: string
    eeo-text: string
    external-id: string
    headquarters-address: string
    hiring-agency: boolean
    image-uri: string
    keyword-searchable-job-custom-attributes: [string]
    name: string
    size: string
    suspended: boolean
    website-uri: string

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .company    career-site-uri=eirmod`
    - Optional.
        
        The URI to employer&#39;s career site or careers page on the employer&#39;s web
        site, for example, &#34;https://careers.google.com&#34;.
* `derived-info.headquarters-location.lat-lng    latitude=0.533265573605`
    - The latitude in degrees. It must be in the range [-90.0, +90.0].
* `longitude=0.365848359249`
    - The longitude in degrees. It must be in the range [-180.0, +180.0].

* `..    location-type=sed`
    - The type of a location, which corresponds to the address lines field of
        PostalAddress. For example, &#34;Downtown, Atlanta, GA, USA&#34; has a type of
        LocationType#NEIGHBORHOOD, and &#34;Kansas City, KS, USA&#34; has a type of
        LocationType#LOCALITY.
* `postal-address    address-lines=et`
    - Unstructured address lines describing the lower levels of an address.
        
        Because values in address_lines do not have type information and may
        sometimes contain multiple values in a single field (e.g.
        &#34;Austin, TX&#34;), it is important that the line order is clear. The order of
        address lines should be &#34;envelope order&#34; for the country/region of the
        address. In places where this can vary (e.g. Japan), address_language is
        used to make it explicit (e.g. &#34;ja&#34; for large-to-small ordering and
        &#34;ja-Latn&#34; or &#34;en&#34; for small-to-large). This way, the most specific line of
        an address can be selected based on the language.
        
        The minimum permitted structural representation of an address consists
        of a region_code with all remaining information placed in the
        address_lines. It would be possible to format such an address very
        approximately without geocoding, but no semantic reasoning could be
        made about any of the address components until it was at least
        partially resolved.
        
        Creating an address only containing a region_code and address_lines, and
        then geocoding is the recommended way to handle completely unstructured
        addresses (as opposed to guessing which parts of the address should be
        localities or administrative areas).
    - Each invocation of this argument appends the given value to the array.
* `administrative-area=dolores`
    - Optional. Highest administrative subdivision which is used for postal
        addresses of a country or region.
        For example, this can be a state, a province, an oblast, or a prefecture.
        Specifically, for Spain this is the province and not the autonomous
        community (e.g. &#34;Barcelona&#34; and not &#34;Catalonia&#34;).
        Many countries don&#39;t use an administrative area in postal addresses. E.g.
        in Switzerland this should be left unpopulated.
* `language-code=kasd`
    - Optional. BCP-47 language code of the contents of this address (if
        known). This is often the UI language of the input form or is expected
        to match one of the languages used in the address&#39; country/region, or their
        transliterated equivalents.
        This can affect formatting in certain countries, but is not critical
        to the correctness of the data and will never affect any validation or
        other non-formatting related operations.
        
        If this value is not known, it should be omitted (rather than specifying a
        possibly incorrect default).
        
        Examples: &#34;zh-Hant&#34;, &#34;ja&#34;, &#34;ja-Latn&#34;, &#34;en&#34;.
* `locality=accusam`
    - Optional. Generally refers to the city/town portion of the address.
        Examples: US city, IT comune, UK post town.
        In regions of the world where localities are not well defined or do not fit
        into this structure well, leave locality empty and use address_lines.
* `organization=takimata`
    - Optional. The name of the organization at the address.
* `postal-code=justo`
    - Optional. Postal code of the address. Not all countries use or require
        postal codes to be present, but where they are used, they may trigger
        additional validation with other parts of the address (e.g. state/zip
        validation in the U.S.A.).
* `recipients=amet.`
    - Optional. The recipient at the address.
        This field may, under certain circumstances, contain multiline information.
        For example, it might contain &#34;care of&#34; information.
    - Each invocation of this argument appends the given value to the array.
* `region-code=erat`
    - Required. CLDR region code of the country/region of the address. This
        is never inferred and it is up to the user to ensure the value is
        correct. See http://cldr.unicode.org/ and
        http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html
        for details. Example: &#34;CH&#34; for Switzerland.
* `revision=66`
    - The schema revision of the `PostalAddress`. This must be set to 0, which is
        the latest revision.
        
        All new revisions **must** be backward compatible with old revisions.
* `sorting-code=sea`
    - Optional. Additional, country-specific, sorting code. This is not used
        in most regions. Where it is used, the value is either a string like
        &#34;CEDEX&#34;, optionally followed by a number (e.g. &#34;CEDEX 7&#34;), or just a number
        alone, representing the &#34;sector code&#34; (Jamaica), &#34;delivery area indicator&#34;
        (Malawi) or &#34;post office indicator&#34; (e.g. Côte d&#39;Ivoire).
* `sublocality=nonumy`
    - Optional. Sublocality of the address.
        For example, this can be neighborhoods, boroughs, districts.

* `..    radius-in-miles=0.820437629783`
    - Radius in meters of the job location. This value is derived from the
        location bounding box in which a circle with the specified radius
        centered from LatLng coves the area associated with the job location.
        For example, currently, &#34;Mountain View, CA, USA&#34; has a radius of
        6.17 miles.


* `...    display-name=gubergren`
    - Required.
        
        The display name of the company, for example, &#34;Google, LLC&#34;.
* `eeo-text=sadipscing`
    - Optional.
        
        Equal Employment Opportunity legal disclaimer text to be
        associated with all jobs, and typically to be displayed in all
        roles.
        
        The maximum number of allowed characters is 500.
* `external-id=aliquyam`
    - Required.
        
        Client side company identifier, used to uniquely identify the
        company.
        
        The maximum number of allowed characters is 255.
* `headquarters-address=ea`
    - Optional.
        
        The street address of the company&#39;s main headquarters, which may be
        different from the job location. The service attempts
        to geolocate the provided address, and populates a more specific
        location wherever possible in DerivedInfo.headquarters_location.
* `hiring-agency=false`
    - Optional.
        
        Set to true if it is the hiring agency that post jobs for other
        employers.
        
        Defaults to false if not provided.
* `image-uri=justo`
    - Optional.
        
        A URI that hosts the employer&#39;s company logo.
* `keyword-searchable-job-custom-attributes=justo`
    - Optional.
        
        A list of keys of filterable Job.custom_attributes, whose
        corresponding `string_values` are used in keyword search. Jobs with
        `string_values` under these specified field keys are returned if any
        of the values matches the search keyword. Custom field values with
        parenthesis, brackets and special symbols won&#39;t be properly searchable,
        and those keyword queries need to be surrounded by quotes.
    - Each invocation of this argument appends the given value to the array.
* `name=et`
    - Required during company update.
        
        The resource name for a company. This is generated by the service when a
        company is created.
        
        The format is &#34;projects/{project_id}/companies/{company_id}&#34;, for example,
        &#34;projects/api-test-project/companies/foo&#34;.
* `size=et`
    - Optional.
        
        The employer&#39;s company size.
* `suspended=true`
    - Output only. Indicates whether a company is flagged to be suspended from
        public availability by the service when job content appears suspicious,
        abusive, or spammy.
* `website-uri=ipsum`
    - Optional.
        
        The URI representing the company&#39;s primary web site or home page,
        for example, &#34;https://www.google.com&#34;.
        
        The maximum number of allowed characters is 255.



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
