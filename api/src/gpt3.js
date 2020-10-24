require('dotenv').config();

export default gpt3 = async (prompt, max_tokens, temperature, top_p = 1, n, logprobs, stop, frequency_penalty = 0, presence_penalty = 0, engine = 'davinci-msft') => {
  const gpt3Response = await axios.post(`https://api.openai.com/v1/engines/${engine}/completions`, {
    prompt,
    max_tokens,
    temperature,
    top_p,
    n,
    logprobs,
    stop,
    frequency_penalty,
    presence_penalty
  },
    {
      headers: {
        'Authorization': `Bearer ${process.env.GPT3_KEY}`,
        'Content-Type': 'application/json'
      }
    });

  console.log(gpt3.data);
  return gpt3Response.data;
}