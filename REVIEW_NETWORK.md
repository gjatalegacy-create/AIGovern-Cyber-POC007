# Review, preservation and discoverability

Status checked on 23 October 2026, with OpenAlex and DataCite observations updated on 24 October 2026 at 14:39–14:40 UTC. These records have different purposes: a review request is not a completed review, an archive is not a security audit, and an index entry does not imply endorsement.

## POC007

- **Public review request published:** [PREreview](https://prereview.org/preprints/doi-10.5281-zenodo.23597026), under Bledar Gjata and [ORCID 0009-0003-7854-266X](https://orcid.org/0009-0003-7854-266X). The page had zero completed PREreviews at this observation.
- **Code-level discussion:** [GitHub review invitation](https://github.com/gjatalegacy-create/AIGovern-Cyber-POC007/issues/1) and [Hugging Face discussion](https://huggingface.co/datasets/gjata-legacy/ess-mai-poc-007-aigovern-cyber/discussions/1). Both link to the preprint request and the exact public software release.
- **Publication and citation:** [software DOI](https://doi.org/10.5281/zenodo.23596907) and [preprint DOI](https://doi.org/10.5281/zenodo.23597026). These identify distinct objects. The public capsule remains the historical, bounded reference; the complete private product runtime is not included.
- **Preservation completed:** Software Heritage [save request 2570192](https://archive.softwareheritage.org/api/1/origin/save/2570192/) succeeded, with a full visit on 23 October 2026. Snapshot `swh:1:snp:e0db9680a3e9f1103b8ea8486a6b3ecd12d2c721` contains main revision `2802df476a62dfbbd612313ff44396c4cb255340` and the frozen `poc007-public-v0.1.2` tag pointing to revision `1407466d1794f1a11defe1174d16a76724e63bc3`. [Inspect the snapshot](https://archive.softwareheritage.org/api/1/snapshot/e0db9680a3e9f1103b8ea8486a6b3ecd12d2c721/). Later documentation is not silently included in that snapshot.
- **OpenAIRE discoverability:** both the [software](https://explore.openaire.eu/search/result?pid=10.5281%2Fzenodo.23596907) and [preprint](https://explore.openaire.eu/search/result?pid=10.5281%2Fzenodo.23597026) returned matching records in the public Graph API. This is indexing, not peer review.
- **OpenAlex indexing confirmed:** the public API returned [software W7223384049](https://openalex.org/W7223384049) for [version DOI 23596907](https://api.openalex.org/works/https://doi.org/10.5281/zenodo.23596907) and [preprint W7223258585](https://openalex.org/W7223258585) for [version DOI 23597026](https://api.openalex.org/works/https://doi.org/10.5281/zenodo.23597026) on 24 October 2026. Their titles and types match the DataCite version records. OpenAlex uses the respective concept DOIs as primary DOIs and includes the version DOIs among its locations. Indexing does not establish peer review or endorsement, and this observation does not establish why ingestion occurred.
- **Remaining metadata limits:** both POC007 records link to [author A5147940812](https://api.openalex.org/authors/A5147940812) and retain the supplied ORCID in `raw_orcid`, but the normalized author ORCID remains absent. Semantic Scholar inclusion remains unconfirmed in this status record.

## Related POC008 and POC009

At the same 24 October 2026 observation, direct OpenAlex DOI lookups for POC008 [software](https://doi.org/10.5281/zenodo.23613613) and [preprint](https://doi.org/10.5281/zenodo.23613785), and POC009 [software](https://doi.org/10.5281/zenodo.23614687) and [preprint](https://doi.org/10.5281/zenodo.23614714), returned HTTP 404. A successful six-DOI filter query returned only the two POC007 records. These are dated lookup absences, not a prediction of future indexing. All four source records were findable in DataCite and were created on 24 October 2026.

The existing public PREreview requests for [POC008](https://prereview.org/preprints/doi-10.5281-zenodo.23613785) and [POC009](https://prereview.org/preprints/doi-10.5281-zenodo.23614714) were published on 24 October 2026. They invite independent criticism; publication of a request does not establish a completed review.

Additional dissemination actions on 24 October 2026:

- **This Week in Rust:** [pull request 8914](https://github.com/rust-lang/this-week-in-rust/pull/8914) proposes a POC009 call for participation linking the existing [reproduction task](https://github.com/gjatalegacy-create/AIGovern-Cyber-POC009-Knowledge-Memory/issues/1). The pull request is open at this observation; it is not merged or confirmed for newsletter publication.
- **Research Compendium:** the [POC009 software deposit](https://doi.org/10.5281/zenodo.23614687) was submitted to the [Zenodo community](https://zenodo.org/communities/research-compendium/); the curator decision is pending. Community submission does not establish acceptance, peer review or independent execution. The author approved the community's curator access to the metadata and files of this public deposit.
- **POC009 software/preprint links:** the software record now adds `isCitedBy` and `isSupplementTo` relations to its existing [preprint](https://doi.org/10.5281/zenodo.23614714). The archive and checksum file identities, sizes and published checksums were unchanged by this metadata update; no new artifact version is asserted.

## Earlier POC003–006

The [v1.1.1 collection](https://doi.org/10.5281/zenodo.22750188) is research software, not a preprint. Its [existing review invitation](https://huggingface.co/datasets/gjata-legacy/ess-mai-executable-prior-art/discussions/1) preserves the four capsules' distinct claims and limitations. PREreview has been asked which review route is suitable for this software collection; it has not been submitted under a false article classification.

POC005 received [external public technical feedback from Manuela Schrittwieser (MANU-de)](https://github.com/gjatalegacy-create/ESS-MAI/discussions/13) on 22 October 2026. She examined published evidence and described a simplified adversarial model, focusing on semantic reissuance, concurrent authorization and crash recovery. She explicitly did not execute the original Rust POC. The post supplies no exact reviewed commit, runnable model or raw logs. This contribution is acknowledged in the [current review register](https://github.com/gjatalegacy-create/ESS-MAI/blob/main/docs/REVIEW_STATUS.md), separately from independent Rust reproduction and from POC007's status.

The [OpenAIRE collection record](https://explore.openaire.eu/search/result?pid=10.5281%2Fzenodo.22750188) exists. OpenAlex [W7212575627](https://openalex.org/W7212575627) still includes the v1.1.1 DOI among its locations but presents the older v1.0.0 title as of 24 October 2026. The source DataCite record remains findable with the v1.1.1 title; accurate version representation is still unresolved. Software Heritage already retained the parent repository in [snapshot d2d862ac5122383925807c53d2ec4cda7d05b46a](https://archive.softwareheritage.org/api/1/snapshot/d2d862ac5122383925807c53d2ec4cda7d05b46a/) from 14 September 2026.

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

Platform rules are applied to the actual assistance used. A ban on AI-generated code or submissions is not bypassed by calling development assistance translation. Where translation is allowed, that contribution can be disclosed separately. A ScienceOpen enquiry was sent on 24 October 2026 about metadata import and a free community-review route for the existing [POC007](https://doi.org/10.5281/zenodo.23597026), [POC008](https://doi.org/10.5281/zenodo.23613785) and [POC009](https://doi.org/10.5281/zenodo.23614714) preprints. No account setup, import, indexing, review or acceptance there is confirmed.
