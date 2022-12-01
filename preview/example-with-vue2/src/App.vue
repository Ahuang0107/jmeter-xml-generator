<template>
  <div id="app">
    <img alt="Vue logo" src="./assets/logo.png">
    <HelloWorld msg="Welcome to Your Vue.js App"/>
    <button id="download-button" v-on:click="downloadScript">download jmeter script</button>
  </div>
</template>

<script>
import HelloWorld from './components/HelloWorld.vue'
import Vue from "vue";

export default {
  name: 'App',
  components: {
    HelloWorld
  },
  methods: {
    downloadScript() {
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
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}

#download-button {
  position: fixed;
  right: 20px;
  top: 40px;
}
</style>
