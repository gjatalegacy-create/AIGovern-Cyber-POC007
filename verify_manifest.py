from pathlib import Path
import hashlib
root = Path(__file__).resolve().parent
count = 0
for line in (root / 'artifact_hashes.sha256').read_text(encoding='utf-8').splitlines():
    expected, name = line.split('  ', 1)
    p = (root / name).resolve()
    if not p.is_relative_to(root) or not p.is_file():
        raise SystemExit('Invalid manifest path: ' + name)
    if hashlib.sha256(p.read_bytes()).hexdigest() != expected:
        raise SystemExit('SHA-256 mismatch: ' + name)
    count += 1
print('PASS: ' + str(count) + ' manifested files; integrity is not authorization.')
