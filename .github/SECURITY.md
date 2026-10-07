# Security policy

## Supported versions

There is no released or runnable application yet. This repository contains a pre-implementation design and documentation foundation; it has not undergone a security audit. There are no supported production versions.

## Report a vulnerability privately

Use [GitHub private vulnerability reporting](https://github.com/david3xu/personal-memory-engine/security/advisories/new) for this repository. Include:

- The affected commit or version.
- Expected and observed behavior.
- Reproduction steps using synthetic or redacted data.
- The potential impact and any suggested mitigation.

Do not open a public issue containing exploit details, credentials, personal memory, or private conversations. Private reports should also avoid unnecessary personal data or secrets; a redacted reproduction is preferred.

The maintainer will assess reports as capacity allows. No guaranteed response time, paid support, or bounty program is offered.

## Relevant security boundaries

Future reports may concern unauthorized reading or recording, accidental publication of private records, unsafe rendering of submitted text or links, corruption of preserved history, or credential leakage. Schema validation does not prove that an AI worker correctly attributed a choice to a user.

The source-code license does not authorize publishing users' memory. Public examples and test fixtures should use synthetic data.

## Current implementation status

Authentication, access rules, persistence, safe rendering, and deployment security still need implementation and verification. A simple viewer passcode must not be described as sufficient protection for private memory.
