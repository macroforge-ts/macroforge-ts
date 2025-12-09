import { expandExample } from '$lib/server/macroforge';

export function load() {
	return {
		examples: {
			basic: expandExample(`import { Deserialize } from "macroforge";

/** @derive(Deserialize) */
class User {
  name: string;
  age: number;
  createdAt: Date;
}`),
			rename: expandExample(`import { Deserialize } from "macroforge";

/** @derive(Deserialize) */
class User {
  /** @serde({ rename: "user_id" }) */
  id: string;

  /** @serde({ rename: "full_name" }) */
  name: string;
}`),
			default: expandExample(`import { Deserialize } from "macroforge";

/** @derive(Deserialize) */
class Config {
  host: string;

  /** @serde({ default: "3000" }) */
  port: string;

  /** @serde({ default: "false" }) */
  debug: boolean;
}`),
			interface: expandExample(`import { Deserialize } from "macroforge";

/** @derive(Deserialize) */
interface ApiResponse {
  status: number;
  message: string;
  timestamp: Date;
}`),
			enum: expandExample(`import { Deserialize } from "macroforge";

/** @derive(Deserialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}`),
			typeAlias: expandExample(`import { Deserialize } from "macroforge";

/** @derive(Deserialize) */
type UserProfile = {
  id: string;
  name: string;
  createdAt: Date;
};`)
		}
	};
}
