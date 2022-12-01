const express = require('express')
const cors = require('cors')
const bodyParser = require('body-parser')
const formidableMiddleware = require('express-formidable')
const app = express()
const port = 26005

app.use(cors())

app.get('/get', (req, res) => {
    res.send(`received ${req.method} ${req.url}`)
})

app.post('/post', bodyParser.json(), (req, res) => {
    res.send(`received ${req.method} ${req.url} with ${JSON.stringify(req.body)}`)
})

app.post('/post-with-form', formidableMiddleware(), (req, res) => {
    res.send(`received ${req.method} ${req.url} with ${JSON.stringify(req.fields)}`)
})

app.put('/put', bodyParser.json(), (req, res) => {
    res.send(`received ${req.method} ${req.url} with ${JSON.stringify(req.body)}`)
})

app.listen(port, () => {
    console.log(`Example app listening on port ${port}`)
})