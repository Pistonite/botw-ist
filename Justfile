deploy:
    rm -rf docs/
    npm run build
    mv -T build docs
