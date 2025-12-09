import { expandExample } from '$lib/server/macroforge';

export function load() {
	return {
		examples: {
			basic: expandExample(`/** @derive(Default) */
class Config {
  host: string;
  port: number;
  enabled: boolean;

  constructor(host: string, port: number, enabled: boolean) {
    this.host = host;
    this.port = port;
    this.enabled = enabled;
  }
}`),
			custom: expandExample(`/** @derive(Default) */
class ServerConfig {
  /** @defaultValue("localhost") */
  host: string;

  /** @defaultValue(8080) */
  port: number;

  /** @defaultValue(true) */
  enabled: boolean;

  /** @defaultValue(["info", "error"]) */
  logLevels: string[];

  constructor(host: string, port: number, enabled: boolean, logLevels: string[]) {
    this.host = host;
    this.port = port;
    this.enabled = enabled;
    this.logLevels = logLevels;
  }
}`),
			interface: expandExample(`/** @derive(Default) */
interface Point {
  x: number;
  y: number;
}`),
			enum: expandExample(`/** @derive(Default) */
enum Status {
  Pending = "pending",
  Active = "active",
  Completed = "completed",
}`),
			typeAlias: expandExample(`/** @derive(Default) */
type Dimensions = {
  width: number;
  height: number;
};`)
		}
	};
}
