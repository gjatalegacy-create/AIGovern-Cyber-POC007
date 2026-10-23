# How we reached this release

## Human direction and AI-assisted engineering

Bledar Gjata supplied the architectural direction, terminology, semantic constraints and critical corrections. AI coding and reasoning tools assisted source inspection, implementation, test preparation, execution, literature checking and editorial work. Their suggestions were not accepted merely because a model generated them. Human authorship of the architectural proposal does not mean every source line was manually typed by the author. AI assistance is disclosed rather than hidden.

The historical conversation is design input, not execution evidence. A tool's agreement is not an independent scientific endorsement; two models repeating an assumption do not make it true. The method asks for an explicit relation between the claim, source, authority boundary, test and observed result.

## Working discipline

1. **State the obligation.** Describe the requested behavior, the law/authority that owns it, its evidence requirements and the scope it cannot exceed.
2. **Read the source first.** ESS-MAI supplies the methodological lineage. Extract only the needed mechanism and preserve its role; distinguish literal code reuse from a new derivative implementation. Keep canonical and frozen copies unchanged.
3. **Expose the mismatch.** Separate witnessed missing evidence from substantive counterevidence, estimate from physical observation, continuation from positive acceptance, and an unkeyed hash from authorization.
4. **Make a bounded change.** Where correction is authorized, use the smallest testable derivative change. Mathematical notation must identify actual operands, units, empty cases and consumers; a formula's appearance alone is not materialization.
5. **Test before claiming closure.** A regression should demonstrate the reported failure when appropriate, then the corrected behavior. Run the relevant Cargo commands; record exit status, environment, warnings, ignored tests and remaining limitations. Passing compilation is not proof of the theory.
6. **Freeze and preserve adverse observations.** Hash the evaluated files and evidence. Preserve the distinction between test success and a successful observation of a vulnerability or missing guarantee.
7. **Invite falsification.** Publish a complete bounded artifact, exact commands and a review protocol. Compare claims against prior art. Correct an identified defect in a versioned record; do not erase an unfavorable finding.

## What actually happened in this publication round

The R48 publication copied the historical POC007 source without behavior edits, ran twelve original Rust tests and the three fixture cases, compared outcome classes against a Python oracle, and retained the caller-trust limitation in public documentation. It did not replace that capsule with the much larger private runtime. The new R48 evidence belongs to the capsule; the separately authorized R47 summary belongs to a different frozen implementation.

Bibliographic titles and author identity were checked against primary sources. The supplied ORCID was verified through the official public record; its GitHub/HF links match the project. The existing Zenodo DOI was checked as the older POC003–006 collection, not relabelled as this release. The manuscript is a draft, not an arXiv acceptance or peer-reviewed result.

The public/private boundary protects ongoing product development while stating exactly what cannot be independently reproduced from this smaller capsule. The methodology, its mistakes and limitations are disclosed; private operational histories and unpublished product source are not needed to describe that methodology honestly.

## Evidence vocabulary

- **Reference:** an existing method, publication or cultural source.
- **Theory:** an intended architectural relationship, with a proposed falsifier.
- **Source observation:** behavior supported by identified code, with call-path limits.
- **Execution evidence:** a result of a particular command and input in a stated environment.
- **Hypothesis / proposed innovation:** an explanation or contribution still needing broader comparison or tests.
- **Unresolved:** a boundary the current evidence does not close.

No label automatically promotes another. A hash is an identifier; a passing test is bounded evidence; a DOI is a publication record. The method becomes stronger when a reviewer can find the first unsupported step and reproduce it.
