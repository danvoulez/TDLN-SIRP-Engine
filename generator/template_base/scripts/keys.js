#!/usr/bin/env node
// Generate Ed25519 keypair from random seed (32 bytes) and print JSON {seed_b64, pub_b64}
import { randomBytes } from 'crypto';
import { ed25519 } from '@noble/curves/ed25519';

const seed = randomBytes(32);
const priv = seed; // seed = private key (ed25519)
const pub = ed25519.getPublicKey(priv);
console.log(JSON.stringify({
  seed_b64: Buffer.from(priv).toString('base64'),
  pub_b64: Buffer.from(pub).toString('base64')
}));
