# botw-ist
Simulator for Inventory Slot Transfer in BOTW

Visit the app at https://ist.itntpiston.app

## Contribute
Contributions are welcomed. For small issues, you can open PRs directly. For big issues contact me on discord iTNTPiston#5339

## Development
#### Clone Repo
Clone repo with `--recurse-submodules`

If you have already cloned, run `git submodule update --init --recursive`

#### Install/Update
To update submodules, go to the submodule directory and pull. Example:
```
cd scripts/base-lint
git pull
```

To install node modules `npm install`

#### Local
Run `npm run start` to start webpack dev server.
When code changes, the dev server will hot reload

#### PR
Do before PR:
- Lint your code `npm run lint-all && npm run lint-ts`
- Run tests `npm run test-all`
