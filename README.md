[FastPay](https://arxiv.org/pdf/2003.11506) demo.

```
$ cargo run --bin demo
...
Valid transfer: OK
Reused nonce  : invalid receipt
Invalid nonce : missing receipt
Wrong balance : missing receipt
Double spend 2: missing receipt
Double spend 1: OK
```

```
$ cargo run --bin tx
...
TX: {
  "src": "030c7639a8bb42b796a7f5eb264e5c945f91efde90a2e5dee643f7b305521cb606",
  "dst": "02f01e6fd46d0a8a15d045278b015ab34c7bcc3a68e1360df570c14b4d9adbaf6f",
  "amt": 100,
  "seq": 0,
  "sig": "8e25deb1b716a71cc5f30867f8ba966c98df11003c0a71cb82a99da57fade6c305c7015ec7d6b24476d70a0eff7d840760285895d51919f41f22c0cf98162e7b"
}
---
ACK: {
  "key": "022d467ec738b1c62e47b985284173db02488c320e37b3890792201b1f528606b5",
  "sig": "e219184fdbd803914cfbc03b9c06f8d777bf136136e76d21d243231100e8615c1bc5bfd213ec168c588fed77f07eea03bcae9a511632cfc86eac0e17b9c3d7a1"
}
ACK: {
  "key": "03423fa48584c9940edf7ff853bba39be36cb3784984e8357bfcfbf0638dcc8a8d",
  "sig": "449a313935e5c1aa6467e910230135f8f4a9920cc768af27b0d18cfaa08bdfa00241ed14252c8b6471f3a301831de4021cce31f5344ee58292655a3b50eb9a77"
}
ACK: {
  "key": "036ba874552657dd18bee78d6cb2acfbd310df1fcba5a4617f365304fea2d4c8b4",
  "sig": "48462c15477ebe37b004533ef45693e2ade7dd9b76f6e995bb086704fe5d09ca1e7bd130e800351cc82ada715107bb349ba21a814129ee1477586f1bece37a77"
}
ACK: {
  "key": "03982617a145e4423645562b711d18f44e9defb5c91414f49b618e6039b821a0a6",
  "sig": "df5fef0aef745ad9f50f2c688318dab8ca964d9dd209cd339ba2c2c5f4e0cde371a407cbcbb206f380104c588e9921e6daeae06597d3f0e66bb9a6afedc488fa"
}
ACK: {
  "key": "0337622658448e2d6e9019d31b4a4dc145a2e79d113ca352dbebee529c5277ce4e",
  "sig": "aea82e1b65a8e1b5b3d417bc5c789ec1fc3c65edf894ffde3b7e6511b7f995fb6419fe2537d1c8b8d3e7c9cccf563ef9908ab50dd0bb2bc48f71d4c5c24ec15c"
}
ACK: {
  "key": "03dc5f3e8e5e70f573104620a98f600ca8306110c87e1dfb6fb9ba51826577ee1b",
  "sig": "6866427a7a9e19c95a7baffa2d87cdb6adef6e299046ef9aed0de0dc5ce5b14019682e11d8cf62e799f9d2a38156a20623720c2f53e2f1f1cd805dbab6b622e4"
}
ACK: {
  "key": "026a00db7a9f035a2db9460191826c12ebb8b30b46cbb74297929e2ce4ea79701b",
  "sig": "852da4585561c1acbf1fe164718e4652b00d4b3172d3bd27d94b95a94b699d8648b29077f77ef8416ebb2dab72702e7f5e90d3f9671aac14db06680076f38349"
}
---
RX: {
  "tx": {
    "src": "030c7639a8bb42b796a7f5eb264e5c945f91efde90a2e5dee643f7b305521cb606",
    "dst": "02f01e6fd46d0a8a15d045278b015ab34c7bcc3a68e1360df570c14b4d9adbaf6f",
    "amt": 100,
    "seq": 0,
    "sig": "8e25deb1b716a71cc5f30867f8ba966c98df11003c0a71cb82a99da57fade6c305c7015ec7d6b24476d70a0eff7d840760285895d51919f41f22c0cf98162e7b"
  },
  "acks": [
    {
      "key": "022d467ec738b1c62e47b985284173db02488c320e37b3890792201b1f528606b5",
      "sig": "e219184fdbd803914cfbc03b9c06f8d777bf136136e76d21d243231100e8615c1bc5bfd213ec168c588fed77f07eea03bcae9a511632cfc86eac0e17b9c3d7a1"
    },
    {
      "key": "03423fa48584c9940edf7ff853bba39be36cb3784984e8357bfcfbf0638dcc8a8d",
      "sig": "449a313935e5c1aa6467e910230135f8f4a9920cc768af27b0d18cfaa08bdfa00241ed14252c8b6471f3a301831de4021cce31f5344ee58292655a3b50eb9a77"
    },
    {
      "key": "036ba874552657dd18bee78d6cb2acfbd310df1fcba5a4617f365304fea2d4c8b4",
      "sig": "48462c15477ebe37b004533ef45693e2ade7dd9b76f6e995bb086704fe5d09ca1e7bd130e800351cc82ada715107bb349ba21a814129ee1477586f1bece37a77"
    },
    {
      "key": "03982617a145e4423645562b711d18f44e9defb5c91414f49b618e6039b821a0a6",
      "sig": "df5fef0aef745ad9f50f2c688318dab8ca964d9dd209cd339ba2c2c5f4e0cde371a407cbcbb206f380104c588e9921e6daeae06597d3f0e66bb9a6afedc488fa"
    },
    {
      "key": "0337622658448e2d6e9019d31b4a4dc145a2e79d113ca352dbebee529c5277ce4e",
      "sig": "aea82e1b65a8e1b5b3d417bc5c789ec1fc3c65edf894ffde3b7e6511b7f995fb6419fe2537d1c8b8d3e7c9cccf563ef9908ab50dd0bb2bc48f71d4c5c24ec15c"
    },
    {
      "key": "03dc5f3e8e5e70f573104620a98f600ca8306110c87e1dfb6fb9ba51826577ee1b",
      "sig": "6866427a7a9e19c95a7baffa2d87cdb6adef6e299046ef9aed0de0dc5ce5b14019682e11d8cf62e799f9d2a38156a20623720c2f53e2f1f1cd805dbab6b622e4"
    },
    {
      "key": "026a00db7a9f035a2db9460191826c12ebb8b30b46cbb74297929e2ce4ea79701b",
      "sig": "852da4585561c1acbf1fe164718e4652b00d4b3172d3bd27d94b95a94b699d8648b29077f77ef8416ebb2dab72702e7f5e90d3f9671aac14db06680076f38349"
    }
  ]
}
---
TX OK
TX confirmed by 022d467ec738b1c62e47b985284173db02488c320e37b3890792201b1f528606b5
TX confirmed by 03423fa48584c9940edf7ff853bba39be36cb3784984e8357bfcfbf0638dcc8a8d
TX confirmed by 036ba874552657dd18bee78d6cb2acfbd310df1fcba5a4617f365304fea2d4c8b4
TX confirmed by 03982617a145e4423645562b711d18f44e9defb5c91414f49b618e6039b821a0a6
TX confirmed by 0337622658448e2d6e9019d31b4a4dc145a2e79d113ca352dbebee529c5277ce4e
TX confirmed by 03dc5f3e8e5e70f573104620a98f600ca8306110c87e1dfb6fb9ba51826577ee1b
TX confirmed by 026a00db7a9f035a2db9460191826c12ebb8b30b46cbb74297929e2ce4ea79701b
---
seq:1 src:100 dst:300 | 022d467ec738b1c62e47b985284173db02488c320e37b3890792201b1f528606b5
seq:1 src:100 dst:300 | 03423fa48584c9940edf7ff853bba39be36cb3784984e8357bfcfbf0638dcc8a8d
seq:1 src:100 dst:300 | 036ba874552657dd18bee78d6cb2acfbd310df1fcba5a4617f365304fea2d4c8b4
seq:1 src:100 dst:300 | 03982617a145e4423645562b711d18f44e9defb5c91414f49b618e6039b821a0a6
seq:1 src:100 dst:300 | 0337622658448e2d6e9019d31b4a4dc145a2e79d113ca352dbebee529c5277ce4e
seq:1 src:100 dst:300 | 03dc5f3e8e5e70f573104620a98f600ca8306110c87e1dfb6fb9ba51826577ee1b
seq:1 src:100 dst:300 | 026a00db7a9f035a2db9460191826c12ebb8b30b46cbb74297929e2ce4ea79701b
```

---

### Consistency

No cross-authorities communications (and thus effectively all state change propagation is a responsibility of a client) in practice means "eventual consistency" accross authorities. So yes, authorities can have different view of the system which needs to be resolved by client via propagating state changes proactively to all authorities and taking into account state quorum (for `f` Byzantine authorities the quorum is at least `2f+1` out of total `3f+1` authorities). This is the foundation of *liveness* property of FastPay.

### Double-spend

If a client tries to double-spend by submitting different transactions with the same nonce/sequence, only one of them will get accepted (assuming it is a valid one: sufficient funds, valid signatures etc) by the quorum of authorities (in practice meaning just reaching necessary number of confirmations) and another one will get rejected (will never reach necessary number of confirmations). It is a race condition, so there is no way to deterministically predict which one will get accepted or rejected, but the important thing is that only one of them will. This is basically a *safety* property of FastPay.

### Participation

In Bitcoin due to Proof-of-Work consensus alrorithm, such process is trustless (no need to trust the miner, as the block validity and eligibility can be verified by anyone) so the guarantees are probabilistic in nature (the heaviest chain wins) and cryptographic that in turn fall back to computational/economic (costs of computing power). 

In Ethereum and pretty much all other Proof-of-Stake-based networks the validator (authority) ensures in can be trusted by locking the "stake" (that can be penalized in case of inconsistent behavior), effectively relying only on economic guarantees (the validator is incentivized to follow the rules to get rewards and disincentivized to break the rules by the rist of partial/complete loss of the stake).

Extending the protocol from supporting only static pre-defined trust-set (authorities' private keys, we can consider it a necessary "trusted setup") to support a dynamic one ("open participation") will require a way to ensure trust for the onboarded authorities, respectfully revoke trust from the offboarded one (thus allowing rotation of auhtorities) as well as some kind of "punishments" for misbehaving/faulty ones. Seems like a task for a consensus algorithm (all parties must agree on something - in this case active set of authorities or a penalty to a specific authority based on specific evidence). Taking into account the payment settlement context, the risks that participants take are rather of economical nature, and it would be reasonable to expect that authorities need to somehow share the risk (e.g. there must be a way to force a refund based on evidence of an accepted invalid payment). So some kind of collateral ("stake") seems like a reasoable ask for authority to ensure economic risks and shared between participants and authorities.

### Extension (EVM?)

The FastPay protocol is just basically a lightweight two-phase commit (with a centralised committer), it has a lot of simplifications and trade-offs baked in. While it is in theory possible to build a Turing-complete EVM-compatible solution on top of it, it would require:
- provide runtime & smart-contract build & debug tooling (compiler, VM, etc)
- address "Halt Problem" (likely by introducing a concept of *gas*)
- advanced state transition verification (Patricia-Merkle tree roots & proofs)
- address lack of such concept as a block in general (maybe lightweight snapshots)

In such solution, a transaction is merely a smart-contract call, which in turn is essentially a deterministic batch update to key-value datastore (smart-contract state) and associated data structures (Merkle-Patricia tree root as part of the smart-contract state). The sender can submit signed envelope containing target contract address and input parameters, and then collect signed confirmations (state-change summaries or failed transaction confirmations) from the authorities. Once required number of confirmations is received, the sender propagetes confirmations to all authorities that verify all the confirmantions and apply state changes. To improve security, the sender might even include ZK-proof of smart-contract invocation trace (e.g. SNARK or recursive SNARK for cross-contract calls) and authorities can validate it and sign the confirmations.

The significant downside of such approach at scale is inevitable cross-authority data delay caused by state-change propagation being the responsibility of a client/proxy/gateway/etc which can stop/fail/disconnect/etc. But such delay can be caused for specific address/account at specific nonce (pending transaction), so introducing a Time-To-Live for each transaction (e.g. 5 minutes) might make sense to avoid blocking the pipeline for the account, maybe even a way to cancel pending transaction before it reached quorum number of confirmations. It is getting trickier with cross-contract calls though, and it might require dedicated layer of proxies that make sure all the necessary confirmations and receipts are propagated on time (significant infrastructure investment and complication).

Another significant issue I see is scaling, specifically sharding - FastPay suggests that at most two shards are affected by the transfer as only two (`to` and `from` accounts) state units are mutated. For the generalised cross-contract calls, there are no limits - it is possible to have "stop the world"-like transactions that requires syncronizing the state across all shards. If this problem could have been solved for sharding (the initial scaling approach planned for Ethereum), maybe there won't be this many L2 rollups around.

So summarize: yes, in theory it is possible to build a Turing-complete EVM-compatible network on top of a lightweight two-phase commit protocol described in FastPay paper, but in practice I see significant problems with scaling it to handle production-like load.

---

Timeline:
- read the paper, draft initial design: ~1h
- initial impl: `common` + `crypto` + `Validator`: ~2h
- `reqwest`-based Client + `axum`-based Server: ~1h
- polishing, `error` + `setup`: ~1h
- polishing, finalize `demo`: ~2h
- readme and answers: ~1h

Total: ~8h

---

*(No LLMs were used during this assignment, all code & text is an authentic work of a biological neural network)*
