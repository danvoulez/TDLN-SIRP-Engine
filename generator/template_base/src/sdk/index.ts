export async function run(host: string, body: any){
  const res = await fetch(`${host}/v1/run`, {
    method: 'POST',
    headers: { 'Content-Type':'application/json' },
    body: JSON.stringify(body)
  });
  if(!res.ok) throw new Error(`HTTP ${res.status}`);
  return await res.json();
}

export function verifyCard(card:any){
  const url = card?.links?.url || '';
  const ok = /^https:\/\/cert\.tdln\.foundry\/r\/b3:[0-9a-f]{16,}$/.test(url);
  return { ok, url };
}

export function sirpKinds(card:any){
  return (card?.refs||[]).filter((r:any)=>/^sirp\./.test(r.kind)).map((r:any)=>r.kind);
}


function loadConfig(){
  try { return require('../../tdln/tdln.json'); } catch { return null; }
}
function idem(){ return crypto.randomUUID ? crypto.randomUUID() : Math.random().toString(16).slice(2); }

  // signature placeholder: attach ed25519 over body if you export signer here
  const res = await fetch(`${host}/v1/run`, { method:'POST', headers, body: JSON.stringify(body)});
  if(!res.ok) throw new Error(`HTTP ${res.status}`);
  return await res.json();
}

function isPlainObject(v:any){ return Object.prototype.toString.call(v)==='[object Object]'; }
function sortKeys(obj:any):any{
  if (Array.isArray(obj)) return obj.map(sortKeys);
  if (isPlainObject(obj)){
    const out:any = {};
    for (const k of Object.keys(obj).sort()) out[k]=sortKeys(obj[k]);
    return out;
  }
  return obj;
}
export function jsonAtomicStringify(o:any){
  return JSON.stringify(sortKeys(o));
}

async function signEd25519(seedB64:string, payload:string){
  try{
    const { ed25519 } = await import('@noble/curves/ed25519');
    const seed = Buffer.from(seedB64, 'base64');
    const priv = seed;
    const sig = ed25519.sign(Buffer.from(payload), priv);
    return 'ed25519:'+Buffer.from(sig).toString('base64');
  }catch(e){
    return '';
  }
}

function loadConfig(){
  try { return require('../../tdln/tdln.json'); } catch { return null; }
}
function idem(){ return (globalThis.crypto?.randomUUID?.() || Math.random().toString(16).slice(2)); }

export async function runCoordinated(body:any){
  const cfg = loadConfig();
  if(!cfg) throw new Error('tdln/tdln.json missing. Run scripts/provision.sh');
  const host = (process.env.TDLN_HOST||cfg.host);
  const canon = jsonAtomicStringify(body);
  const headers:any = { 'Content-Type':'application/json', 'X-Idempotency-Key': idem() };
  if(cfg.app?.did){ headers['X-App-DID'] = cfg.app.did; }
  if(cfg.app?.seed_ed25519_b64){
    headers['X-App-Signature'] = await signEd25519(cfg.app.seed_ed25519_b64, canon);
  }
  const res = await fetch(`${host}/v1/run`, { method:'POST', headers, body: canon });
  if(!res.ok) throw new Error(`HTTP ${res.status}`);
  return await res.json();
}
