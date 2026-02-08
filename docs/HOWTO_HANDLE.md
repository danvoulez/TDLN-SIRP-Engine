# Handles curtos, portabilidade e verificação offline

## Regra de ouro
- **CID é identidade**. URL é só caminho.
- `links.card_url` é um *handle curto* humano/QR:
  `https://cert.tdln.foundry/r/<run_cid>` (onde `<run_cid>` = `b3:<hex>` do **run**).

## Como o handle resolve
1. Cliente abre `https://cert.tdln.foundry/r/<run_cid>`.
2. Servidor retorna **303 See Other** para pelo menos dois destinos (mirrors):
   - `registry` (público)
   - `mirror` (secundário)
   - opcional: `tdln://objects/<cid>` (resolver local configurável)
3. Cliente baixa bytes de qualquer href e valida **pelo CID** (BLAKE3).

## Estrutura em `receipt.card.v1` (exemplo)
```json
{
  "links": { "card_url": "https://cert.tdln.foundry/r/b3:<hex>" },
  "refs": [
    {
      "kind": "unit.manifest",
      "cid": "cid:b3:<hex>",
      "media_type": "application/json",
      "hrefs": [
        "https://registry.example/v1/objects/cid:b3:<hex>",
        "tdln://objects/cid:b3:<hex>"
      ]
    },
    {
      "kind": "bundle.private",
      "cid": "cid:b3:<hex>",
      "media_type": "application/car",
      "hrefs": [
        "https://registry.example/v1/presign/cid:b3:<hex>?ttl=600"
      ]
    }
  ]
}
```

## Verificação offline (passo a passo)
1. Validar assinatura (`ed25519-blake3`) do card.
2. Resolver refs por qualquer href disponível.
3. Calcular BLAKE3 e comparar com o CID.
4. Reconstituir `hash_chain` e checar integridade.

## QR Code
- **Sempre encode o handle curto (`card_url`)**.
- Presigned **não é identidade** (expira/muda).

## Política
- Card é **public-safe**.
- Bundles podem ser privados (presign/ACL/cripto).

