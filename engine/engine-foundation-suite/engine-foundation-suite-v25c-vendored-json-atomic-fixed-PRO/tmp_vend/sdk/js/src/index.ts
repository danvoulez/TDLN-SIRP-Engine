
export type Decision = "ACK"|"ASK"|"NACK";
export type Receipt = {
  did: string;
  decision: Decision;
  runtime: { input_cid: string; output_cid: string; };
  proof: { hash_chain: string[] };
  poi?: { missing_fields?: string[], hint?: string };
};

export async function verifyReceipt(receipt: Receipt, recompute?: {input: unknown, output: unknown}): Promise<boolean>{
  // NOTE: placeholder determinism check — real impl should canonicalize and recompute CIDs
  if (!receipt.proof?.hash_chain?.length) return false;
  return ["ACK","ASK","NACK"].includes(receipt.decision);
}

export async function fetchPresigned(url: string): Promise<Response>{
  return fetch(url, { method:"GET" });
}


export async function verifySirp(card:any){
  // Minimal invariant checks: presence and CID patterns. Real impl should canonicalize and verify sigs.
  if(!card?.refs) return {ok:false, reason:'missing refs'};
  const need = ['sirp.intent','sirp.delivery','sirp.result','sirp.execution'];
  const seen = new Set(card.refs.map((r:any)=>r.kind));
  for(const k of need){ if(!seen.has(k)) return {ok:false, reason:`missing ${k}`}; }
  const cidOk = (c:string)=>/^b3:[0-9a-f]{16,}$/.test(c||'');
  for(const r of card.refs){ if(!cidOk(r.cid)) return {ok:false, reason:`bad cid ${r.kind}`}; }
  return {ok:true};
}

import { ed25519 } from '@noble/curves/ed25519'
import { blake3 } from '@noble/hashes/blake3'

export async function verifySirpSignatures(objs:any[], pubkeyRaw?:Uint8Array){
  // Expect objects as plain JSON values (already fetched), canonicalized externally by engine (JSON✯Atomic).
  // We recompute BLAKE3 and verify 'signature' if pubkey provided.
  const results = []
  for (const o of objs){
    const payload = JSON.stringify(o) // placeholder for canon JSON✯Atomic if you export it here
    const cid = 'b3:' + Buffer.from(blake3(payload)).toString('hex')
    const sig = (o.signature||'').toString()
    results.push({cid, ok: !pubkeyRaw ? true : (sig.startsWith('ed25519:') && ed25519.verify(Buffer.from(sig.split(':')[1],'base64'), Buffer.from(payload), pubkeyRaw))})
  }
  return results
}
