require('dotenv').config();
const express = require('express');
const axios = require('axios').default;

const app = express();
const port = process.env.PORT || 8080;

app.get("/", async (req, res) => {
    res.status(200).json({});
});

app.listen(port, () => {
    console.log(`Listening on ${port}...`);
});