const { supabaseClient } = require("./supabaseClient");

const getPrompt = async () => {
  return (await supabaseClient.from('Prompt').select('*')).body[0].content;
};

module.exports = {
  getPrompt
}