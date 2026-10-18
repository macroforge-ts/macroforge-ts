import { assertEquals } from 'jsr:@std/assert@1';
import { declarationFileOf } from '../src/source-file.ts';

Deno.test('each module declares into a file of its own kind', () => {
    assertEquals(declarationFileOf('src/user.ts'), 'src/user.d.ts');
    assertEquals(declarationFileOf('src/view.tsx'), 'src/view.d.ts');
    assertEquals(declarationFileOf('src/user.mts'), 'src/user.d.mts');
    assertEquals(declarationFileOf('src/user.cts'), 'src/user.d.cts');
    assertEquals(declarationFileOf('src/card.svelte.ts'), 'src/card.svelte.d.ts');
});
