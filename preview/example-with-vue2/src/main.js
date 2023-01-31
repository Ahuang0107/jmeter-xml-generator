import Vue from 'vue'
import App from './App.vue'
import axios from 'axios'
import {ScriptBuilder} from "jmeter-xml-generator"

Vue.config.productionTip = false

Vue.prototype.instance = axios.create({
    baseURL: 'http://localhost:26005/'
});

Vue.prototype.scriptBuilder = ScriptBuilder.new()

Vue.prototype.instance.interceptors.request.use(function (config) {
    console.log(JSON.stringify(config));
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

new Vue({
    render: h => h(App),
}).$mount('#app')
