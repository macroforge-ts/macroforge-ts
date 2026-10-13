import type { Named, Dated } from './elsewhere';

/** @derive(Clone, PartialEq, Encode, Decode) */
export type Both = Named & Dated;

/** @derive(Encode, Decode) */
export type Mixed = Named & { extra: number };
