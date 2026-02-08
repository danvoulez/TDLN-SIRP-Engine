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
