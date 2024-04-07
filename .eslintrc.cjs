module.exports = {
    root: true,
    env: { browser: true, es2020: true },
    extends: [
        "eslint:recommended",
        "plugin:@typescript-eslint/recommended",
        "plugin:react-hooks/recommended",
    ],
    ignorePatterns: ["dist", ".eslintrc.cjs", "*.generated.ts"],
    parser: "@typescript-eslint/parser",
    plugins: ["react-refresh"],
    rules: {
        "@typescript-eslint/no-unused-vars": [
            "warn",
            {
                varsIgnorePattern: "_",
                argsIgnorePattern: "_",
            },
        ],
        "no-constant-condition": ["error", { checkLoops: false }],
        "no-multiple-empty-lines": [
            "warn",
            {
                max: 1,
            },
        ],
        "no-console": [
            "warn",
            {
                allow: ["error", "warn"],
            },
        ],
        "no-unreachable-loop": ["error"],
        curly: ["warn", "all"],
        "react-hooks/exhaustive-deps": "off",
        "react-refresh/only-export-components": "off",
        // 'react-refresh/only-export-components': [
        //   'warn',
        //   { allowConstantExport: true },
        // ],
    },
};
