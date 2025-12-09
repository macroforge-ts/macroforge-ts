import { expandExample } from '$lib/server/macroforge';

export function load() {
	return {
		examples: {
			basic: expandExample(`import { Serialize } from "macroforge";

/** @derive(Serialize) */
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
			rename: expandExample(`import { Serialize } from "macroforge";

/** @derive(Serialize) */
class User {
  /** @serde({ rename: "user_id" }) */
  id: string;

  /** @serde({ rename: "full_name" }) */
  name: string;
}`),
			skip: expandExample(`import { Serialize } from "macroforge";

/** @derive(Serialize) */
class User {
  name: string;
  email: string;

  /** @serde({ skip: true }) */
  password: string;

  /** @serde({ skip_serializing: true }) */
  internalId: string;
}`),
			interface: expandExample(`import { Serialize } from "macroforge";

/** @derive(Serialize) */
interface ApiResponse {
  status: number;
  message: string;
  timestamp: Date;
}`),
			enum: expandExample(`import { Serialize } from "macroforge";

/** @derive(Serialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}`),
			typeAlias: expandExample(`import { Serialize } from "macroforge";

/** @derive(Serialize) */
type UserProfile = {
  id: string;
  name: string;
  createdAt: Date;
};`)
		}
	};
}
