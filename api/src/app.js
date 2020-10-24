require('dotenv').config();
const express = require('express');
const { gpt3 } = require("./gpt3");
const { getPrompt } = require("./db");


const app = express();
const port = process.env.PORT || 8080;


app.get("/", async (req, res) => {
    const input = req.query.q;
    const prompt = `${await getPrompt()}\n\nText: ${input}\nCommand:`;
    console.log(`Prompt is: ${prompt}`);
    const maxTokens = 100;
    const temp = 0.2;
    const stop = '\n\n';
    res.status(200).send(await gpt3(prompt, maxTokens, temp, stop));
});

app.listen(port, () => {
    console.log(`Listening on ${port}...`);
});