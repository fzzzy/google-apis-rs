Updates a survey. Currently the only property that can be updated is the owners property.
# Scopes

You will need authorization for at least one of the following scopes to make a valid call:

* *https://www.googleapis.com/auth/surveys*
* *https://www.googleapis.com/auth/userinfo.email*

If unset, the scope for this method defaults to *https://www.googleapis.com/auth/surveys*.
You can set the scope for this method like this: `surveys2 --scope <scope> surveys update ...`
# Required Scalar Argument
* **&lt;survey-url-id&gt;** *(string)*
    - External URL ID for the survey.
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
Survey:
  audience:
    ages: [string]
    country: string
    country-subdivision: string
    gender: string
    languages: [string]
    population-source: string
  cost:
    cost-per-response-nanos: string
    currency-code: string
    max-cost-per-response-nanos: string
    nanos: string
  customer-data: string
  description: string
  owners: [string]
  rejection-reason:
    explanation: string
    type: string
  state: string
  survey-url-id: string
  title: string
  wanted-response-count: integer

```

can be set completely with the following arguments which are assumed to be executed in the given order. Note how the cursor position is adjusted to the respective structures, allowing simple field names to be used most of the time.

* `-r .audience    ages=justo`
    - Optional list of age buckets to target. Supported age buckets are: [&#39;18-24&#39;, &#39;25-34&#39;, &#39;35-44&#39;, &#39;45-54&#39;, &#39;55-64&#39;, &#39;65+&#39;]
    - Each invocation of this argument appends the given value to the array.
* `country=justo`
    - Required country code that surveys should be targeted to. Accepts standard ISO 3166-1 2 character language codes. For instance, &#39;US&#39; for the United States, and &#39;GB&#39; for the United Kingdom.
* `country-subdivision=et`
    - Country subdivision (states/provinces/etc) that surveys should be targeted to. For all countries except GB, ISO-3166-2 subdivision code is required (eg. &#39;US-OH&#39; for Ohio, United States). For GB, NUTS 1 statistical region codes for the United Kingdom is required (eg. &#39;UK-UKC&#39; for North East England).
* `gender=et`
    - Optional gender to target.
* `languages=diam`
    - Language code that surveys should be targeted to. For instance, &#39;en-US&#39;. Surveys may target bilingual users by specifying a list of language codes (for example, &#39;de&#39; and &#39;en-US&#39;). In that case, all languages will be used for targeting users but the survey content (which is displayed) must match the first language listed. Accepts standard BCP47 language codes. See specification.
    - Each invocation of this argument appends the given value to the array.
* `population-source=ipsum`
    - Online population source where the respondents are sampled from.

* `..cost    cost-per-response-nanos=lorem`
    - Cost per survey response in nano units of the given currency. To get the total cost for a survey, multiply this value by wanted_response_count.
* `currency-code=et`
    - Currency code that the cost is given in.
* `max-cost-per-response-nanos=duo`
    - *Deprecated* Threshold to start a survey automatically if the quoted price is at most this value. When a survey has a Screener (threshold) question, it must go through an incidence pricing test to determine the final cost per response. Typically you will have to make a followup call to start the survey giving the final computed cost per response. If the survey has no threshold_answers, setting this property will return an error. By specifying this property, you indicate the max price per response you are willing to pay in advance of the incidence test. If the price turns out to be lower than the specified value, the survey will begin immediately and you will be charged at the rate determined by the incidence pricing test. If the price turns out to be greater than the specified value the survey will not be started and you will instead be notified what price was determined by the incidence test. At that point, you must raise the value of this property to be greater than or equal to that cost before attempting to start the survey again. This will immediately start the survey as long the incidence test was run within the last 21 days. This will no longer be available after June 2018.
* `nanos=aliquyam`
    - Cost of survey in nano units of the given currency. DEPRECATED in favor of cost_per_response_nanos

* `..    customer-data=sea`
    - Additional information to store on behalf of the API consumer and associate with this question. This binary blob is treated as opaque. This field is limited to 64K bytes.
* `description=lorem`
    - Text description of the survey.
* `owners=eos`
    - List of email addresses for survey owners. Must contain at least the address of the user making the API call.
    - Each invocation of this argument appends the given value to the array.
* `rejection-reason    explanation=erat`
    - A human-readable explanation of what was wrong with the survey.
* `type=sadipscing`
    - Which category of rejection this was. See the  Google Surveys Help Center for additional details on each category.

* `..    state=dolor`
    - State that the survey is in.
* `survey-url-id=eirmod`
    - Unique survey ID, that is viewable in the URL of the Survey Creator UI
* `title=elitr`
    - Optional name that will be given to the survey.
* `wanted-response-count=4`
    - Number of responses desired for the survey.


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
