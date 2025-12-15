import { expandExample } from '$lib/server/macroforge';
import { getBuiltinMacro, getVersion } from '$lib/server/api-docs';

export function load() {
	return {
		version: getVersion('rust', 'macroforge_ts'),
		macro: getBuiltinMacro('serialize'),
		examples: {
			basic: expandExample(`/** @derive(Serialize) */
class User {
  name: string;
  age: number;
  createdAt: Date;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
    this.createdAt = new Date();
  }
}`),
			rename: expandExample(`/** @derive(Serialize) */
class User {
  /** @serde({ rename: "user_id" }) */
  id: string;

  /** @serde({ rename: "full_name" }) */
  name: string;
}`),
			skip: expandExample(`/** @derive(Serialize) */
class User {
  name: string;
  email: string;

  /** @serde({ skip: true }) */
  password: string;

  /** @serde({ skip_serializing: true }) */
  internalId: string;
}`),
			interface: expandExample(`/** @derive(Serialize) */
interface ApiResponse {
  status: number;
  message: string;
  timestamp: Date;
}`),
			enum: expandExample(`/** @derive(Serialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}`),
			typeAlias: expandExample(`/** @derive(Serialize) */
type UserProfile = {
  id: string;
  name: string;
  createdAt: Date;
};`)
		}
	};
}
