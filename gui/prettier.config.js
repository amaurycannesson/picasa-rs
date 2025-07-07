/**
 * @see https://prettier.io/docs/configuration
 * @type {import("prettier").Config}
 */
const config = {
  plugins: ['prettier-plugin-tailwindcss'],
  singleQuote: true,
  arrowParens: 'always',
  trailingComma: 'all',
  semi: true,
  printWidth: 100,
  tabWidth: 2,
};

export default config;
