import { Derive } from "./macros";

// Example of using Derive decorator
@Derive("Debug", "JSON")
export class User {
  constructor(
    public id: number,
    public name: string,
    public email: string,
  ) {}
}
