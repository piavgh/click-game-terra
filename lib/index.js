module.exports = ({ wallets, refs, config, client }) => ({
  getLength: () => client.query('clicker', { get_length: {} }),
  getName: () => client.query('clicker', { get_name: {} }),
  getScores: () => client.query("clicker", { get_scores: {} }),
  upsertScore: (score, signer = wallets.validator) =>
    client.execute(signer, "clicker", { upsert_score: { score } }),
})
