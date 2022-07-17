lint VERBOSE="": 
    pylint scripts
    python3 scripts/lint.py {{VERBOSE}}
    npm run lint

test:
    npm run test

# make sure this passes before you PR
check: lint test
