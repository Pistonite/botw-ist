//https://medium.com/@fsufitch/is-javascript-array-sort-stable-46b90822543f
export const stableSort = <T>(array: T[], cmp: (a:T, b:T) => number): void => {
    const stabilizedThis: [T, number][] = array.map((el, index) => [el, index]);
    const stableCmp = (a: [T, number], b: [T, number]) => {
      let order = cmp(a[0], b[0]);
      if (order != 0) return order;
      return a[1] - b[1];
    }
    stabilizedThis.sort(stableCmp);
    
    for (let i=0; i<array.length; i++) {
        array[i] = stabilizedThis[i][0];
    }
}
