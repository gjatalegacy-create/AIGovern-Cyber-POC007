# Publication visibility changes

23 October 2026: current main adds accurate citation metadata, a scholarly paper landing page, a voluntary community-review invitation, the author's reported shell-review method, and verified public account and discovery links. This is a documentation and metadata update; it adds no runtime code or new Cargo validation.

The software archive is now citable as [10.5281/zenodo.23596907](https://doi.org/10.5281/zenodo.23596907). OpenAIRE returned one matching software record, with status `UNDER_CURATION`. Discoverability does not constitute peer review. The manuscript is published separately as [Zenodo preprint 10.5281/zenodo.23597026](https://doi.org/10.5281/zenodo.23597026), not peer reviewed or submitted to arXiv. Its PDF, Markdown and bibliography were downloaded from the public record and verified byte-identical to the R48 materials.

## Frozen artifact and current main

The immutable [public release 0.1.2](https://github.com/gjatalegacy-create/AIGovern-Cyber-POC007/releases/tag/poc007-public-v0.1.2), source commit `1407466d1794f1a11defe1174d16a76724e63bc3`, and its Zenodo archive retain the R48 publication bytes:

- R48 archive SHA-256: `6d7d243d79ab8eb00f91e964509f9128109c93e37983483329af3a6107a59e24`.
- R48 manifest SHA-256: `3c278a59ac7a9fae048fc4ab3c88e084327106281c8ca6e21e06da796dd6088d`.

Current main's `artifact_hashes.sha256` is regenerated for its public documentation additions and therefore differs from the frozen R48 manifest. Use the manifest from the exact checkout or archive being verified. The ten original source, test, fixture, helper-script and license files remain byte-identical to the frozen archive. The earlier twelve-test execution evidence is unchanged; it is not a new test result for this documentation round.
