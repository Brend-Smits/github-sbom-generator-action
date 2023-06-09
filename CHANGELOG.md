# Changelog

## [1.1.0](https://github.com/Brend-Smits/github-sbom-generator-action/compare/v1.0.3...v1.1.0) (2023-05-01)


### Features

* add repository argument ([5940d0b](https://github.com/Brend-Smits/github-sbom-generator-action/commit/5940d0ba814e6d8bfb4fdc71915e77bfbf723d6d))


### Bug Fixes

* failing tests due to repo name change ([8535cf1](https://github.com/Brend-Smits/github-sbom-generator-action/commit/8535cf1dcaf668a88df9ead5748458e17d3f3c21))

## [1.0.3](https://github.com/Brend-Smits/github-sbom-generator-action/compare/v1.0.2...v1.0.3) (2023-04-21)


### Bug Fixes

* create directory if it does not exist yet ([48485fc](https://github.com/Brend-Smits/github-sbom-generator-action/commit/48485fc6471072ef9d9c0e997112b5b5e5f8d03f))

## [1.0.2](https://github.com/Brend-Smits/github-sbom-generator-action/compare/v1.0.1...v1.0.2) (2023-04-21)


### Bug Fixes

* change curl output to include action_path ([1618108](https://github.com/Brend-Smits/github-sbom-generator-action/commit/16181083d523d338e8394f4cd84e0ec81dcd1c69))

## [1.0.1](https://github.com/Brend-Smits/retrieve-github-sbom-action/compare/v1.0.0...v1.0.1) (2023-04-21)


### Bug Fixes

* give actions the proper credentials to upload binaries ([a361b57](https://github.com/Brend-Smits/retrieve-github-sbom-action/commit/a361b5714fdcf9eff4fe4098d9071c0229c9b600))

## [1.0.0](https://github.com/Brend-Smits/retrieve-github-sbom-action/compare/v0.1.0...v1.0.0) (2023-04-21)


### Features

* add ci and release workflow ([911aa36](https://github.com/Brend-Smits/retrieve-github-sbom-action/commit/911aa36252e48eae38e942d75b1d63afbef35d44))
* move away from bash and use rust cli in action ([bfd4af8](https://github.com/Brend-Smits/retrieve-github-sbom-action/commit/bfd4af80ea2a6676a0fed5340cc254b8ee47f5e0))
* open repo list path and loop through each line ([d77e549](https://github.com/Brend-Smits/retrieve-github-sbom-action/commit/d77e549cb58dadfc6a51238e229cb99d001c2a9a))
* save spdx files to configured directory ([1ca1b4b](https://github.com/Brend-Smits/retrieve-github-sbom-action/commit/1ca1b4b437a233a94d62c83f985575007768a3c3))


### Bug Fixes

* **ci:** annotate files for release-please ([30ecdcc](https://github.com/Brend-Smits/retrieve-github-sbom-action/commit/30ecdcca05a5bdfc5f279171defdc4014854673d))
* **ci:** only run test workflow on non release prs ([b3e67ca](https://github.com/Brend-Smits/retrieve-github-sbom-action/commit/b3e67ca70f1476fd587a3b8fd5ec6facdbd56fd8))
* **ci:** skip test workflow when ref_name contains release-please ([9bf27b9](https://github.com/Brend-Smits/retrieve-github-sbom-action/commit/9bf27b91c451bbab32e6fcd316899d1758e4e5e6))
* directories were not being created if they did not exist ([ffdf7ba](https://github.com/Brend-Smits/retrieve-github-sbom-action/commit/ffdf7ba8c7deae02bf2da2cd29f0704d794f9ca0))


### Miscellaneous Chores

* release 1.0.0 ([d13727c](https://github.com/Brend-Smits/retrieve-github-sbom-action/commit/d13727c82c76a025bd36016f20be6c27c8284f77))
