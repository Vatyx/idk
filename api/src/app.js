require('dotenv').config();
const express = require('express');
const { gpt3 } = require("./gpt3");
const { getPrompt } = require("./db");
const {
  asyncWrapper,
  authenticate,
  errorHandler
} = require("./middleware");
const admin = require("firebase-admin");
const cors = require('cors')


const app = express();
const port = process.env.PORT || 8080;

app.use(cors({
  origin: "*"
}));
app.use(express.json());
app.use(authenticate);

app.get("/", asyncWrapper(async (req, res) => {
  const input = req.query.q;
  const { uid } = req.body;

  // TODO: does the user have enough credit to make this request?

  const prompt = `${await getPrompt()}\n\nText: ${input}\nCommand:`;
  console.log(`Prompt is: ${prompt}`);
  const maxTokens = 100;
  const temp = 0.2;
  const stop = '\n\n';
  res.status(200).send(await gpt3(prompt, maxTokens, temp, stop));
}));

app.post("/user", asyncWrapper(async (req, res) => {
  const { uid } = req.body;
  const userDoc = admin.firestore().collection("users").doc(uid);
  let user = await userDoc.get();
  if (!user.exists) {
    await userDoc.set(
      {
        ...req.body
      },
      {
        merge: true
      });
    user = await userDoc.get();
  }
  res.status(200).json(user.data());
}));

app.use(errorHandler); // Has to be at the very bottom.

app.listen(port, () => {
  console.log(`Listening on ${port}...`);
});