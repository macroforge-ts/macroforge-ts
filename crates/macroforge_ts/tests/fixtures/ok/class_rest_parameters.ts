import { Derive } from "@macro/derive";

/** @derive(Clone) */
class EventEmitter {
    listeners: Map<string, Function[]>;

    constructor() {
        this.listeners = new Map();
    }

    on(event: string, ...callbacks: Array<(...args: any[]) => void>): void {
        const existing = this.listeners.get(event) || [];
        this.listeners.set(event, [...existing, ...callbacks]);
    }

    emit(event: string, ...args: any[]): void {
        const callbacks = this.listeners.get(event) || [];
        callbacks.forEach(cb => cb(...args));
    }
}
