# Known limitations

This is a historical reference model with trusted fixtures, not a hardened verifier.

- All named roles execute in one process. Public structs are caller-constructible; they are not authenticated principals.
- `shadow_verify` in `src/lib.rs:302–359` compares supplied digest strings and selected fields. It does not independently recompute every upstream body or validate every case identity. Altered caller-built structs can therefore retain an accepted `verified=true`; that label is local model output.
- ASHT's digest binds selected candidate IDs and scalar masses, not all `Candidate.evidence` bytes. Pipe-delimited string formatting is not an unambiguous codec for unrestricted hostile text.
- `mass_milli`, integer averaging and negative-on-equality are experimental controls. They do not replace production GCL, scientific evidence or independent observation.
- The Python oracle directly supplies `verified=True`. Its digest inputs differ from Rust, including Boolean spelling. Validation checks outcome classes, not byte-identical receipts or independent authentication.
- The fixture has 40 candidates and three cases. There is no automatic prose interpretation, physical measurement, production candidate maturity, persistent Vault, restart protocol or interactive evidence frontier.
- The output question invites a later input; it does not prove that another person answered or that new evidence was obtained.
- No formal proof, universal safety, legal novelty, institutional endorsement or independent peer review is claimed.

This capsule has no persistent WAL. No authenticated rollback prevention or external freshness guarantee is asserted.

Failures and counterexamples are welcome. Keep them distinct from reproducing the three expected outcomes.
