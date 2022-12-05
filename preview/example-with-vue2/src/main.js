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

const imgDOMPos = [20, 40]
const imgDOM = document.createElement('img')
imgDOM.draggable = false
imgDOM.style.display = 'none'
imgDOM.src = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAB4AAAAeCAYAAAA7MK6iAAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAALEgAACxIB0t1+/AAAABZ0RVh0Q3JlYXRpb24gVGltZQAwMi8yNy8xM+5H/WsAAAAcdEVYdFNvZnR3YXJlAEFkb2JlIEZpcmV3b3JrcyBDUzbovLKMAAAD/0lEQVRIia3XW4hVdRQG8N85M85YmpcuXtJQLEy7IMbuKaqHLB+yTMgKUgqjoCKwBwmkt54iCHrI6kHsZsF0gYoeooisl6BdQVFEamVZad4dtbmfHtbadTyNM+r4hw37+v/WWt+3LrvWaDSMtoqiqKETg2VZ9p/gnXa0oa8sy1E3rY0EnIDt6MAEXIHLMBHj8rU+dGMbvsFh9GOoLMvBUwIuikLTxpfgZtyEGRjAQfyNRho0KQ38Cx/hA2xFf1mWfScFXBRFW5OHt2ANzsdn+BS/JMCRBJ6ECzAH1+EaHMNLeD0jMFSW5dBowJ24EA8l8Pd4AV9hJi7FPEzPT3bjJ2zHHxmh+9OAd/EsduFYM/ftLaD19G49lmADNuMiPIYiwYdQ8deGGvbga7yBdbgLDwt61mOnoOl4j1OVk/A4luNJvIUVuA/noleIqZFgms7HCeUfSmM35z7rBOdPoLvivJ6g8uPlWIbn8DYewaMYL7jqrQxuClR1Xql7XHq6NkP9PG7D7ehIDR0X6jlCSF8KYazASpEaAy1gw63q+SCO4g4R/tdwNVYJgW7DYD297cBSTMMmIZ416UX/SYC2GlCB340FeBHn4Ua0F0VRqwtepuBWfC6KwEpMPUlPRwKfnHt9hy9ELZiIznq+sAiz8XF6XQg+Twe0GbwHi0UmbMEszMdQvSzLAZGb/SIf54sUGLbinOLqF8VlHn7Oe5ejr54Xk3BAJPrFIkXO1Gok8F5R7aaUZakCbhdhOSpCPWhsYa5WxfV0Ud+7hZD/Ba4say4MZ3odF8UKuE8UiYmi9ra1vjgGsLbcc2ru39sMfFhIf7oQ2Jn0upZ7ThNa2l8UhXrW6O2i1M3O8z3+68djWR1CsFuFwBr4VuZxW17sxvX4XXSZ8cYW7oYoTqVQ9FL8mkbU6oLfg2JyuBYLRWs75PS5bohMOYAuUZAW4T2ROT31bM4DonUdxQP4UbS1s08DvCFo6sBGEcF7sSOd623O40ERgo1p3Z14VXg+oQl8JAOq5+0J3IV3RC9fKBrFzurlOuQ02J9A74t+ugzPiNGlH+ekFxWIlvNOodoePC3GpVViDOoSYW6UZYmWmSsVPldMC4vz45fFSLsy703z/9Gnjj+FkLrwGx7EaryJp7CvedxtBa7hLJHPa8WwtwWv4AfRPOblMSM/2yVScKtQb4F7cKXQyQbsb52xh5sy6+lB1UtXi6rzBT4RXWavUP1QPpspJpgluEqkzSZBW/dwg/0J/ySKougQnM7FDSIPZwlOj+QxKHidkvd3iDnrQ6FmJ/rlGe0Xpk1w2C7EtUDwPVkotybqwD5RdLaLYb5Pk5BOGbjJgJrwvjHcL0nObePzsmckwGr9A5V4ZzeOFB9pAAAAAElFTkSuQmCC'
imgDOM.style.position = 'fixed'
imgDOM.style.right = '20px'
imgDOM.style.top = '40px'
imgDOM.addEventListener('mouseover', () => {
    imgDOM.src = 'data:image/png;base64,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'
})
imgDOM.addEventListener('mouseleave', () => {
    imgDOM.src = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAB4AAAAeCAYAAAA7MK6iAAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAALEgAACxIB0t1+/AAAABZ0RVh0Q3JlYXRpb24gVGltZQAwMi8yNy8xM+5H/WsAAAAcdEVYdFNvZnR3YXJlAEFkb2JlIEZpcmV3b3JrcyBDUzbovLKMAAAD/0lEQVRIia3XW4hVdRQG8N85M85YmpcuXtJQLEy7IMbuKaqHLB+yTMgKUgqjoCKwBwmkt54iCHrI6kHsZsF0gYoeooisl6BdQVFEamVZad4dtbmfHtbadTyNM+r4hw37+v/WWt+3LrvWaDSMtoqiqKETg2VZ9p/gnXa0oa8sy1E3rY0EnIDt6MAEXIHLMBHj8rU+dGMbvsFh9GOoLMvBUwIuikLTxpfgZtyEGRjAQfyNRho0KQ38Cx/hA2xFf1mWfScFXBRFW5OHt2ANzsdn+BS/JMCRBJ6ECzAH1+EaHMNLeD0jMFSW5dBowJ24EA8l8Pd4AV9hJi7FPEzPT3bjJ2zHHxmh+9OAd/EsduFYM/ftLaD19G49lmADNuMiPIYiwYdQ8deGGvbga7yBdbgLDwt61mOnoOl4j1OVk/A4luNJvIUVuA/noleIqZFgms7HCeUfSmM35z7rBOdPoLvivJ6g8uPlWIbn8DYewaMYL7jqrQxuClR1Xql7XHq6NkP9PG7D7ehIDR0X6jlCSF8KYazASpEaAy1gw63q+SCO4g4R/tdwNVYJgW7DYD297cBSTMMmIZ416UX/SYC2GlCB340FeBHn4Ua0F0VRqwtepuBWfC6KwEpMPUlPRwKfnHt9hy9ELZiIznq+sAiz8XF6XQg+Twe0GbwHi0UmbMEszMdQvSzLAZGb/SIf54sUGLbinOLqF8VlHn7Oe5ejr54Xk3BAJPrFIkXO1Gok8F5R7aaUZakCbhdhOSpCPWhsYa5WxfV0Ud+7hZD/Ba4say4MZ3odF8UKuE8UiYmi9ra1vjgGsLbcc2ru39sMfFhIf7oQ2Jn0upZ7ThNa2l8UhXrW6O2i1M3O8z3+68djWR1CsFuFwBr4VuZxW17sxvX4XXSZ8cYW7oYoTqVQ9FL8mkbU6oLfg2JyuBYLRWs75PS5bohMOYAuUZAW4T2ROT31bM4DonUdxQP4UbS1s08DvCFo6sBGEcF7sSOd623O40ERgo1p3Z14VXg+oQl8JAOq5+0J3IV3RC9fKBrFzurlOuQ02J9A74t+ugzPiNGlH+ekFxWIlvNOodoePC3GpVViDOoSYW6UZYmWmSsVPldMC4vz45fFSLsy703z/9Gnjj+FkLrwGx7EaryJp7CvedxtBa7hLJHPa8WwtwWv4AfRPOblMSM/2yVScKtQb4F7cKXQyQbsb52xh5sy6+lB1UtXi6rzBT4RXWavUP1QPpspJpgluEqkzSZBW/dwg/0J/ySKougQnM7FDSIPZwlOj+QxKHidkvd3iDnrQ6FmJ/rlGe0Xpk1w2C7EtUDwPVkotybqwD5RdLaLYb5Pk5BOGbjJgJrwvjHcL0nObePzsmckwGr9A5V4ZzeOFB9pAAAAAElFTkSuQmCC'
})
let lastMousePos = undefined
let mouseMove = false
imgDOM.addEventListener('mousedown', (e) => {
    lastMousePos = [e.x, e.y]
})
document.addEventListener('mousemove', (e) => {
    if (lastMousePos !== undefined) {
        mouseMove = true
        const offsetX = e.x - lastMousePos[0]
        const offsetY = e.y - lastMousePos[1]
        imgDOMPos[0] = imgDOMPos[0] - offsetX
        imgDOMPos[1] = imgDOMPos[1] + offsetY
        const bounds = document.body.getBoundingClientRect()
        const {width, height} = bounds
        if (imgDOMPos[0] < width) {
            imgDOM.style.right = `${imgDOMPos[0]}px`
        }
        if (imgDOMPos[1] < height) {
            imgDOM.style.top = `${imgDOMPos[1]}px`
        }
        lastMousePos = [e.x, e.y]
    }
})
document.addEventListener('mouseup', () => {
    if (!mouseMove && lastMousePos !== undefined) {
        downloadScript()
    }
    lastMousePos = undefined
    mouseMove = false
})

document.body.appendChild(imgDOM)

document.addEventListener('keypress', (e) => {
    if (e.shiftKey && e.key === 'J') {
        if (imgDOM.style.display === 'none') {
            imgDOM.style.display = ''
        } else {
            imgDOM.style.display = 'none'
        }
    }
})

function downloadScript() {
    const result = Vue.prototype.scriptBuilder.build()
    let blob = new Blob([result], {type: "application/xml"})
    let blobUrl = URL.createObjectURL(blob)
    let linkDOM = document.createElement("a")
    linkDOM.style.display = "none"
    linkDOM.href = blobUrl
    linkDOM.setAttribute("download", "test plan " + Date.now() + ".jmx")
    document.body.appendChild(linkDOM)
    linkDOM.click()
    document.body.removeChild(linkDOM)
    URL.revokeObjectURL(blobUrl)
}

