# wrapper-gen v3 — baseado no Wrapper Template v3 (TDLN + SIRP, PROD)

Gera cascas **produtizáveis** a partir de 5 respostas.
Sem placeholders. Saída sempre com `/v1/run`, **links.url** curto (`/r/<run_cid>`) e **RREF + SIRP** documentados.

## Uso rápido
```bash
# 1) Configurar motor
export TDLN_HOST="https://seu-engine"

# 2) Gerar casca (interativo)
./generate.sh

# 3) Rodar
cd wrappers/<slug>
./scripts/run.sh ./examples/run.json
./scripts/verify.sh ./examples/card.sample.json
```

## O que este gerador faz
- Copia `template_base/` para `wrappers/<slug>/`.
- Preenche `README.md`, `policies/product.policy.json`, e `examples/run.json` com as escolhas.
- Mantém **RREF + SIRP** e `.env.example` sem defaults.
