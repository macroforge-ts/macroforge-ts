// Regression fixture for block-bodied declarative macros.
//
// Rust-like block semantics: the last expression in the block body is
// the block's value. JavaScript's arrow-function block bodies do NOT
// auto-return their last expression, so the expander rewrites
//
//   (() => { ...decls...; __trailing })()
// →
//   (() => { ...decls...; return __trailing })()
//
// Each macro below exercises a distinct scenario for the return-injection
// pass. Lexical-surface cases that can't be expressed safely inside an
// outer macroRules template literal (nested templates, nested regex with
// literal `;`) are covered by the unit tests in `expander.rs`.
//
//   basicBlock          — trailing binary expression
//   trailingArrow       — arrow expression as the final value
//   trailingObjectLit   — parenthesized object literal as the final value
//   trailingSatisfies   — TS `satisfies` expression
//   withReturn          — explicit `return` respected (no double-inject)
//   withSemi            — explicit trailing `;` means "discard", not "return"
//   nestedBlockInBody   — trailing statement is a block: don't touch it

import { macroRules } from 'macroforge/rules';

const $basicBlock = macroRules`
  ($x:Expr) => {
    const __v = $x;
    __v + 1
  }
`;

const $trailingArrow = macroRules`
  ($x:Expr) => {
    const __y = $x;
    () => __y
  }
`;

const $trailingObjectLit = macroRules`
  ($x:Expr) => {
    const __k = "value";
    ({ [__k]: $x })
  }
`;

const $trailingSatisfies = macroRules`
  ($x:Expr) => {
    const __v = $x;
    __v satisfies number
  }
`;

const $withReturn = macroRules`
  ($x:Expr) => {
    const __v = $x;
    return __v * 3;
  }
`;

const $withSemi = macroRules`
  ($x:Expr) => {
    const __v = $x;
    __v + 4;
  }
`;

const $nestedBlockInBody = macroRules`
  ($x:Expr) => {
    const __v = $x;
    { __v + 5 }
  }
`;

export const basicBlock = $basicBlock(10);
export const trailingArrowFn = $trailingArrow(42);
export const trailingObject = $trailingObjectLit(7);
export const trailingSatisfies = $trailingSatisfies(9);
export const withReturnResult = $withReturn(6);
export const withSemiResult = $withSemi(8);
export const nestedBlock = $nestedBlockInBody(11);
