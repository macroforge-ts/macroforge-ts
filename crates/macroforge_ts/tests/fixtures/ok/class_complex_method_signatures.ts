import { Derive } from '@macro/derive';

/** @derive(Debug) */
class API {
    endpoint: string;

    constructor(endpoint: string) {
        this.endpoint = endpoint;
    }

    async fetch<T>(
        path: string,
        options?: { method?: string; body?: any }
    ): Promise<T> {
        return {} as T;
    }

    subscribe(
        event: 'data' | 'error',
        callback: (data: any) => void,
        thisArg?: any
    ): () => void {
        return () => {};
    }
}
