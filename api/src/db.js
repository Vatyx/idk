const { createClient } = require('@supabase/supabase-js')

const getPrompt = async () => {
  const SUPABASE_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTYwMzUwMjQ5MSwiZXhwIjoxOTE5MDc4NDkxfQ.z94jL3kJSO8awkd3h_yvdhsX5lsgmStFmQZShRBzoi4'
  const SUPABASE_URL = "https://bhlpgnzrnvrchycujafn.supabase.co"
  const supabase = createClient(SUPABASE_URL, SUPABASE_KEY);
  return (await supabase.from('Prompt').select('*')).body[0].content;
};

module.exports = {
  getPrompt
}