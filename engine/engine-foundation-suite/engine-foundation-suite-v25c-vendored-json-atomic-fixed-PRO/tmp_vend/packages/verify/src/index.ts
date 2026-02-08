import { blake3 } from '@noble/hashes/blake3';
import * as ed from '@noble/ed25519';

const te = new TextEncoder();

function utf8Compare(a: string, b: string): number {
  const aa = te.encode(a);
  const bb = te.encode(b);
  const n = Math.min(aa.length, bb.length);
  for (let i = 0; i < n; i++) {
    if (aa[i] !== bb[i]) return aa[i] < bb[i] ? -1 : 1;
  }
  return aa.length === bb.length ? 0 : (aa.length < bb.length ? -1 : 1);
}

function jsonEscape(s: string): string {
  // JSON.stringify on a string is a correct escape implementation.
  return JSON.stringify(s.normalize('NFC'));
}

export function jsonAtomicStringify(v: any): string {
  if (v === null) return "null";
  const t = typeof v;
  if (t === "string") return jsonEscape(v);
  if (t === "boolean") return v ? "true" : "false";
  if (t === "number") {
    if (!Number.isFinite(v)) throw new Error("non-finite number not allowed");
    // JS toString matches JSON.stringify for numbers (no quotes).
    // NOTE: extremely large/small floats may use exponent notation.
    return String(v);
  }
  if (Array.isArray(v)) {
    return "[" + v.map(jsonAtomicStringify).join(",") + "]";
  }
  if (t === "object") {
    const keys = Object.keys(v).sort(utf8Compare);
    const parts: string[] = [];
    for (const k of keys) {
      const val = v[k];
      // JSON objects omit undefined
      if (val === undefined) continue;
      parts.push(jsonEscape(k) + ":" + jsonAtomicStringify(val));
    }
    return "{" + parts.join(",") + "}";
  }
  // functions/symbols/undefined are not allowed in JSON✯Atomic
  throw new Error("unsupported type in JSON✯Atomic");
}


function b3(data: Uint8Array): Uint8Array {
  return blake3(data);
}

function b64urlToBytes(s: string): Uint8Array {
  s = s.replace(/-/g, '+').replace(/_/g, '/');
  const pad = s.length % 4 ? 4 - (s.length % 4) : 0;
  return Uint8Array.from(atob(s + '='.repeat(pad)), c => c.charCodeAt(0));
}

export type AccessGrant = {
  kind: 'access.grant.v1';
  grant_id: string;
  sub: string;
  resource: any;
  exp: string;
  iat: string;
  nonce: string;
  seal: { alg: 'ed25519-blake3'; kid: string; sig: string };
};

export async function verifyGrantJSON(grant: AccessGrant, publicKeyB64: string): Promise<boolean> {
  if (grant.kind !== 'access.grant.v1') return false;
  const pk = Uint8Array.from(atob(publicKeyB64), c => c.charCodeAt(0));
  const g = { ...grant, seal: { ...grant.seal, sig: '' } };
  const msg = new TextEncoder().encode(jsonAtomicStringify(g));
  const digest = b3(msg);
  const sig = Uint8Array.from(atob(grant.seal.sig), c => c.charCodeAt(0));
  return ed.verify(sig, digest, pk);
}

// PASETO v4.public (Engine flavor): token = "v4.public." + b64url( json || sig )
export async function verifyPaseto(token: string, publicKeyB64: string): Promise<AccessGrant> {
  if (!token.startsWith('v4.public.')) throw new Error('invalid token prefix');
  const blob = token.substring('v4.public.'.length);
  const body = b64urlToBytes(blob);
  if (body.length < 64) throw new Error('invalid token body');
  const msg = body.slice(0, body.length - 64);
  const sig = body.slice(body.length - 64);
  const pk = Uint8Array.from(atob(publicKeyB64), c => c.charCodeAt(0));
  const digest = b3(msg);
  const ok = await ed.verify(sig, digest, pk);
  if (!ok) throw new Error('signature verification failed');
  const grant = JSON.parse(new TextDecoder().decode(msg));
  if (grant.kind !== 'access.grant.v1') throw new Error('payload kind mismatch');
  return grant;
}

export type AuditReport = {
  kind: 'audit.report.v1';
  proofs?: {
    inputs_merkle_root?: string;
    result_digest?: string;
  }
};

export function summarizePoi(resp: any) {
  if (resp?.decision === 'ASK' && resp?.poi) {
    const { reason, violations = [], hints = [] } = resp.poi;
    return { reason, violations, hints, ok: false };
  }
  return { ok: true };
}


export type VerifyOptions = {
  publicKeyB64: string;
  requiredKid?: string;
  now?: Date;
};

export function ipHash(ip: string): string {
  // simple: iphash:<blake3(ip)>, but we don't have blake3 in browser always;
  // return marker for server-side verification if unavailable.
  try {
    // @ts-ignore
    const h = blake3(new TextEncoder().encode(ip));
    // @ts-ignore
    const hex = Array.from(h).map(b => b.toString(16).padStart(2, '0')).join('');
    return `iphash:${hex}`;
  } catch {
    return 'iphash:<unavailable-in-browser>';
  }
}

export async function verifyPasetoWith(token: string, opts: VerifyOptions): Promise<AccessGrant> {
  const grant = await verifyPaseto(token, opts.publicKeyB64);
  // exp check
  const now = (opts.now ?? new Date()).toISOString();
  if (grant.exp && grant.exp < now) {
    throw new Error('grant expired');
  }
  // kid check (seal.kid for JSON grants; for PASETO we mirror from payload if present)
  if (opts.requiredKid) {
    if (!grant.seal?.kid || grant.seal.kid !== opts.requiredKid) {
      throw new Error('kid mismatch');
    }
  }
  return grant;
}


export function validateIpConstraint(grant: AccessGrant, clientIp: string) {
  const ip = clientIp || '';
  const want = grant?.resource?.constraints?.ip_hash;
  if (!want) return { ok: true };
  const have = ipHash(ip);
  if (have === want) return { ok: true };
  return { ok: false, violation: 'ip_hash_mismatch', hint: 'use X-Client-IP or request a new grant for this IP' };
}

export function rangeHint(byteRangeMax?: number, requestedRange?: string) {
  if (!byteRangeMax || !requestedRange) return { ok: true };
  const m = requestedRange.match(/^bytes=(\d+)-(\d+)$/);
  if (!m) return { ok: true };
  const start = parseInt(m[1], 10);
  const end = parseInt(m[2], 10);
  const size = end - start + 1;
  if (size <= byteRangeMax) return { ok: true };
  const effEnd = start + byteRangeMax - 1;
  return { ok: false, violation: 'byte_range_exceeds_limit', hint: `request <= ${byteRangeMax} bytes (e.g., bytes=${start}-${effEnd})` };
}


export type Health = { ok: boolean; presign_enabled: boolean; proxy_enabled: boolean; ts: string };

export async function fetchHealth(baseUrl: string, fetchImpl = fetch): Promise<Health> {
  const res = await fetchImpl(new URL('/health', baseUrl));
  if (!res.ok) throw new Error(`health http ${res.status}`);
  return await res.json();
}

export function parseHealth(h: Health) {
  const issues: string[] = [];
  if (!h.presign_enabled) issues.push('presign disabled');
  if (!h.proxy_enabled) issues.push('proxy disabled');
  return { ok: h.ok && issues.length === 0, issues };
}

export function startHealthMonitor(baseUrl: string, onUpdate: (h: Health & { issues: string[] }) => void, intervalMs = 30000, fetchImpl = fetch) {
  let stopped = false;
  async function tick() {
    try {
      const h = await fetchHealth(baseUrl, fetchImpl);
      const p = parseHealth(h);
      onUpdate({ ...h, issues: p.issues });
    } catch (e) {
      onUpdate({ ok: false, presign_enabled: false, proxy_enabled: false, ts: new Date().toISOString(), issues: ['health fetch error'] });
    }
    if (!stopped) setTimeout(tick, intervalMs);
  }
  tick();
  return () => { stopped = true; };
}


export type GRL = { kind: 'revocation.manifest.v1', updated_at: string, grants: string[], sig?: string };

export async function fetchGrl(baseUrl: string, fetchImpl = fetch): Promise<GRL> {
  const res = await fetchImpl(new URL('/.well-known/logline/grl.json', baseUrl));
  if (!res.ok) throw new Error(`grl http ${res.status}`);
  return await res.json();
}

export function isRevoked(grantId: string, grl: GRL): boolean {
  return Array.isArray(grl?.grants) && grl.grants.includes(grantId);
}

export function makeGrlCache() {
  let memo: { grl?: GRL, ts?: number } = {};
  return {
    get: () => memo.grl,
    set: (grl: GRL) => { memo = { grl, ts: Date.now() }; },
    fetchAndSet: async (baseUrl: string, fetchImpl = fetch) => {
      const g = await fetchGrl(baseUrl, fetchImpl);
      memo = { grl: g, ts: Date.now() };
      return g;
    }
  };
}
