import assert from 'node:assert/strict';
import test from 'node:test';
import { transpileBytes } from '@bytecodealliance/jco-transpile';
import { parse } from '@bytecodealliance/jco-transpile/wasm-tools';

test('component bindings preserve wasm32 addresses above 2 GiB and signed scalar values', async () => {
  const component = await parse(String.raw`(component
    (core module $m
      (memory (export "memory") 32769)
      (data (i32.const 0x80000020) "\10\00\00\80\01\00\00\00")
      (data (i32.const 0x80000030) "\f8\ff\ff\ff\10\00\00\00")
      (func (export "realloc") (param i32 i32 i32 i32) (result i32) i32.const 0x80000010)
      (func (export "first") (param i32 i32) (result i32) local.get 0 i32.load8_u)
      (func (export "bytes") (result i32) i32.const 0x80000020)
      (func (export "invalid") (result i32) i32.const 0x80000030)
      (func (export "signed") (result i32) i32.const -1))
    (core instance $i (instantiate $m))
    (func (export "first") (param "text" string) (result u32)
      (canon lift (core func $i "first") (memory (core memory $i "memory")) (realloc (core func $i "realloc"))))
    (func (export "bytes") (result (list u8))
      (canon lift (core func $i "bytes") (memory (core memory $i "memory"))))
    (func (export "invalid") (result (list u8))
      (canon lift (core func $i "invalid") (memory (core memory $i "memory"))))
    (func (export "signed") (result s32) (canon lift (core func $i "signed"))))`);
  const { files } = await transpileBytes(component, {
    name: 'probe',
    instantiation: 'async',
    base64Cutoff: 0,
    wasiShim: false,
    nodejsCompat: false,
  });
  const { instantiate } = await import(
    `data:text/javascript;base64,${Buffer.from(files['probe.js']).toString('base64')}`
  );
  const probe = await instantiate((file: string) => WebAssembly.compile(files[file]), {});
  assert.equal(probe.first('Q'), 81);
  assert.deepEqual(probe.bytes(), Uint8Array.of(81));
  assert.equal(probe.signed(), -1);
  assert.throws(() => probe.invalid(), WebAssembly.RuntimeError);
});
