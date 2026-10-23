# Review, preservation and discoverability

Status checked on 23 October 2026. These records have different purposes: a review request is not a completed review, an archive is not a security audit, and an index entry does not imply endorsement.

## POC007

- **Public review request published:** [PREreview](https://prereview.org/preprints/doi-10.5281-zenodo.23597026), under Bledar Gjata and [ORCID 0009-0003-7854-266X](https://orcid.org/0009-0003-7854-266X). The page had zero completed PREreviews at this observation.
- **Code-level discussion:** [GitHub review invitation](https://github.com/gjatalegacy-create/AIGovern-Cyber-POC007/issues/1) and [Hugging Face discussion](https://huggingface.co/datasets/gjata-legacy/ess-mai-poc-007-aigovern-cyber/discussions/1). Both link to the preprint request and the exact public software release.
- **Publication and citation:** [software DOI](https://doi.org/10.5281/zenodo.23596907) and [preprint DOI](https://doi.org/10.5281/zenodo.23597026). These identify distinct objects. The public capsule remains the historical, bounded reference; the complete private product runtime is not included.
- **Preservation completed:** Software Heritage [save request 2570192](https://archive.softwareheritage.org/api/1/origin/save/2570192/) succeeded, with a full visit on 23 October 2026. Snapshot `swh:1:snp:e0db9680a3e9f1103b8ea8486a6b3ecd12d2c721` contains main revision `2802df476a62dfbbd612313ff44396c4cb255340` and the frozen `poc007-public-v0.1.2` tag pointing to revision `1407466d1794f1a11defe1174d16a76724e63bc3`. [Inspect the snapshot](https://archive.softwareheritage.org/api/1/snapshot/e0db9680a3e9f1103b8ea8486a6b3ecd12d2c721/). Later documentation is not silently included in that snapshot.
- **OpenAIRE discoverability:** both the [software](https://explore.openaire.eu/search/result?pid=10.5281%2Fzenodo.23596907) and [preprint](https://explore.openaire.eu/search/result?pid=10.5281%2Fzenodo.23597026) returned matching records in the public Graph API. This is indexing, not peer review.
- **Further indexing requested:** specific metadata inquiries were sent to OpenAlex and Semantic Scholar. No inclusion or profile approval is claimed while responses are pending.

## Earlier POC003–006

The [v1.1.1 collection](https://doi.org/10.5281/zenodo.22750188) is research software, not a preprint. Its [existing review invitation](https://huggingface.co/datasets/gjata-legacy/ess-mai-executable-prior-art/discussions/1) preserves the four capsules' distinct claims and limitations. PREreview has been asked which review route is suitable for this software collection; it has not been submitted under a false article classification.

POC005 received [external public technical feedback from Manuela Schrittwieser (MANU-de)](https://github.com/gjatalegacy-create/ESS-MAI/discussions/13) on 22 October 2026. She examined published evidence and described a simplified adversarial model, focusing on semantic reissuance, concurrent authorization and crash recovery. She explicitly did not execute the original Rust POC. The post supplies no exact reviewed commit, runnable model or raw logs. This contribution is acknowledged in the [current review register](https://github.com/gjatalegacy-create/ESS-MAI/blob/main/docs/REVIEW_STATUS.md), separately from independent Rust reproduction and from POC007's status.

The [OpenAIRE collection record](https://explore.openaire.eu/search/result?pid=10.5281%2Fzenodo.22750188) exists. OpenAlex [W7212575627](https://openalex.org/W7212575627) includes the v1.1.1 DOI among its locations but presents an older version's title; the metadata inquiry asks for accurate version representation without artificial duplicate counts. Software Heritage already retained the parent repository in [snapshot d2d862ac5122383925807c53d2ec4cda7d05b46a](https://archive.softwareheritage.org/api/1/snapshot/d2d862ac5122383925807c53d2ec4cda7d05b46a/) from 14 September 2026.

## Automated repository checks and their limits

OpenSSF Scorecard v5.6.0, tool commit `61e84743356275f6ffca23b1b2f2cb945e19eb96`, was executed locally against the public repositories. The official Windows archive checksum matched `f69b8b91f38c35f326c1e02085d4c2f189d5e22be0083bc952181faf767fcf67`.

| Target at checked commit | Raw aggregate | Evidence |
| --- | --- | --- |
| POC007 `2802df476a62dfbbd612313ff44396c4cb255340` | 2.3/10 | [default archive mode](evidence/openssf-2026-10-23/poc007.completed.json), [git fetch mode](evidence/openssf-2026-10-23/poc007.git_mode.json) |
| ESS-MAI `0c897c050ff3f7ed072f2173033b09b3139af278` | 4.0/10 | [raw result](evidence/openssf-2026-10-23/essmai.completed.json) |

These are tool outputs, not validated security ratings. A concrete detection mismatch remains: both POC007 modes report no workflows, although [the workflow at that exact commit](https://github.com/gjatalegacy-create/AIGovern-Cyber-POC007/blob/2802df476a62dfbbd612313ff44396c4cb255340/.github/workflows/poc007.yml) exists and [CI run 39960012672](https://github.com/gjatalegacy-create/AIGovern-Cyber-POC007/actions/runs/39960012672) completed successfully on that commit. The reason for that detection mismatch is unproven. CI-Tests also reports no pull request in the new POC007 repository; that does not mean Cargo was never run. The raw reports retain negative findings and unknown checks. Zero known vulnerabilities detected is not evidence of zero vulnerabilities. No security badge or architectural correctness claim is derived from these scores.

## Human direction, AI assistance and independent criticism

Bledar Gjata is the architect of the structure and design and directed the AI-assisted implementation. He reports personally reviewing generated code and execution through the shell. AI tools also assisted English translation and scientific presentation. The [working method](WORKING_METHOD.md) distinguishes the architectural proposal, tool-assisted engineering, author inspection and independent reproduction. This is not represented as exclusively manual coding or translation-only AI assistance.

Participation is open to relevant researchers and practitioners, whether institutionally affiliated or independent. Existing Apache-2.0 rights remain unchanged, including commercial use under that license. No positive review, compulsory citation or institutional endorsement is a condition of access. The author has a prospective commercial interest in a separate private AIGovern-Cyber product; that interest does not constrain reviewers' conclusions. Appropriate citation identifies the exact work actually used or discussed.

Platform rules are applied to the actual assistance used. A ban on AI-generated code or submissions is not bypassed by calling development assistance translation. Where translation is allowed, that contribution can be disclosed separately. ScienceOpen remains a prospective route after account setup; no import or review there is claimed yet.
