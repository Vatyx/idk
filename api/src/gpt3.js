const axios = require('axios').default;

const gpt3 = async (prompt, maxTokens, temperature, stop) => {
  const gpt3Response = await axios.post(`https://api.openai.com/v1/engines/davinci-msft/completions`, {
    prompt,
    max_tokens: maxTokens,
    temperature,
    stop,
  },
    {
      headers: {
        'Authorization': `Bearer ${process.env.GPT3_KEY}`,
        'Content-Type': 'application/json'
      }
    });

  const raw = gpt3Response.data.choices[0].text.trim();
  if (raw.includes("Command: ")) {
    return raw.split("Command: ")[0];
  }
  return raw;
}

module.exports = {
  gpt3
}