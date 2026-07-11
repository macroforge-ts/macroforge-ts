<script lang="ts" module>
    /** @derive(Default, Serialize) */
    export interface CardLabels {
        heading: string;
    }
</script>

<script lang="ts">
    import type { PersonName } from './types/person-name.js';

    let { person }: { person: PersonName } = $props();
</script>

<article>
    <p>{person.firstName} {person.lastName}</p>
</article>
