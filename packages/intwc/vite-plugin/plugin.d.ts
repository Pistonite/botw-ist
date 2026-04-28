
export type Options = {
    /**
     * Basic languages to load
     */
    basicLanguages: string[];

    /** If TypeScript worker should be loaded */
    typescript?: boolean;
    /** If JSON worker should be loaded */
    json?: boolean;
    /** If CSS worker should be loaded */
    css?: boolean;
    /** If HTML worker should be loaded */
    html?: boolean;
};

export default function plugin(options: Options): any;
