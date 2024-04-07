/// <reference types="vite/client" />

declare module '*.lang.yaml' {
  type StringTree = { readonly [key: string]: StringTree | string };
  const classes: StringTree;
  export default classes;
}
