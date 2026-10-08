import { expandExample } from '$lib/server/macroforge.ts';

export const load = async () => {
    const [fieldAttributes, deriveBasic, deriveMultiple] = await Promise.all([
        expandExample(
            `/** @derive(Debug, Encode) */
class User {
  /** @debug({ rename: "userId" }) */
  /** @endec({ rename: "user_id" }) */
  id: number;

  name: string;

  /** @debug({ skip: true }) */
  /** @endec({ skip: true }) */
  password: string;

  metadata: Record<string, unknown>;
}`,
            'derive-system-field-attributes.ts'
        ),
        expandExample(
            `/** @derive(Debug) */
class MyClass {
  value: string;
}`,
            'derive-basic.ts'
        ),
        expandExample(
            `/** @derive(Debug, Clone) */
class User {
  name: string;
  email: string;
}`,
            'derive-multiple.ts'
        )
    ]);

    return {
        examples: {
            fieldAttributes,
            deriveBasic,
            deriveMultiple
        }
    };
};
