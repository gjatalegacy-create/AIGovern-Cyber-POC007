# ESS-MAI: an executable research case for evidence-bound authority

**Bledar Gjata · Independent researcher and systems architect · [ORCID 0009-0003-7854-266X](https://orcid.org/0009-0003-7854-266X)**

ESS-MAI asks a concrete engineering question: **can a system make the right to act depend on a scoped obligation and inspectable evidence, while preserving failed attempts as useful information without turning them into permission or scientific truth?** Its public Rust POC003–007 artifacts let reviewers examine parts of that question, including counterexamples. The contribution proposed for review is the composition of these mechanisms, not the invention of hashes, capabilities, event logs or negative evidence.

## ESS-MAI is the originating method; AIGovern-Cyber is a derivative

[ESS-MAI](https://github.com/gjatalegacy-create/ESS-MAI) is the architectural and methodological core of this research line. Its method separates applicable law, bounded authority, evidence, traceability and the final right to commit an outcome. Calling it a core here identifies that lineage; it does not assert a production-certified general-purpose platform or an unrestricted reusable core library.

The derivation is inspectable in three layers:

1. **Method and claims:** the [Academic 1–3 corpus and constitutional proposal](theory/README.md), with the [contribution/prior-art boundary](CONTRIBUTIONS.md).
2. **Bounded executable extractions:** each POC identifies its source, adaptation, protocol and limits. POC005 records three focal files byte-identical to the declared v189 source; POC006 records four. Their adapters and experiments are identified separately in the [collection release notes](https://github.com/gjatalegacy-create/ESS-MAI/blob/main/publications/executable-prior-art/RELEASE_NOTES_v1.1.1.md).
3. **AIGovern-Cyber development:** public POC007 is a small, self-contained historical reference. [SOURCE_PROVENANCE.json](SOURCE_PROVENANCE.json) records its exact source hashes and method anchors. The larger integrated runtime is retained privately as a separate product direction; the public experiment does not depend on access to it.

The proposal that further specialized systems can be derived from ESS-MAI remains a research hypothesis. The existing family is evidence of bounded derivation work, not proof that every proposed profile or deployment will succeed.

## Five public artifacts, five specific review targets

The following are published author-reported execution results, including tool-assisted runs. They are not new executions performed for this note, and are not independent replications. Read each capsule's own protocol and limits before interpreting a count.

| Artifact | Public result | Boundary that makes review useful |
|---|---|---|
| [POC003: cold-start reachability](https://github.com/gjatalegacy-create/ESS-MAI/tree/main/publications/executable-prior-art/poc-003-system-cold-start-reachability) | Build PASS; 84/84 tests; empty-state gap and positive control | Distinguish an executable positive control from a generally solved cold start. |
| [POC004: GCL/Law-0 continuity](https://github.com/gjatalegacy-create/ESS-MAI/tree/main/publications/executable-prior-art/poc-004-gcl-law0-global-continuity) | Build PASS; 19/19 tests; supported behavior and counterexamples | Determine the exact continuity scope; counterexamples remain part of the record. |
| [POC005: FFI authority nonduplication](https://github.com/gjatalegacy-create/ESS-MAI/tree/main/publications/executable-prior-art/poc-005-ffi-law0-authority-nonduplication) | Strict build PASS; 15/15 detached tests; three matching runtime launches | One issued generation permits at most one post-authority result. Fresh semantic reissuance remains a counterexample; direct write remains held. |
| [POC006: living negative knowledge](https://github.com/gjatalegacy-create/ESS-MAI/tree/main/publications/executable-prior-art/poc-006-living-negative-knowledge-verified-reentry) | Strict build PASS; 30/30 detached tests; three matching runtime launches | Valid required sealed negative history re-enters and blocks a matching candidate. The detached FNV-based seal does not establish cryptographic collision resistance. The Boolean persistence receipt and detached restart boundary remain disclosed. |
| [POC007: obligation, promise and scoped comparison](https://github.com/gjatalegacy-create/AIGovern-Cyber-POC007) | 12 Rust tests; release build/run and Python controls PASS | Three synthetic fixture outcomes: `FAIL_CLOSED`, `RIGOROUS_NEGATIVE`, `CONSTRUCTIVE`. Caller-trusted verification remains a known limitation of this historical capsule. |

[Collection v1.1.1](https://doi.org/10.5281/zenodo.22750188) archives POC003–006. [POC007 bundle 0.1.2](https://doi.org/10.5281/zenodo.23596907) is a different software artifact. Hash equality concerns identified source/archive bytes; the collection explicitly does **not** claim identical Windows executable hashes across clean build roots.

## The architectural propositions worth challenging

| Proposition | Proposed role and available evidence | What is not established |
|---|---|---|
| Law and authority stay distinct | A transition requires its applicable conditions; an intermediate component cannot acquire the final commit right by forwarding data. POC004/005 expose bounded tests and failures. | Universal nonduplication, global enforcement or hostile-host security. |
| D-BAB witnesses; 0-NOI triggers | Witnessing records conditions/failure; triggering requests the next permitted action. Public POC007 contains simplified obligation/witness/promise mechanisms with source anchors. | That the full richer operators or all their native integrations are contained in this small capsule. |
| Negative knowledge supports advancement | Verified failure knowledge can exclude a failed route. A missing proof, a procedural HOLD and substantive counterevidence must retain distinct meanings. POC006 exposes a scoped re-entry experiment. | That every refusal proves a thesis false, or that failure-memory itself is newly invented. |
| Persistent memory retains context without becoming another judge | The proposed PD map identifies scoped references; the Vault stores records; LGC carries processing evidence. | Public POC007 has no production WAL or native process isolation. Private observations cannot be reproduced from it. |
| iZ turns a specific deficiency into the next question | The design connects current output and missing evidence to a scoped clarification. POC007 makes missing material visible through fixed fixtures. | General free-text understanding, arbitrary scientific truth, or complete conversational closure. |
| Hardware execution is bounded by evidence | The wider proposal distinguishes physical measurements, estimates, admission and actuator acknowledgements. | The public capsule has no hardware-control implementation. No physical cooling, energy-saving or measured acceleration claim is made. |

The [separately disclosed R47 receipt](PRIVATE_RUNTIME_EVIDENCE.md) reports 1,789 Rust tests and four doctests for a different, private implementation. Its author-reported native model cases include both `1:1` and `0:0`; its disclosed adverse result is acceptance of a coherent older WAL snapshot. These facts are not added to the public capsule's twelve tests and do not establish an external freshness guarantee.

## A practical invitation to reviewers

Start with one capsule, one obligation and one counterexample. For POC007, use the frozen release and [reproduction procedure](REPRODUCIBILITY.md):

```sh
cargo test --workspace --all-targets --locked --offline
cargo run --release --locked --offline --bin poc007-opls
python -B -X utf8 scripts/reference_oracle.py
```

The most useful findings would identify:

- the first point where claimed authority exceeds the evidence actually available;
- whether missing material is incorrectly treated as substantive counterevidence;
- a replay, scope mismatch or state transition that contradicts a stated obligation;
- whether the ESS-MAI composition adds a testable contribution beyond established capability, reference-monitor and runtime-verification methods.

Report the revision, environment, input, command, observed result and proposed boundary using the [review protocol](REVIEW_PROTOCOL.md). Adverse findings are welcome. [Existing POC005 external feedback](https://github.com/gjatalegacy-create/ESS-MAI/discussions/13) concerns public evidence and a simplified model; its author did not execute the original Rust POC. [PREreview](https://prereview.org/preprints/doi-10.5281-zenodo.23597026) hosts the POC007 preprint review request, not a completed review. [OpenSSF](https://www.bestpractices.dev/en/projects/15834/baseline-1) records an incomplete maintainer self-assessment, not a security certification.

For public-sector engineers, the potential relevance is inspectable evidence and failure boundaries before adoption. This is an invitation to evaluate the research, not a claim of public-administration deployment, EU endorsement or regulatory compliance. The material is suitable for discussion as research; its suitability for a production service remains to be demonstrated.

## Authorship, prior art and the retained boundary

Bledar Gjata directs the architecture and reports reviewing code and execution through the shell. AI tools assisted implementation, tests, literature checking and writing; [WORKING_METHOD.md](WORKING_METHOD.md) distinguishes that assistance from human review and independent replication. Albanian *besa* provides cultural naming context, not evidence that an ancient software protocol existed. The [bibliography](paper/references.bib) and [contribution statement](CONTRIBUTIONS.md) identify established foundations separately from proposed originality.

All five public artifacts retain their published licenses and scope. The full AIGovern-Cyber runtime, operational histories and unpublished product code remain private. This note adds a reviewable account of the method and existing evidence; it neither publishes that private system nor claims a new validation result. The associated [preprint](https://doi.org/10.5281/zenodo.23597026) has a separate DOI and is not peer reviewed.
