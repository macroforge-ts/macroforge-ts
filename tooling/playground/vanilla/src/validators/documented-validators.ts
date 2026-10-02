/**
 * One class per validator in the serde validators documentation, each holding
 * a single `value` field, so the documented behaviour of every validator is
 * checked by documented-validators.test.mjs.
 */

/** @derive(Deserialize) */
export class DocEmail {
    /** @serde({ validate: ["email"] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocUrl {
    /** @serde({ validate: ["url"] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocUuid {
    /** @serde({ validate: ["uuid"] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocPattern {
    /** @serde({ validate: ['pattern("^[A-Z]{3}$")'] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocPatternWithSlash {
    /** @serde({ validate: ['pattern("^a/b$")'] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocMinLength {
    /** @serde({ validate: ["minLength(3)"] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocMaxLength {
    /** @serde({ validate: ["maxLength(3)"] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocLength {
    /** @serde({ validate: ["length(3)"] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocLengthRange {
    /** @serde({ validate: ["length(2, 4)"] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocNonEmpty {
    /** @serde({ validate: ["nonEmpty"] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocTrimmed {
    /** @serde({ validate: ["trimmed"] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocLowercase {
    /** @serde({ validate: ["lowercase"] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocUppercase {
    /** @serde({ validate: ["uppercase"] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocCapitalized {
    /** @serde({ validate: ["capitalized"] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocUncapitalized {
    /** @serde({ validate: ["uncapitalized"] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocStartsWith {
    /** @serde({ validate: ['startsWith("pre")'] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocStartsWithQuote {
    /** @serde({ validate: ['startsWith("a\\"b")'] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocEndsWith {
    /** @serde({ validate: ['endsWith("fix")'] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocIncludes {
    /** @serde({ validate: ['includes("mid")'] }) */
    value: string;
}

/** @derive(Deserialize) */
export class DocGreaterThan {
    /** @serde({ validate: ["greaterThan(5)"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocGreaterThanOrEqualTo {
    /** @serde({ validate: ["greaterThanOrEqualTo(5)"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocLessThan {
    /** @serde({ validate: ["lessThan(5)"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocLessThanOrEqualTo {
    /** @serde({ validate: ["lessThanOrEqualTo(5)"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocBetween {
    /** @serde({ validate: ["between(1, 10)"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocInt {
    /** @serde({ validate: ["int"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocNonNegativeInt {
    /** @serde({ validate: ["nonNegativeInt"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocUint8 {
    /** @serde({ validate: ["uint8"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocMultipleOfDecimal {
    /** @serde({ validate: ["multipleOf(0.1)"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocMultipleOfInteger {
    /** @serde({ validate: ["multipleOf(5)"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocPositive {
    /** @serde({ validate: ["positive"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocNonNegative {
    /** @serde({ validate: ["nonNegative"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocNegative {
    /** @serde({ validate: ["negative"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocNonPositive {
    /** @serde({ validate: ["nonPositive"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocFinite {
    /** @serde({ validate: ["finite"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocNonNaN {
    /** @serde({ validate: ["nonNaN"] }) */
    value: number;
}

/** @derive(Deserialize) */
export class DocPositiveBigInt {
    /** @serde({ validate: ["positiveBigInt"] }) */
    value: bigint;
}

/** @derive(Deserialize) */
export class DocNonNegativeBigInt {
    /** @serde({ validate: ["nonNegativeBigInt"] }) */
    value: bigint;
}

/** @derive(Deserialize) */
export class DocNegativeBigInt {
    /** @serde({ validate: ["negativeBigInt"] }) */
    value: bigint;
}

/** @derive(Deserialize) */
export class DocNonPositiveBigInt {
    /** @serde({ validate: ["nonPositiveBigInt"] }) */
    value: bigint;
}

/** @derive(Deserialize) */
export class DocGreaterThanBigInt {
    /** @serde({ validate: ["greaterThanBigInt(5)"] }) */
    value: bigint;
}

/** @derive(Deserialize) */
export class DocGreaterThanOrEqualToBigInt {
    /** @serde({ validate: ["greaterThanOrEqualToBigInt(5)"] }) */
    value: bigint;
}

/** @derive(Deserialize) */
export class DocLessThanBigInt {
    /** @serde({ validate: ["lessThanBigInt(5)"] }) */
    value: bigint;
}

/** @derive(Deserialize) */
export class DocLessThanOrEqualToBigInt {
    /** @serde({ validate: ["lessThanOrEqualToBigInt(5)"] }) */
    value: bigint;
}

/** @derive(Deserialize) */
export class DocBetweenBigInt {
    /** @serde({ validate: ["betweenBigInt(1, 10)"] }) */
    value: bigint;
}

/** @derive(Deserialize) */
export class DocValidDate {
    /** @serde({ validate: ["validDate"] }) */
    value: Date;
}

/** @derive(Deserialize) */
export class DocGreaterThanDate {
    /** @serde({ validate: ['greaterThanDate("2020-01-01")'] }) */
    value: Date;
}

/** @derive(Deserialize) */
export class DocGreaterThanOrEqualToDate {
    /** @serde({ validate: ['greaterThanOrEqualToDate("2020-01-01")'] }) */
    value: Date;
}

/** @derive(Deserialize) */
export class DocLessThanDate {
    /** @serde({ validate: ['lessThanDate("2020-01-01")'] }) */
    value: Date;
}

/** @derive(Deserialize) */
export class DocLessThanOrEqualToDate {
    /** @serde({ validate: ['lessThanOrEqualToDate("2020-01-01")'] }) */
    value: Date;
}

/** @derive(Deserialize) */
export class DocBetweenDate {
    /** @serde({ validate: ['betweenDate("2020-01-01", "2020-12-31")'] }) */
    value: Date;
}

/** @derive(Deserialize) */
export class DocMinItems {
    /** @serde({ validate: ["minItems(2)"] }) */
    value: number[];
}

/** @derive(Deserialize) */
export class DocMaxItems {
    /** @serde({ validate: ["maxItems(2)"] }) */
    value: number[];
}

/** @derive(Deserialize) */
export class DocItemsCount {
    /** @serde({ validate: ["itemsCount(2)"] }) */
    value: number[];
}

/** @derive(Deserialize) */
export class DocNullableMaxLength {
    /** @serde({ validate: ["maxLength(3)"] }) */
    value: string | null;
}

/** @derive(Deserialize) */
export class DocNullableDate {
    /** @serde({ validate: ["greaterThanDate(\"2020-01-01\")"] }) */
    value: Date | null;
}

/** @derive(Deserialize) */
export class DocMessageEscaping {
    /** @serde({ validate: [{ validate: "nonEmpty", message: "say \"hi\"\nthen stop" }] }) */
    value: string;
}
