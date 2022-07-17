# E2E Tests
## How to add a test
1. Make up a name for the test
2. Write the script in simulator
3. Export the script and put it as `<name>.in.txt` in `src/__tests__`
4. Write another script that uses `initialize`, `save`, `save as` and `break x slots` to achieve the expected result as the first script (look at one of the existing tests as an example)
5. Export the new script and put it as `<name>.out.txt` in `src/__tests__`
6. Create a new file `<name>.e2e.ts`
7. copy paste the following and replace `<name>` with the name of your test. Replace `YOUR_NAME` with your name
```typescript
// Author: YOUR_NAME
const TEST = "<name>";
it(TEST, ()=>{
    expect(TEST).toPassE2ESimulation();
});
export {};
```
8. Make a PR on https://github.com/iTNTPiston/botw-hundo-dupl
## How to run E2E tests
`npm run test-all`

## How to debug E2E tests
1. Open the `e2e.ts` file of the failed test
2. replace `it` with `it.only`
3. replace `toPassE2ESimulation()` with `toPassE2ESimulation(true)` (i.e. type `true` in the parenthese)
4. Run the test again, you should now see `debug.expected.log` and `debug.actual.log` containing the expected and actual content of the simuation states.
5. After fixing the test, revert the changes to `e2e.ts` you made
