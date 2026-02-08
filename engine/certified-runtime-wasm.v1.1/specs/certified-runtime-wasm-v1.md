# Certified Runtime — WASM (v1)

**Objetivo:** Executar unidades WASM de forma **determinística**, **capability-safe** e **auditável**, emitindo recibos `receipt.card.v1` assinados e verificáveis offline.


## Invariantes (obrigatórios)

1. **Determinismo:** `nan-canonical`, proibição de fontes de tempo/aleatoriedade não declaradas, ordenação estável de iterações, sem threads.
2. **Sandbox:** VM isolada; **imports** só via whitelists; sem FS/rede por padrão; limites de memória/tempo.
3. **Metering:** fuel/quota obrigatório; aborta em exaustão com `ASK/PoI: fuel_exhausted`.
4. **Capability Model:** a unit declara `required_effects[]` e o runtime aplica `EngineMode::conservative()` por padrão.
5. **CID-first:** entradas/saídas/artefatos **endereçados por BLAKE3** dos bytes canônicos (JSON✯Atomic).
6. **Selagem:** `seal.alg = ed25519-blake3` sobre bytes canônicos do card; chave gerida por HSM/TPM quando disponível.
7. **EER (Exec Env Receipt):** hash do binário do runtime + config (VM flags) + `wasmtime_version` + `policy_hash`.
8. **NHE:** Sem HITL. Indecisão ⇒ `ASK` + `PoI` máquina-legível.


## Protocolo de Execução

Entrada:
```json
{
  "unit_ref": "cid:b3:<WASM or pack>",
  "input": { "...": "..." },
  "mode": {"profile":"conservative","fuel": 10_000_000},
  "effects": ["json_io"]  // exemplo
}
```

Saída (resumo):
```json
{
  "kind": "receipt.card.v1",
  "realm": "trust",
  "decision": "ACK|ASK|NACK",
  "output_cid": "cid:b3:<...>",
  "proof": { "seal": {...}, "hash_chain": [{"kind":"input","cid":"..."},{"kind":"output","cid":"..."}] },
  "refs": [{"kind":"unit.wasm","cid":"cid:b3:<...>","hrefs":["https://registry.../objects/<cid>","tdln://objects/<cid>"]}],
  "links": {"card_url":"https://cert.tdln.foundry/r/b3:<run_cid>"}
}
```

## Conformidade v1

- `wasm`: **sem** `wasi_snapshot_preview1` por padrão; apenas imports explícitos (`env::json_in`, `env::json_out`).
- `deterministic`: flags ativas (canon NaN, no fuel nondet).
- `limits`: memória ≤ 256MiB (configurável), fuel obrigatória.
- `proof`: inclui `eer` com `runtime_hash`, `config_digest`, `wasmtime_version`.
- `verify`: recomputar BLAKE3 de todos os blobs por `cid:`; validar `seal`.
