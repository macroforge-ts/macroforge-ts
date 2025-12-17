import { expandExample } from '$lib/server/macroforge';
import { getBuiltinMacro, getVersion } from '$lib/server/api-docs';

export async function load() {
	return {
		version: getVersion('rust', 'macroforge_ts'),
		macro: getBuiltinMacro('default'),
		examples: {
			basic: await expandExample(`/** @derive(Default) */
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
			nullable: await expandExample(`/** @derive(Default) */
interface User {
  name: string;
  email: string | null;
  age: number;
  metadata: Record<string, unknown> | null;
}`),
			customType: await expandExample(`/** @derive(Default) */
interface AppConfig {
  name: string;
  port: number;
  /** @default(Settings.defaultValue()) */
  settings: Settings;
  /** @default(Permissions.defaultValue()) */
  permissions: Permissions;
}`),
			custom: await expandExample(`/** @derive(Default) */
class ServerConfig {
  /** @default("localhost") */
  host: string;

  /** @default(8080) */
  port: number;

  /** @default(true) */
  enabled: boolean;

  /** @default(["info", "error"]) */
  logLevels: string[];

  constructor(host: string, port: number, enabled: boolean, logLevels: string[]) {
    this.host = host;
    this.port = port;
    this.enabled = enabled;
    this.logLevels = logLevels;
  }
}`),
			interface: await expandExample(`/** @derive(Default) */
interface Point {
  x: number;
  y: number;
}`),
			enum: await expandExample(`/** @derive(Default) */
enum Status {
  Pending = "pending",
  Active = "active",
  Completed = "completed",
}`),
			typeAlias: await expandExample(`/** @derive(Default) */
type Dimensions = {
  width: number;
  height: number;
};`)
		}
	};
}
