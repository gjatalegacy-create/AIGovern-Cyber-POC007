# Separate private-runtime evidence

**Status: INTERNAL_AUTHOR_REPORTED; not independently reproduced from this public package.**

[evidence/private_R47_FINAL.json](evidence/private_R47_FINAL.json) is a byte-exact copy of the private R47 aggregate receipt. It is published for transparency, not as a claim that POC007 includes the underlying runtime or its complete evidence corpus. Its freeze/evidence hashes identify retained local records; hashes alone do not establish external authenticity.

The receipt reports 1,789 Rust tests, four doctests, 82 native checks, 100 continuity checks and 57 fault observations. These counts must not be added to this capsule's 12 tests. No institutional or independent peer-review status is implied.

The two native model cases used era00 with UT1 Julian-date parts `2400000.5 + 54388.0` and tolerance `1e-12` radians. Claim `0.40228372400281387` produced the recorded `1:1` pair; claim `0` produced `0:0`. These are comparisons under declared model inputs, not physical measurements or proof of universal scientific applicability.

The fault run confirmed that a valid older WAL snapshot remains accepted: its old reference resolves while a later reference is absent. There is no demonstrated external monotonic freshness guarantee. Damaged-record rejection and torn-tail recovery do not close that gap.

The public capsule has no production WAL or native process isolation. Its limitations and the private runtime's limitations remain separate. Full-system commercial development and private evidence retention continue outside this release.
