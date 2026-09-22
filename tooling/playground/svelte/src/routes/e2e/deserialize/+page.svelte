<script lang="ts">
import {
    type Organization,
    organizationDeserialize,
    type Employee,
    type Project,
    type Milestone,
    type Skill,
    type Tag,
    type Feature,
    type Limit,
    type MetadataValue
} from '$lib/e2e/types.svelte';

let result = $state<ReturnType<typeof organizationDeserialize> | null>(null);
let org: Organization | null = $derived(result?.success ? result.value : null);

// Verification checks — each one probes a different deserialization path
let checks: Array<{ label: string; pass: boolean; detail: string }> = $derived.by(() => {
    if (!org) return [];
    const results: Array<{ label: string; pass: boolean; detail: string }> = [];

    // 1. Top-level Date
    results.push({
        label: 'org.foundedAt is Date',
        pass: org.foundedAt instanceof Date,
        detail: String(org.foundedAt)
    });

    // 2. Array<Tag> — recursive deser of array elements
    results.push({
        label: 'tags deserialized (length)',
        pass: org.tags.length === 3,
        detail: `${org.tags.length}`
    });
    results.push({
        label: 'tags[0].createdAt is Date',
        pass: org.tags[0]?.createdAt instanceof Date,
        detail: String(org.tags[0]?.createdAt)
    });

    // 3. Set<Feature> — recursive deser of set elements
    const features = org.config.features;
    results.push({
        label: 'config.features is Set',
        pass: features instanceof Set,
        detail: `${features.constructor.name}`
    });
    results.push({
        label: 'config.features.size === 3',
        pass: features.size === 3,
        detail: `${features.size}`
    });
    const firstFeature = [...features][0];
    results.push({
        label: 'feature.enabledAt is Date',
        pass: firstFeature?.enabledAt instanceof Date,
        detail: String(firstFeature?.enabledAt)
    });

    // 4. Map<string, Limit> — recursive deser of map values
    const limits = org.config.limits;
    results.push({
        label: 'config.limits is Map',
        pass: limits instanceof Map,
        detail: `${limits.constructor.name}`
    });
    results.push({
        label: 'config.limits.size === 3',
        pass: limits.size === 3,
        detail: `${limits.size}`
    });
    const maxUsersLimit = limits.get('maxUsers');
    results.push({
        label: 'limits.maxUsers.resetAt is Date',
        pass: maxUsersLimit?.resetAt instanceof Date,
        detail: String(maxUsersLimit?.resetAt)
    });
    results.push({
        label: 'limits.maxUsers.max === 500',
        pass: maxUsersLimit?.max === 500,
        detail: `${maxUsersLimit?.max}`
    });

    // 5. Department[].createdAt — nested array > Date
    const eng = org.departments[0];
    results.push({
        label: 'dept[0].createdAt is Date',
        pass: eng?.createdAt instanceof Date,
        detail: String(eng?.createdAt)
    });

    // 6. Budget (nested Serializable single field)
    results.push({
        label: 'dept.budget.approvedAt is Date',
        pass: eng?.budget.approvedAt instanceof Date,
        detail: String(eng?.budget.approvedAt)
    });

    // 7. Employee (lead) — nested Serializable
    results.push({
        label: 'dept.lead.hireDate is Date',
        pass: eng?.lead.hireDate instanceof Date,
        detail: String(eng?.lead.hireDate)
    });
    results.push({
        label: 'dept.lead.terminatedAt is null',
        pass: eng?.lead.terminatedAt === null,
        detail: String(eng?.lead.terminatedAt)
    });

    // 8. Address (nested inside Employee)
    results.push({
        label: 'dept.lead.address.city === "San Francisco"',
        pass: eng?.lead.address.city === 'San Francisco',
        detail: eng?.lead.address.city
    });

    // 9. Set<Skill> on lead — Set elements with Date
    const leadSkills = eng?.lead.skills;
    results.push({
        label: 'lead.skills is Set',
        pass: leadSkills instanceof Set,
        detail: `${leadSkills?.constructor.name}`
    });
    const firstSkill = [...(leadSkills ?? [])][0];
    results.push({
        label: 'lead.skill.certifiedAt is Date',
        pass: firstSkill?.certifiedAt instanceof Date,
        detail: String(firstSkill?.certifiedAt)
    });

    // 10. Project[] > Milestone[] — doubly nested arrays
    const macroEngineProject = eng?.lead.projects[0];
    results.push({
        label: 'project.startedAt is Date',
        pass: macroEngineProject?.startedAt instanceof Date,
        detail: String(macroEngineProject?.startedAt)
    });
    results.push({
        label: 'project has 3 milestones',
        pass: macroEngineProject?.milestones.length === 3,
        detail: `${macroEngineProject?.milestones.length}`
    });
    const completedMilestone = macroEngineProject?.milestones[0];
    results.push({
        label: 'milestone.dueDate is Date',
        pass: completedMilestone?.dueDate instanceof Date,
        detail: String(completedMilestone?.dueDate)
    });
    results.push({
        label: 'milestone.completedAt is Date (non-null)',
        pass: completedMilestone?.completedAt instanceof Date,
        detail: String(completedMilestone?.completedAt)
    });
    const pendingMilestone = macroEngineProject?.milestones[2];
    results.push({
        label: 'milestone.completedAt is null (pending)',
        pass: pendingMilestone?.completedAt === null,
        detail: String(pendingMilestone?.completedAt)
    });

    // 11. Map<string, MetadataValue> on Project
    const projectMeta = macroEngineProject?.metadata;
    results.push({
        label: 'project.metadata is Map',
        pass: projectMeta instanceof Map,
        detail: `${projectMeta?.constructor.name}`
    });
    const priorityMeta = projectMeta?.get('priority');
    results.push({
        label: 'metadata.priority.updatedAt is Date',
        pass: priorityMeta?.updatedAt instanceof Date,
        detail: String(priorityMeta?.updatedAt)
    });

    // 12. employees[] array — dept employee list
    results.push({
        label: 'dept.employees.length === 2',
        pass: eng?.employees.length === 2,
        detail: `${eng?.employees.length}`
    });
    const bob = eng?.employees[0];
    results.push({
        label: 'employee.hireDate is Date',
        pass: bob?.hireDate instanceof Date,
        detail: String(bob?.hireDate)
    });

    // 13. Terminated employee — nullable Date is non-null
    const carol = eng?.employees[1];
    results.push({
        label: 'carol?.terminatedAt is Date (non-null)',
        pass: carol?.terminatedAt instanceof Date,
        detail: String(carol?.terminatedAt)
    });

    // 14. Second department — empty employees array
    const design = org.departments[1];
    results.push({
        label: 'design dept has 0 employees',
        pass: design?.employees.length === 0,
        detail: `${design?.employees.length}`
    });
    results.push({
        label: 'design?.lead.hireDate is Date',
        pass: design?.lead.hireDate instanceof Date,
        detail: String(design?.lead.hireDate)
    });

    return results;
});

async function runTest() {
    const res = await fetch('/api/organization');
    const raw = await res.json();
    result = organizationDeserialize(raw);
}
</script>

<main data-testid="deser-e2e">
    <h1>Deserialization E2E</h1>

    <button onclick={runTest} data-testid="run-test">Fetch & Deserialize</button>

    {#if result}
        <div data-testid="result-status" data-success={result.success}>
            {result.success ? 'SUCCESS' : 'FAILURE'}
        </div>

        {#if !result.success}
            <pre data-testid="errors">{JSON.stringify(result.errors, null, 2)}</pre>
        {/if}

        {#if checks.length > 0}
            <table data-testid="checks">
                <thead><tr><th>Check</th><th>Pass</th><th>Detail</th></tr></thead>
                <tbody>
                    {#each checks as check}
                        <tr data-check={check.label} data-pass={check.pass}>
                            <td>{check.label}</td>
                            <td>{check.pass ? 'PASS' : 'FAIL'}</td>
                            <td><code>{check.detail}</code></td>
                        </tr>
                    {/each}
                </tbody>
            </table>

            <div data-testid="summary">
                {checks.filter(c => c.pass).length}/{checks.length} passed
            </div>
        {/if}
    {/if}
</main>
