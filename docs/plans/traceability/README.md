# Traceability

Cross-phase traceability matrices linking requirements, ADRs, schema domains, and example workflows to implementation phases. These matrices ensure complete coverage and prevent goal drift.

## Matrix Documents

- schema-to-phase-matrix.md - Maps 19 schema domains from unified-workflow-schema.yml to owning phases (Phase 0-4) with acceptance criteria
- adr-to-phase-matrix.md - Links 47 ADR constraints to phase ownership with implementation verification
- workflow-to-phase-matrix.md - Maps 53 example workflows to phase testing responsibility
- requirements-traceability.md - Traces every requirement (R01-R31) through implementation phases

## Traceability Process

Each matrix provides:
- Ownership assignment for every component
- Acceptance criteria for phase transitions
- Test coverage verification
- Integration points between phases
- Validation criteria for completion

## Key Coverage

- **19 schema domains** → Phase ownership defined
- **47 ADR constraints** → Phase implementation verified
- **53 example workflows** → Phase testing responsibility assigned
- **31 requirements** → Complete traceability matrix

## Related

- [Unified Schema](../../reports/requirements/unifying-schema/unified-workflow-schema.yml) - Source schema with all 19 domains
- [Validation Framework](../validation-criteria/framework.md) - Phase exit and checkpoint criteria
- [Master Plan](../INDEX.md) - Phase definitions and dependency mapping
