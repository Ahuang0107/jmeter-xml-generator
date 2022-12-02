<div style="display:flex;flex-direction: column;align-items: center">

  <h1><code>jmeter-xml-generator</code></h1>

  <p>
    <strong>Faster way to get jmeter script.</strong>
  </p>

<sub>Built with 🦀Rust</sub>
</div>

## Example

### Vue + Axios

You can see example in [example-with-vue2](/preview/example-with-vue2)

Generate jmeter performance test script with Vue and Axios.

1. install

   ```shell
   npm install jmeter-xml-generator@0.3.0
   ```

2. enable WebAssembly in Vue2

   ```js
   // vue.config.js
   const {defineConfig} = require('@vue/cli-service')
   module.exports = defineConfig({
      transpileDependencies: true,
      configureWebpack: {
         experiments: {
            asyncWebAssembly: true,
         }
      }
   })
   ```

3. bind scriptBuilder to Vue.prototype

    ```js
    import {ScriptBuilder} from "jmeter-xml-generator"
    
    Vue.prototype.scriptBuilder = ScriptBuilder.new()
    ```

4. add middleware to axios

    ```js
    const instance = axios.create({
        baseURL: 'http://localhost:26005/'
    })
    instance.interceptors.request.use(function (config) {
        const heads = {"Content-Type": config.headers["Content-Type"]}
        const requestArg = JSON.stringify({
            baseUrl: config.baseURL,
            url: config.url,
            method: config.method,
            heads,
            params: config.params,
            data: config.data,
        })
        Vue.prototype.scriptBuilder.add_axios_request(requestArg)
        return config
    })
    ```
5. download script

    ```js
    const result = Vue.prototype.scriptBuilder.build()
    let blob = new Blob([result], {type: "application/xml"})
    let blobUrl = URL.createObjectURL(blob)
    let linkDOM = document.createElement("a")
    linkDOM.style.display = "none"
    linkDOM.href = blobUrl
    linkDOM.setAttribute("download", "test script " + Date.now() + ".jmx")
    document.body.appendChild(linkDOM)
    linkDOM.click()
    document.body.removeChild(linkDOM)
    URL.revokeObjectURL(blobUrl)
    ```