deploy:
    rm -rf docs/
    npm run build
    mv -T build docs
    git add .
    git commit -m "Local Build Deployment"
    git push