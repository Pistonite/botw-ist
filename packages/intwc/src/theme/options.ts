export type ThemeOptions = {
    customTokenColors?: CustomTokenColor[];
}

export type CustomTokenColor = {
    /** 
     * The token to set the color for.
     *
     * For example: string.regexp
     */
    token: string;

    /**
     * The color to set the token to.
     *
     * This can either be "#" followed by 6 hexadecimal digits to 
     * set the color exactly, or another token name to use the color
     * of that token.
     *
     * If the color needs to be different for light and dark mode
     * (which is almost always the case), use an array of 2 colors [light, dark]
     */
    value: string | [string, string];
}
