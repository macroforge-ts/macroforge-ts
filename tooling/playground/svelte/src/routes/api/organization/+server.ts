import { json } from "@sveltejs/kit";
import { organizationFixture } from "$lib/e2e/fixture";

export function GET() {
  return json(organizationFixture);
}
