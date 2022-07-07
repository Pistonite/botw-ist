deploy:
    rm -rf docs/
    npm run build
    mv -T build docs
    git add .
    git commit -m "Local Build Deployment"
    git push

lint VERBOSE="": 
    pylint scripts
    python3 scripts/lint.py {{VERBOSE}}
    npm run lint
