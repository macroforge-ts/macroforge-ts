import { expandExample } from '$lib/server/macroforge';
import { getBuiltinMacro, getVersion } from '$lib/server/api-docs';

export async function load() {
	return {
		version: getVersion('rust', 'macroforge_ts'),
		macro: getBuiltinMacro('deserialize'),
		examples: {
			basic: await expandExample(`/** @derive(Deserialize) */
class User {
  name: string;
  age: number;
  createdAt: Date;
}`),
			rename: await expandExample(`/** @derive(Deserialize) */
class User {
  /** @serde({ rename: "user_id" }) */
  id: string;

  /** @serde({ rename: "full_name" }) */
  name: string;
}`),
			default: await expandExample(`/** @derive(Deserialize) */
class Config {
  host: string;

  /** @serde({ default: "3000" }) */
  port: string;

  /** @serde({ default: "false" }) */
  debug: boolean;
}`),
			interface: await expandExample(`/** @derive(Deserialize) */
interface ApiResponse {
  status: number;
  message: string;
  timestamp: Date;
}`),
			enum: await expandExample(`/** @derive(Deserialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}`),
			typeAlias: await expandExample(`/** @derive(Deserialize) */
type UserProfile = {
  id: string;
  name: string;
  createdAt: Date;
};`)
		}
	};
}
