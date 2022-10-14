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
// GET request
scriptBuilder.add_request(
    Request.from(
        "request url",
        "request method(get)",
        false,
        JSON.stringify([
            {key: "query string parameter key", value: "query string parameter value"},
        ])
    )
);
// POST request with multipart data
scriptBuilder.add_request(
    Request.from(
        "request url",
        "request method(post)",
        true,
        JSON.stringify([
            {key: "query string parameter key", value: "query string parameter value"},
        ])
    )
);
// POST request with json body
scriptBuilder.add_request(
    Request.from(
        "request url",
        "request method(post)",
        false,
        JSON.stringify([]),
        JSON.stringify({
            current: 1,
            size: 15,
            status: 0,
        })
    )
);

const result = scriptBuilder.build();
```