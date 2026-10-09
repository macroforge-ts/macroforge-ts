/**
 * One class per validator in the endec validators documentation, each holding
 * a single `value` field, so the documented behaviour of every validator is
 * checked by documented-validators.test.mjs.
 */

/** @derive(Decode) */
export class DocEmail {
    /** @endec({ validate: ["email"] }) */
    value: string;
}

/** @derive(Decode) */
export class DocUrl {
    /** @endec({ validate: ["url"] }) */
    value: string;
}

/** @derive(Decode) */
export class DocUuid {
    /** @endec({ validate: ["uuid"] }) */
    value: string;
}

/** @derive(Decode) */
export class DocPattern {
    /** @endec({ validate: ['pattern("^[A-Z]{3}$")'] }) */
    value: string;
}

/** @derive(Decode) */
export class DocPatternWithSlash {
    /** @endec({ validate: ['pattern("^a/b$")'] }) */
    value: string;
}

/** @derive(Decode) */
export class DocMinLength {
    /** @endec({ validate: ["minLength(3)"] }) */
    value: string;
}

/** @derive(Decode) */
export class DocMaxLength {
    /** @endec({ validate: ["maxLength(3)"] }) */
    value: string;
}

/** @derive(Decode) */
export class DocLength {
    /** @endec({ validate: ["length(3)"] }) */
    value: string;
}

/** @derive(Decode) */
export class DocLengthRange {
    /** @endec({ validate: ["length(2, 4)"] }) */
    value: string;
}

/** @derive(Decode) */
export class DocNonEmpty {
    /** @endec({ validate: ["nonEmpty"] }) */
    value: string;
}

/** @derive(Decode) */
export class DocTrimmed {
    /** @endec({ validate: ["trimmed"] }) */
    value: string;
}

/** @derive(Decode) */
export class DocLowercase {
    /** @endec({ validate: ["lowercase"] }) */
    value: string;
}

/** @derive(Decode) */
export class DocUppercase {
    /** @endec({ validate: ["uppercase"] }) */
    value: string;
}

/** @derive(Decode) */
export class DocCapitalized {
    /** @endec({ validate: ["capitalized"] }) */
    value: string;
}

/** @derive(Decode) */
export class DocUncapitalized {
    /** @endec({ validate: ["uncapitalized"] }) */
    value: string;
}

/** @derive(Decode) */
export class DocStartsWith {
    /** @endec({ validate: ['startsWith("pre")'] }) */
    value: string;
}

/** @derive(Decode) */
export class DocStartsWithQuote {
    /** @endec({ validate: ['startsWith("a\\"b")'] }) */
    value: string;
}

/** @derive(Decode) */
export class DocEndsWith {
    /** @endec({ validate: ['endsWith("fix")'] }) */
    value: string;
}

/** @derive(Decode) */
export class DocIncludes {
    /** @endec({ validate: ['includes("mid")'] }) */
    value: string;
}

/** @derive(Decode) */
export class DocGreaterThan {
    /** @endec({ validate: ["greaterThan(5)"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocGreaterThanOrEqualTo {
    /** @endec({ validate: ["greaterThanOrEqualTo(5)"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocLessThan {
    /** @endec({ validate: ["lessThan(5)"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocLessThanOrEqualTo {
    /** @endec({ validate: ["lessThanOrEqualTo(5)"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocBetween {
    /** @endec({ validate: ["between(1, 10)"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocInt {
    /** @endec({ validate: ["int"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocNonNegativeInt {
    /** @endec({ validate: ["nonNegativeInt"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocUint8 {
    /** @endec({ validate: ["uint8"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocMultipleOfDecimal {
    /** @endec({ validate: ["multipleOf(0.1)"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocMultipleOfInteger {
    /** @endec({ validate: ["multipleOf(5)"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocPositive {
    /** @endec({ validate: ["positive"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocNonNegative {
    /** @endec({ validate: ["nonNegative"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocNegative {
    /** @endec({ validate: ["negative"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocNonPositive {
    /** @endec({ validate: ["nonPositive"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocFinite {
    /** @endec({ validate: ["finite"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocNonNaN {
    /** @endec({ validate: ["nonNaN"] }) */
    value: number;
}

/** @derive(Decode) */
export class DocPositiveBigInt {
    /** @endec({ validate: ["positiveBigInt"] }) */
    value: bigint;
}

/** @derive(Decode) */
export class DocNonNegativeBigInt {
    /** @endec({ validate: ["nonNegativeBigInt"] }) */
    value: bigint;
}

/** @derive(Decode) */
export class DocNegativeBigInt {
    /** @endec({ validate: ["negativeBigInt"] }) */
    value: bigint;
}

/** @derive(Decode) */
export class DocNonPositiveBigInt {
    /** @endec({ validate: ["nonPositiveBigInt"] }) */
    value: bigint;
}

/** @derive(Decode) */
export class DocGreaterThanBigInt {
    /** @endec({ validate: ["greaterThanBigInt(5)"] }) */
    value: bigint;
}

/** @derive(Decode) */
export class DocGreaterThanOrEqualToBigInt {
    /** @endec({ validate: ["greaterThanOrEqualToBigInt(5)"] }) */
    value: bigint;
}

/** @derive(Decode) */
export class DocLessThanBigInt {
    /** @endec({ validate: ["lessThanBigInt(5)"] }) */
    value: bigint;
}

/** @derive(Decode) */
export class DocLessThanOrEqualToBigInt {
    /** @endec({ validate: ["lessThanOrEqualToBigInt(5)"] }) */
    value: bigint;
}

/** @derive(Decode) */
export class DocBetweenBigInt {
    /** @endec({ validate: ["betweenBigInt(1, 10)"] }) */
    value: bigint;
}

/** @derive(Decode) */
export class DocValidDate {
    /** @endec({ validate: ["validDate"] }) */
    value: Date;
}

/** @derive(Decode) */
export class DocGreaterThanDate {
    /** @endec({ validate: ['greaterThanDate("2020-01-01")'] }) */
    value: Date;
}

/** @derive(Decode) */
export class DocGreaterThanOrEqualToDate {
    /** @endec({ validate: ['greaterThanOrEqualToDate("2020-01-01")'] }) */
    value: Date;
}

/** @derive(Decode) */
export class DocLessThanDate {
    /** @endec({ validate: ['lessThanDate("2020-01-01")'] }) */
    value: Date;
}

/** @derive(Decode) */
export class DocLessThanOrEqualToDate {
    /** @endec({ validate: ['lessThanOrEqualToDate("2020-01-01")'] }) */
    value: Date;
}

/** @derive(Decode) */
export class DocBetweenDate {
    /** @endec({ validate: ['betweenDate("2020-01-01", "2020-12-31")'] }) */
    value: Date;
}

/** @derive(Decode) */
export class DocMinItems {
    /** @endec({ validate: ["minItems(2)"] }) */
    value: number[];
}

/** @derive(Decode) */
export class DocMaxItems {
    /** @endec({ validate: ["maxItems(2)"] }) */
    value: number[];
}

/** @derive(Decode) */
export class DocItemsCount {
    /** @endec({ validate: ["itemsCount(2)"] }) */
    value: number[];
}

/** @derive(Decode) */
export class DocNullableMaxLength {
    /** @endec({ validate: ["maxLength(3)"] }) */
    value: string | null;
}

/** @derive(Decode) */
export class DocNullableDate {
    /** @endec({ validate: ["greaterThanDate(\"2020-01-01\")"] }) */
    value: Date | null;
}

/** @derive(Decode) */
export class DocMessageEscaping {
    /** @endec({ validate: [{ validate: "nonEmpty", message: "say \"hi\"\nthen stop" }] }) */
    value: string;
}
