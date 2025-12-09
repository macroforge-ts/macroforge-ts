import { expandExample } from '$lib/server/macroforge';

export function load() {
	return {
		examples: {
			basic: expandExample(`/** @derive(Deserialize) */
class User {
  name: string;
  age: number;
  createdAt: Date;
}`),
			rename: expandExample(`/** @derive(Deserialize) */
class User {
  /** @serde({ rename: "user_id" }) */
  id: string;

  /** @serde({ rename: "full_name" }) */
  name: string;
}`),
			default: expandExample(`/** @derive(Deserialize) */
class Config {
  host: string;

  /** @serde({ default: "3000" }) */
  port: string;

  /** @serde({ default: "false" }) */
  debug: boolean;
}`),
			interface: expandExample(`/** @derive(Deserialize) */
interface ApiResponse {
  status: number;
  message: string;
  timestamp: Date;
}`),
			enum: expandExample(`/** @derive(Deserialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}`),
			typeAlias: expandExample(`/** @derive(Deserialize) */
type UserProfile = {
  id: string;
  name: string;
  createdAt: Date;
};`)
		}
	};
}
