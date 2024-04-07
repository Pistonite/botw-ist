/* eslint-disable @typescript-eslint/no-unused-vars */
  declare namespace jest {
    interface Matchers<R> {
      toEqualItemStacks(expected: ItemStack[], eq?: (a: ItemStack, b: ItemStack) => boolean): CustomMatcherResult;
            toPassE2ESimulation(debug?: boolean): CustomMatcherResult;
            toMatchItemSearch(result: string | ItemStack): CustomMatcherResult;
            toParseIntoCommand(search: ItemSearchFunction, expected: Command | CmdErr): CustomMatcherResult;
    }
  }

