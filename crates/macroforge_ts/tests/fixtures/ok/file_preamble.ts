#!/usr/bin/env node
/// <reference lib="esnext" />
// @ts-check
'use client';

/** The first declaration's own comment. */
export const first = 1;

/** @derive(Clone, PartialEq) */
export interface Point {
    x: number;
    y: number;
}
