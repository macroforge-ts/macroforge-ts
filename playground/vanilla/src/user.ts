import { Derive } from "./macros";

// Example of using Derive decorator and dynamic macro
@Derive("Debug", "JsonNative")
export class User {
  constructor(
    public id: number,
    public name: string,
    public email: string,
  ) {}
}
