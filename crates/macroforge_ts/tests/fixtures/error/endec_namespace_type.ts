import type * as models from './models';
import * as shapes from './shapes';

/** @derive(Encode, Decode) */
export interface Held {
    user: models.User;
    points: shapes.Point[];
    maybe: models.User | null;
}

/** A type two names deep is imported by the first of them. */
/** @derive(Encode) */
export interface Deep {
    at: models.Clock.Instant;
}
