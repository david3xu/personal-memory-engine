// Type facade for the generated standalone validator; domain types come from Rust.
import type { DecisionVersion } from '../../../../contracts/generated/records';
export default function validateRecord(value: unknown): value is DecisionVersion;
