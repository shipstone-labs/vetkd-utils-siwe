import importRemappings from "solhint-import-remappings";

module.exports = {
  extends: "solhint:recommended",
  plugins: ["solhint-import-remappings"],
  rules: {
    "func-visibility": ["error", { ignoreConstructors: true }],
    "compiler-version": ["error", "^0.8.19"],
    "no-unused-vars": "warn",
    "max-line-length": ["warn", 120],
  },
  settings: {
    importRemappings: [
      ["@openzeppelin/", "lib/openzeppelin-contracts/"],
      ["@foundry/", "lib/foundry/"],
    ],
  },
};
