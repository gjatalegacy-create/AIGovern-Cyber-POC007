---
license: apache-2.0
language:
  - en
  - sq
tags:
  - rust
  - reproducibility
  - governance
  - formal-methods
  - research
  - proof-of-concept
pretty_name: AIGovern-Cyber POC007 — Obligation Promise Law to Success
---

# AIGovern-Cyber POC007

A small, runnable historical reference for the ESS-MAI method by **Bledar Gjata**, Gjata Legacy. Publication bundle **0.1.2** preserves the historical v0.1.1 capsule's executable source exactly; its Cargo crate remains version **0.1.0**.

The bounded proposition is: positive support alone cannot satisfy this fixture's exact, same-scope dual-polarity obligation. Missing material produces a scoped question; completed comparisons retain the opposing polarity in their output.

| Original case | Observed result |
|---|---|
| Missing exact positive candidate | `FAIL_CLOSED` |
| Negative fixture mass is greater | `RIGOROUS_NEGATIVE` |
| Positive fixture mass is greater | `CONSTRUCTIVE` |

Fresh public-package validation: **12 Rust tests passed, 0 failed, 0 ignored; release build/run and Python controls passed; no warnings**. The Rust crate has no third-party dependencies. These results concern synthetic fixtures and one process. They do not establish production isolation, scientific truth or full-system security.

```sh
cargo test --workspace --all-targets --locked --offline
cargo run --release --locked --offline --bin poc007-opls
python -B -X utf8 scripts/reference_oracle.py
```

Read [the working method](WORKING_METHOD.md), [reproducibility](REPRODUCIBILITY.md), [limitations](KNOWN_LIMITATIONS.md), [public/private boundary](PUBLIC_PRIVATE_BOUNDARY.md) and the [voluntary review protocol](REVIEW_PROTOCOL.md). [Public results](evidence/RESULT.json) have sanitized logs and recorded transformations. A [separate author-reported R47 summary](PRIVATE_RUNTIME_EVIDENCE.md) discloses private-runtime counts and limitations. The full private runtime and its complete evidence corpus are outside this release.

Canonical code: [GitHub](https://github.com/gjatalegacy-create/AIGovern-Cyber-POC007). Identical publication mirror: [Hugging Face](https://huggingface.co/datasets/gjata-legacy/ess-mai-poc-007-aigovern-cyber).

**Shqip:** Ky POC i vogël riprodhon tri raste të kontrolluara. Nuk është sistemi i plotë privat dhe nuk dëshmon siguri universale. Rishikimi i pavarur është i mirëpritur dhe vullnetar.

Licensed under [Apache-2.0](LICENSE), including commercial use under its terms. Review is not a condition of use. See [NOTICE](NOTICE.md) for attribution and AI-assistance disclosure.

## Research and provenance

[ESS-MAI origin](https://github.com/gjatalegacy-create/ESS-MAI) · [Bledar Gjata's ORCID](https://orcid.org/0009-0003-7854-266X) · [earlier POC003–006 DOI](https://doi.org/10.5281/zenodo.22750188).

Read the [contribution boundary](CONTRIBUTIONS.md), [source theory and Academic 1–3](theory/README.md), [claims ledger](CLAIMS_LEDGER.md), and [manuscript draft](paper/manuscript.md) ([PDF](paper/constitutional_self_governance_draft.pdf)). The draft is not submitted to arXiv or peer reviewed. The hardware-adaptation proposal and current estimator limitations are explicitly separated from this capsule's capabilities.

Verify the publication files by running python verify_manifest.py. The manifest binds this release's files, not their truth or author identity. No new DOI is claimed for this bundle.
