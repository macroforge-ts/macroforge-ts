import { assertEquals } from 'jsr:@std/assert@1';
import { parseMacroImportComments } from '../src/macro-imports.ts';

Deno.test('parseMacroImportComments - parses single macro import', () => {
    const text = `/** import macro {JSON} from "@playground/macro"; */`;
    const result = parseMacroImportComments(text);

    assertEquals(result.size, 1);
    assertEquals(result.get('JSON'), '@playground/macro');
});

Deno.test('parseMacroImportComments - parses multiple macros from same package', () => {
    const text = `/** import macro {JSON, FieldController, Gigaform} from "@playground/macro"; */`;
    const result = parseMacroImportComments(text);

    assertEquals(result.size, 3);
    assertEquals(result.get('JSON'), '@playground/macro');
    assertEquals(result.get('FieldController'), '@playground/macro');
    assertEquals(result.get('Gigaform'), '@playground/macro');
});

Deno.test('parseMacroImportComments - parses imports from multiple packages', () => {
    const text = `
        /** import macro {JSON} from "@playground/macro"; */
        /** import macro {Debug} from "macroforge"; */
    `;
    const result = parseMacroImportComments(text);

    assertEquals(result.size, 2);
    assertEquals(result.get('JSON'), '@playground/macro');
    assertEquals(result.get('Debug'), 'macroforge');
});

Deno.test('parseMacroImportComments - handles single quotes', () => {
    const text = `/** import macro {JSON} from '@playground/macro'; */`;
    const result = parseMacroImportComments(text);

    assertEquals(result.size, 1);
    assertEquals(result.get('JSON'), '@playground/macro');
});

Deno.test('parseMacroImportComments - handles whitespace variations', () => {
    const text = `/**   import   macro   {  JSON  ,  Debug  }   from   "@playground/macro"  ; */`;
    const result = parseMacroImportComments(text);

    assertEquals(result.size, 2);
    assertEquals(result.get('JSON'), '@playground/macro');
    assertEquals(result.get('Debug'), '@playground/macro');
});

Deno.test('parseMacroImportComments - returns empty map for no imports', () => {
    const text = `
        // Regular comment
        const x = 1;
        /** Regular JSDoc comment */
    `;
    const result = parseMacroImportComments(text);

    assertEquals(result.size, 0);
});

Deno.test('parseMacroImportComments - ignores regular imports', () => {
    const text = `
        import { something } from "some-package";
        /** import macro {JSON} from "@playground/macro"; */
    `;
    const result = parseMacroImportComments(text);

    assertEquals(result.size, 1);
    assertEquals(result.get('JSON'), '@playground/macro');
});

Deno.test('parseMacroImportComments - handles scoped packages', () => {
    const text = `/** import macro {Form} from "@myorg/my-macros"; */`;
    const result = parseMacroImportComments(text);

    assertEquals(result.size, 1);
    assertEquals(result.get('Form'), '@myorg/my-macros');
});

Deno.test('parseMacroImportComments - handles multiline macro import comment', () => {
    const text = `/**
        import macro {JSON, Debug, Clone} from "@playground/macro";
    */`;
    const result = parseMacroImportComments(text);

    assertEquals(result.size, 3);
    assertEquals(result.get('JSON'), '@playground/macro');
    assertEquals(result.get('Debug'), '@playground/macro');
    assertEquals(result.get('Clone'), '@playground/macro');
});

Deno.test('parseMacroImportComments - case insensitive matching', () => {
    const text = `/** IMPORT MACRO {JSON} FROM "@playground/macro"; */`;
    const result = parseMacroImportComments(text);

    assertEquals(result.size, 1);
    assertEquals(result.get('JSON'), '@playground/macro');
});

Deno.test('parseMacroImportComments - last import wins for duplicate macro names', () => {
    const text = `
        /** import macro {JSON} from "package-a"; */
        /** import macro {JSON} from "package-b"; */
    `;
    const result = parseMacroImportComments(text);

    assertEquals(result.size, 1);
    assertEquals(result.get('JSON'), 'package-b');
});
