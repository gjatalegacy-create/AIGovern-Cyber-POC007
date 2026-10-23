#!/usr/bin/env python3
import pathlib, tomllib
root=pathlib.Path(__file__).resolve().parents[1]
tomllib.loads((root/'Cargo.toml').read_text())
text=(root/'src/lib.rs').read_text()
assert text.count('{')==text.count('}'), 'brace count mismatch'
for token in ['ZeroNoiVector','CollisionObligation','DbabLawWitness','BesaPromise','AshtTrace','NbesClaim','ShadowReceipt']:
    assert token in text, token
print('STATIC_SANITY_PASS')
