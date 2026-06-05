const globals = require('globals');
const js = require('@eslint/js');

module.exports = [
  js.configs.recommended,
  {
    files: ['src/**/*.js'],
    languageOptions: {
      ecmaVersion: 'latest',
      globals: globals.node,
      sourceType: 'commonjs',
    },
    rules: {
      'no-console': 'off',
    },
  },
];
