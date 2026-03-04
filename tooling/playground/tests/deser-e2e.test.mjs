import { describe, test } from 'node:test';
import assert from 'node:assert/strict';
import { svelteRoot, withViteServer } from './test-utils.mjs';

describe('Deserialization E2E — complex nested type graph via SvelteKit', () => {
    test('deserializes a hyper-complex Organization from raw JSON through the full macro pipeline', async () => {
        await withViteServer(svelteRoot, async (server) => {
            // Load the raw fixture (plain JSON — dates are ISO strings, sets are arrays, maps are objects)
            const fixtureMod = await server.ssrLoadModule('/src/lib/e2e/fixture.ts');
            const raw = fixtureMod.organizationFixture;

            // Sanity: raw data has ISO strings, not Dates
            assert.equal(typeof raw.foundedAt, 'string');
            assert.equal(typeof raw.tags[0].createdAt, 'string');
            assert.ok(
                Array.isArray(raw.config.features),
                'raw features is array, not Set'
            );
            assert.ok(!raw.config.limits.has, 'raw limits is plain object, not Map');

            // Load the macro-expanded types module
            const typesMod = await server.ssrLoadModule(
                '/src/lib/e2e/types.svelte.ts'
            );
            const { organizationDeserialize } = typesMod;
            assert.equal(
                typeof organizationDeserialize,
                'function',
                'organizationDeserialize should be exported'
            );

            // ============================================================
            //  DESERIALIZE
            // ============================================================
            const result = organizationDeserialize(raw);
            assert.ok(
                result.success,
                `Deserialization failed: ${JSON.stringify(result.errors ?? [])}`
            );
            const org = result.value;

            // ============================================================
            //  TOP-LEVEL FIELDS
            // ============================================================
            assert.equal(org.id, 'org_001');
            assert.equal(org.name, 'Macroforge Industries');
            assert.ok(org.foundedAt instanceof Date, 'org.foundedAt should be Date');
            assert.equal(org.foundedAt.toISOString(), '2020-03-15T09:00:00.000Z');

            // ============================================================
            //  Array<Tag> — recursive deser of array elements with Date
            // ============================================================
            assert.equal(org.tags.length, 3);
            for (const tag of org.tags) {
                assert.ok(
                    tag.createdAt instanceof Date,
                    `tag "${tag.label}".createdAt should be Date`
                );
                assert.equal(typeof tag.label, 'string');
                assert.equal(typeof tag.color, 'string');
            }
            assert.equal(org.tags[0].label, 'tech');
            assert.equal(
                org.tags[0].createdAt.toISOString(),
                '2023-01-10T08:00:00.000Z'
            );

            // ============================================================
            //  OrgConfig.features: Set<Feature> with Date
            // ============================================================
            const features = org.config.features;
            assert.ok(features instanceof Set, 'features should be Set');
            assert.equal(features.size, 3);
            for (const f of features) {
                assert.ok(
                    f.enabledAt instanceof Date,
                    `feature "${f.name}".enabledAt should be Date`
                );
                assert.equal(typeof f.name, 'string');
            }

            // ============================================================
            //  OrgConfig.limits: Map<string, Limit> with Date
            // ============================================================
            const limits = org.config.limits;
            assert.ok(limits instanceof Map, 'limits should be Map');
            assert.equal(limits.size, 3);
            for (const [key, limit] of limits) {
                assert.equal(typeof key, 'string');
                assert.ok(
                    limit.resetAt instanceof Date,
                    `limit "${key}".resetAt should be Date`
                );
                assert.equal(typeof limit.max, 'number');
            }
            assert.equal(limits.get('maxUsers').max, 500);
            assert.equal(limits.get('maxProjects').max, 100);
            assert.equal(limits.get('maxStorage').max, 1024);
            assert.equal(
                limits.get('maxUsers').resetAt.toISOString(),
                '2025-01-01T00:00:00.000Z'
            );

            // ============================================================
            //  Department[0]: Engineering
            // ============================================================
            assert.equal(org.departments.length, 2);
            const eng = org.departments[0];
            assert.equal(eng.id, 'dept_eng');
            assert.equal(eng.name, 'Engineering');
            assert.ok(eng.createdAt instanceof Date, 'dept.createdAt should be Date');

            // Budget — nested Serializable with Date
            assert.ok(
                eng.budget.approvedAt instanceof Date,
                'budget.approvedAt should be Date'
            );
            assert.equal(eng.budget.amount, 2500000);
            assert.equal(eng.budget.currency, 'USD');

            // ============================================================
            //  Lead (Employee) — deeply nested Serializable
            // ============================================================
            const lead = eng.lead;
            assert.equal(lead.id, 'emp_001');
            assert.equal(lead.name, 'Alice Chen');
            assert.ok(lead.hireDate instanceof Date, 'lead.hireDate should be Date');
            assert.equal(lead.hireDate.toISOString(), '2020-03-15T09:00:00.000Z');
            assert.equal(lead.terminatedAt, null, 'lead.terminatedAt should be null');

            // Address — nested Serializable with only primitives
            assert.equal(lead.address.street, '123 Compiler Lane');
            assert.equal(lead.address.city, 'San Francisco');
            assert.equal(lead.address.state, 'CA');
            assert.equal(lead.address.zip, '94102');
            assert.equal(lead.address.country, 'US');

            // ============================================================
            //  Set<Skill> on lead — Set elements with Date
            // ============================================================
            const skills = lead.skills;
            assert.ok(skills instanceof Set, 'skills should be Set');
            assert.equal(skills.size, 3);
            for (const skill of skills) {
                assert.ok(
                    skill.certifiedAt instanceof Date,
                    `skill "${skill.name}".certifiedAt should be Date`
                );
                assert.equal(typeof skill.level, 'number');
                assert.equal(typeof skill.name, 'string');
            }

            // ============================================================
            //  Project[] > Milestone[] — doubly nested arrays with Date | null
            // ============================================================
            assert.equal(lead.projects.length, 1);
            const project = lead.projects[0];
            assert.equal(project.id, 'proj_macro_engine');
            assert.equal(project.name, 'Macro Engine v2');
            assert.ok(
                project.startedAt instanceof Date,
                'project.startedAt should be Date'
            );
            assert.equal(project.milestones.length, 3);

            // Milestone 0: completed — Date | null where non-null
            const m0 = project.milestones[0];
            assert.equal(m0.title, 'Parser rewrite');
            assert.ok(m0.dueDate instanceof Date, 'milestone.dueDate should be Date');
            assert.ok(
                m0.completedAt instanceof Date,
                'completed milestone.completedAt should be Date'
            );
            assert.equal(m0.completedAt.toISOString(), '2024-02-28T17:30:00.000Z');

            // Milestone 1: completed
            const m1 = project.milestones[1];
            assert.ok(m1.completedAt instanceof Date);
            assert.equal(m1.completedAt.toISOString(), '2024-05-15T14:00:00.000Z');

            // Milestone 2: pending — Date | null where null
            const m2 = project.milestones[2];
            assert.equal(m2.title, 'Incremental expansion');
            assert.ok(m2.dueDate instanceof Date);
            assert.equal(
                m2.completedAt,
                null,
                'pending milestone.completedAt should be null'
            );

            // ============================================================
            //  Map<string, MetadataValue> on Project — Map values with Date
            // ============================================================
            const meta = project.metadata;
            assert.ok(meta instanceof Map, 'project.metadata should be Map');
            assert.equal(meta.size, 2);
            const priority = meta.get('priority');
            assert.equal(priority.value, 'critical');
            assert.ok(
                priority.updatedAt instanceof Date,
                'metadata.updatedAt should be Date'
            );
            assert.equal(
                priority.updatedAt.toISOString(),
                '2024-01-10T08:00:00.000Z'
            );
            const status = meta.get('status');
            assert.equal(status.value, 'in-progress');
            assert.ok(status.updatedAt instanceof Date);

            // ============================================================
            //  employees[] — department employee list
            // ============================================================
            assert.equal(eng.employees.length, 2);

            // Bob — active employee with projects
            const bob = eng.employees[0];
            assert.equal(bob.id, 'emp_002');
            assert.equal(bob.name, 'Bob Martinez');
            assert.ok(bob.hireDate instanceof Date);
            assert.equal(bob.terminatedAt, null);
            assert.ok(bob.skills instanceof Set);
            assert.equal(bob.skills.size, 2);
            for (const s of bob.skills) {
                assert.ok(s.certifiedAt instanceof Date);
            }

            // Bob's project
            assert.equal(bob.projects.length, 1);
            const vitePlugin = bob.projects[0];
            assert.equal(vitePlugin.name, 'Vite Plugin');
            assert.ok(vitePlugin.startedAt instanceof Date);
            assert.equal(vitePlugin.milestones.length, 2);
            for (const ms of vitePlugin.milestones) {
                assert.ok(ms.dueDate instanceof Date);
                assert.ok(
                    ms.completedAt instanceof Date,
                    'both vite plugin milestones are completed'
                );
            }
            assert.ok(vitePlugin.metadata instanceof Map);
            assert.equal(vitePlugin.metadata.size, 2);
            for (const [, v] of vitePlugin.metadata) {
                assert.ok(v.updatedAt instanceof Date);
            }

            // Carol — terminated employee (nullable Date is non-null)
            const carol = eng.employees[1];
            assert.equal(carol.id, 'emp_003');
            assert.equal(carol.name, 'Carol Nakamura');
            assert.ok(
                carol.terminatedAt instanceof Date,
                'carol.terminatedAt should be Date'
            );
            assert.equal(
                carol.terminatedAt.toISOString(),
                '2024-08-30T17:00:00.000Z'
            );
            assert.equal(carol.projects.length, 0, 'carol has no projects');
            assert.ok(carol.skills instanceof Set);
            assert.equal(carol.skills.size, 1);

            // ============================================================
            //  Department[1]: Design — empty employees, lead with projects
            // ============================================================
            const design = org.departments[1];
            assert.equal(design.id, 'dept_design');
            assert.equal(design.name, 'Design');
            assert.ok(design.createdAt instanceof Date);
            assert.equal(
                design.employees.length,
                0,
                'design dept has empty employees array'
            );

            // Design lead
            const designLead = design.lead;
            assert.equal(designLead.name, 'Diana Okafor');
            assert.ok(designLead.hireDate instanceof Date);
            assert.equal(designLead.terminatedAt, null);
            assert.ok(designLead.skills instanceof Set);
            assert.equal(designLead.skills.size, 2);

            // Design lead project
            assert.equal(designLead.projects.length, 1);
            const dsProject = designLead.projects[0];
            assert.equal(dsProject.name, 'Design System v3');
            assert.ok(dsProject.startedAt instanceof Date);
            assert.equal(dsProject.milestones.length, 2);
            assert.ok(
                dsProject.milestones[0].completedAt instanceof Date,
                'token system milestone completed'
            );
            assert.equal(
                dsProject.milestones[1].completedAt,
                null,
                'component library milestone pending'
            );
            assert.ok(dsProject.metadata instanceof Map);
            assert.equal(dsProject.metadata.size, 1);
            const dsPriority = dsProject.metadata.get('priority');
            assert.equal(dsPriority.value, 'medium');
            assert.ok(dsPriority.updatedAt instanceof Date);
        });
    });
});
