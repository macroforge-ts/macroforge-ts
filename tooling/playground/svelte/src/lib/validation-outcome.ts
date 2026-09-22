import { Cause, Exit, Option } from 'effect';

/** A form's `validate()` result, as the pages render and publish it. */
export type ValidationOutcome<Data, Failure> =
    | { success: true; data: Data }
    | { success: false; errors: Failure };

/** The outcome a form's `validate()` produces. */
export type OutcomeOf<Form extends { validate(): Exit.Exit<unknown, unknown> }> =
    ReturnType<Form['validate']> extends Exit.Exit<infer Data, infer Failure>
        ? ValidationOutcome<Data, Failure>
        : never;

/**
 * Converts a form's validation `Exit`. Only an expected failure becomes
 * `errors`; a defect is rethrown rather than rendered as a validation result.
 */
export function validationOutcome<Data, Failure>(
    exit: Exit.Exit<Data, Failure>
): ValidationOutcome<Data, Failure> {
    return Exit.match(exit, {
        onSuccess: (data) => ({ success: true, data }),
        onFailure: (cause) => ({
            success: false,
            errors: Option.getOrThrowWith(Cause.failureOption(cause), () => Cause.squash(cause))
        })
    });
}
