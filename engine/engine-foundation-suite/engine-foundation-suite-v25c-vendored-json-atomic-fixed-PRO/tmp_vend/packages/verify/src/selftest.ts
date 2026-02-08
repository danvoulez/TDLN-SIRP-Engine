import assert from 'node:assert/strict';
import { jsonAtomicStringify } from './index.js';

const obj: any = {
  z: 2,
  a: 1,
  name: 'cafe\u0065\u0301',
  arr: [{ b: 2, a: 1 }, 3],
};

const got = jsonAtomicStringify(obj);
const expected = '{"a":1,"arr":[{"a":1,"b":2},3],"name":"caf\u00e9","z":2}';
assert.equal(got, expected);
console.log('OK: jsonAtomicStringify matches golden canonical string.');
