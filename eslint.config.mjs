// Apply strict TypeScript rules to interface and verification code.
import js from '@eslint/js';
import ts from 'typescript-eslint';
import globals from 'globals';
export default ts.config(
  {
    ignores: [
      '**/dist/**',
      'target/**',
      'contracts/generated/**',
      'web/owner/src/generated/validate-record.mjs',
    ],
  },
  js.configs.recommended,
  ...ts.configs.recommended,
  {
    languageOptions: { globals: { ...globals.browser, ...globals.node } },
    rules: { '@typescript-eslint/no-explicit-any': 'error' },
  },
);
