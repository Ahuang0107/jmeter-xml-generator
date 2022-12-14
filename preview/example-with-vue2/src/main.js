import Vue from 'vue'
import App from './App.vue'
import axios from 'axios'
import {ScriptBuilder} from "jmeter-xml-generator"

Vue.config.productionTip = false

const instance = axios.create({
    baseURL: 'http://localhost:26005/'
});

Vue.prototype.scriptBuilder = ScriptBuilder.new()

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
});

instance.get('/get', {params: {name: "elase"}}).then(res => console.log(res.data))
instance.post('/post', {name: "elase"}).then(res => console.log(res.data))
instance.postForm('/post-with-form', {name: "elase", age: 24}).then(res => console.log(res.data))
instance.put('/put', {name: "elase"}).then(res => console.log(res.data))

new Vue({
    render: h => h(App),
}).$mount('#app')
