import type { Result, Void } from "@pistonite/pure/result";

import type { MaybeAborted } from "@pistonite/skybook-api";

/**
 * Errors that can happen in the worker thread.
 *
 * With the exception of Aborted, these are only for *unexpected* errors.
 */
export type WorkerError = {
    message?: string;
} & (
    | {
          type: "Aborted";
      }
    | {
          type: "NativePanic";
      }
    | {
          type: "UnexpectedNullptr";
      }
    | {
          type: "UnexpectedThrow";
      }
);

export type Pwr<T> = Promise<Result<T, WorkerError>>;
export type Pwv = Promise<Void<WorkerError>>;

export const nullptrError = (message: string): WorkerError => ({
    type: "UnexpectedNullptr",
    message,
});

export const abortedError = () => {
    return {
        err: {
            type: "Aborted",
        } satisfies WorkerError,
    } as const;
};

export const unwrap = <T>(result: Result<T, WorkerError>): T => {
    if (result.err) {
        const { type, message } = result.err;
        const messagePart = message ? `: ${message}` : "";
        throw new Error(`WorkerError: ${type}${messagePart}`);
    }
    return result.val;
};

export const unwrapMaybeAborted = <T>(
    result: Result<T, WorkerError>,
): MaybeAborted<T> => {
    if (result.err) {
        if (result.err.type === "Aborted") {
            return { type: "Aborted" };
        }
        const { type, message } = result.err;
        throw new Error(`WorkerError: ${type}${message ? "" : ": " + message}`);
    }
    return { type: "Ok", value: result.val };
};
