<div style="display:flex;flex-direction: column;align-items: center">

  <h1><code>jmeter-xml-generator</code></h1>

  <p>
    <strong>Faster way to get jmeter script.</strong>
  </p>

<sub>Built with 🦀Rust</sub>
</div>

## Example

Hello world.

```ts
import {hello_world} from "jmeter-xml-generator";

hello_world();
```

Generate jmeter performance test script.

```ts
import {Request, ScriptBuilder} from "jmeter-xml-generator";

// 500 is threads number
const scriptBuilder = ScriptBuilder.new("custom host", "custom post", 500);
scriptBuilder.add_header("header key", "header value");
// GET request (delay 300 milliseconds)
scriptBuilder.get(
    "request url",
    JSON.stringify([
        {key: "query string parameter key", value: "query string parameter value"},
    ]),
    300
);
// POST request with multipart data
scriptBuilder.post_with_form_data(
    "request url",
    JSON.stringify([
        {key: "query string parameter key", value: "query string parameter value"},
    ]),
    300
);
// POST request with json body
scriptBuilder.post(
    "request url",
    JSON.stringify({
        current: 1,
        size: 15,
        status: 0,
    }),
    300
);
// PUT request with json body
scriptBuilder.put(
    "request url",
    JSON.stringify({
        current: 1,
        size: 15,
        status: 0,
    }),
    300
);

const result = scriptBuilder.build();
```