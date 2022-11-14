//https://www.geeksforgeeks.org/longest-common-substring-dp-29/
// This code contributed by Rajput-Ji
// JavaScript implementation of
// finding length of longest
// Common substring using
// Dynamic Programming

/*
Returns length of longest common
substring of X[0..m-1] and Y[0..n-1]
*/
function LCSubStr(a: string, b: string ,m: number , n: number): number {
// Create a table to store
// lengths of longest common
// suffixes of substrings.
// Note that LCSuff[i][j]
// contains length of longest
// common suffix of
// X[0..i-1] and Y[0..j-1].
// The first row and first
// column entries have no
// logical meaning, they are
// used only for simplicity of program

const LCStuff = Array(m + 1).fill(undefined).map(()=>Array(n + 1).fill(0));

    // To store length of the longest
    // common substring
    let result = 0;

    // Following steps build
    // LCSuff[m+1][n+1] in bottom up fashion
    for (let i = 0; i <= m; i++) {
        for (let j = 0; j <= n; j++) {
            if (i == 0 || j == 0)
                LCStuff[i][j] = 0;
            else if (a[i - 1] == b[j - 1]) {
                LCStuff[i][j] = LCStuff[i - 1][j - 1] + 1;
                result = Math.max(result, LCStuff[i][j]);
            } else
                LCStuff[i][j] = 0;
        }
    }
    return result;
}

// Driver Code

// var X = "OldSite:GeeksforGeeks.org";
// var Y = "NewSite:GeeksQuiz.com";

// var m = X.length;
// var n = Y.length;

// document.write("Length of Longest Common Substring is " +
// LCSubStr(X, Y, m, n));


export const longestCommonSubstringLength = (a: string, b: string): number => {
    return LCSubStr(a,b,a.length,b.length);
}
