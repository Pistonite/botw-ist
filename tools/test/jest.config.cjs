const path = require("path");

module.exports = {
    rootDir: path.resolve(__dirname, "../../"),
    testEnvironment: "jsdom",
    setupFilesAfterEnv: ["<rootDir>/tools/test/jest.setup.ts"],
    moduleNameMapper: {
        "\\.(css|less|scss|sass)$": "identity-obj-proxy",
        "^ui/(.*)": "<rootDir>/src/ui/$1",
        "^core/(.*)": "<rootDir>/src/core/$1",
        "^data/(.*)": "<rootDir>/src/data/$1",
        "^@test/(.*)": "<rootDir>/tools/test/$1",
    },
    transform: {
        "\\.[jt]sx?$": [
            "babel-jest",
            {
                presets: [
                    [
                        "@babel/preset-env",
                        {
                            targets: {
                                node: "current",
                            },
                        },
                    ],
                    "@babel/preset-react",
                    "@babel/preset-typescript",
                ],
            },
        ],
    },
    transformIgnorePatterns: [
        "/node_modules/(?!yaml)"
    ],
};
