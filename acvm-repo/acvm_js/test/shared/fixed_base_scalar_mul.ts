// See `fixed_base_scalar_mul_circuit` integration test in `acir/tests/test_program_serialization.rs`.
export const bytecode = Uint8Array.from([
  31, 139, 8, 0, 0, 0, 0, 0, 0, 255, 77, 138, 91, 10, 0, 48, 12, 194, 178, 215, 207, 78, 189, 163, 175, 165, 10, 21, 36,
  10, 57, 192, 160, 146, 188, 226, 139, 78, 113, 69, 183, 190, 61, 111, 218, 182, 231, 124, 68, 185, 243, 207, 92, 0, 0,
  0,
]);
export const initialWitnessMap = new Map([
  [1, '0x0000000000000000000000000000000000000000000000000000000000000001'],
  [2, '0x0000000000000000000000000000000000000000000000000000000000000000'],
]);

export const expectedWitnessMap = new Map([
  [1, '0x0000000000000000000000000000000000000000000000000000000000000001'],
  [2, '0x0000000000000000000000000000000000000000000000000000000000000000'],
  [3, '0x0000000000000000000000000000000000000000000000000000000000000001'],
  [4, '0x0000000000000002cf135e7506a45d632d270d45f1181294833fc48d823f272c'],
]);