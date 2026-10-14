/** A promise is settled on its value's default, and a generic type defaults through its own function. */
/** @derive(Default) */
export interface Holder {
    pending: Promise<string>;
    done: Promise<void>;
    boxed: String;
    anything: Object;
    wrapped: Box<string>;
    later: [string, URL?];
    optional: { at?: URL };
}
