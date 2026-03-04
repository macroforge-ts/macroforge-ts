/**
 * Raw JSON fixture data for the Organization type graph.
 * All dates are ISO strings (as they'd arrive from a real API).
 * Maps are plain objects. Sets are arrays. This is the wire format.
 */
export const organizationFixture = {
    id: 'org_001',
    name: 'Macroforge Industries',
    foundedAt: '2020-03-15T09:00:00.000Z',
    tags: [
        { label: 'tech', color: '#3b82f6', createdAt: '2023-01-10T08:00:00.000Z' },
        {
            label: 'open-source',
            color: '#10b981',
            createdAt: '2023-06-20T14:30:00.000Z'
        },
        { label: 'rust', color: '#f97316', createdAt: '2024-02-01T11:00:00.000Z' }
    ],
    config: {
        features: [
            { name: 'sso', enabledAt: '2023-03-01T00:00:00.000Z' },
            { name: 'audit-log', enabledAt: '2023-09-15T12:00:00.000Z' },
            { name: 'custom-macros', enabledAt: '2024-01-20T08:30:00.000Z' }
        ],
        limits: {
            maxUsers: { max: 500, resetAt: '2025-01-01T00:00:00.000Z' },
            maxProjects: { max: 100, resetAt: '2025-06-01T00:00:00.000Z' },
            maxStorage: { max: 1024, resetAt: '2025-01-01T00:00:00.000Z' }
        }
    },
    departments: [
        {
            id: 'dept_eng',
            name: 'Engineering',
            createdAt: '2020-04-01T10:00:00.000Z',
            budget: {
                amount: 2500000,
                currency: 'USD',
                approvedAt: '2024-12-01T09:00:00.000Z'
            },
            lead: {
                id: 'emp_001',
                name: 'Alice Chen',
                email: 'alice@macroforge.dev',
                hireDate: '2020-03-15T09:00:00.000Z',
                terminatedAt: null,
                address: {
                    street: '123 Compiler Lane',
                    city: 'San Francisco',
                    state: 'CA',
                    zip: '94102',
                    country: 'US'
                },
                skills: [
                    { name: 'Rust', level: 10, certifiedAt: '2019-06-01T00:00:00.000Z' },
                    {
                        name: 'TypeScript',
                        level: 9,
                        certifiedAt: '2020-01-15T00:00:00.000Z'
                    },
                    {
                        name: 'Systems Design',
                        level: 8,
                        certifiedAt: '2021-11-01T00:00:00.000Z'
                    }
                ],
                projects: [
                    {
                        id: 'proj_macro_engine',
                        name: 'Macro Engine v2',
                        startedAt: '2024-01-10T08:00:00.000Z',
                        milestones: [
                            {
                                title: 'Parser rewrite',
                                dueDate: '2024-03-01T00:00:00.000Z',
                                completedAt: '2024-02-28T17:30:00.000Z'
                            },
                            {
                                title: 'Codegen pipeline',
                                dueDate: '2024-06-01T00:00:00.000Z',
                                completedAt: '2024-05-15T14:00:00.000Z'
                            },
                            {
                                title: 'Incremental expansion',
                                dueDate: '2024-09-01T00:00:00.000Z',
                                completedAt: null
                            }
                        ],
                        metadata: {
                            priority: {
                                value: 'critical',
                                updatedAt: '2024-01-10T08:00:00.000Z'
                            },
                            status: {
                                value: 'in-progress',
                                updatedAt: '2024-11-20T16:00:00.000Z'
                            }
                        }
                    }
                ]
            },
            employees: [
                {
                    id: 'emp_002',
                    name: 'Bob Martinez',
                    email: 'bob@macroforge.dev',
                    hireDate: '2021-06-01T09:00:00.000Z',
                    terminatedAt: null,
                    address: {
                        street: '456 AST Avenue',
                        city: 'Austin',
                        state: 'TX',
                        zip: '73301',
                        country: 'US'
                    },
                    skills: [
                        {
                            name: 'TypeScript',
                            level: 8,
                            certifiedAt: '2021-03-01T00:00:00.000Z'
                        },
                        {
                            name: 'Svelte',
                            level: 7,
                            certifiedAt: '2022-08-15T00:00:00.000Z'
                        }
                    ],
                    projects: [
                        {
                            id: 'proj_vite_plugin',
                            name: 'Vite Plugin',
                            startedAt: '2023-09-01T08:00:00.000Z',
                            milestones: [
                                {
                                    title: 'HMR support',
                                    dueDate: '2023-12-01T00:00:00.000Z',
                                    completedAt: '2023-11-28T11:00:00.000Z'
                                },
                                {
                                    title: 'Sourcemap integration',
                                    dueDate: '2024-02-01T00:00:00.000Z',
                                    completedAt: '2024-01-30T09:45:00.000Z'
                                }
                            ],
                            metadata: {
                                priority: {
                                    value: 'high',
                                    updatedAt: '2023-09-01T08:00:00.000Z'
                                },
                                ecosystem: {
                                    value: 'vite',
                                    updatedAt: '2023-09-01T08:00:00.000Z'
                                }
                            }
                        }
                    ]
                },
                {
                    id: 'emp_003',
                    name: 'Carol Nakamura',
                    email: 'carol@macroforge.dev',
                    hireDate: '2022-01-15T09:00:00.000Z',
                    terminatedAt: '2024-08-30T17:00:00.000Z',
                    address: {
                        street: '789 Token Street',
                        city: 'Seattle',
                        state: 'WA',
                        zip: '98101',
                        country: 'US'
                    },
                    skills: [
                        { name: 'Rust', level: 6, certifiedAt: '2022-05-01T00:00:00.000Z' }
                    ],
                    projects: []
                }
            ]
        },
        {
            id: 'dept_design',
            name: 'Design',
            createdAt: '2021-01-15T10:00:00.000Z',
            budget: {
                amount: 800000,
                currency: 'USD',
                approvedAt: '2024-12-01T09:00:00.000Z'
            },
            lead: {
                id: 'emp_004',
                name: 'Diana Okafor',
                email: 'diana@macroforge.dev',
                hireDate: '2021-01-15T09:00:00.000Z',
                terminatedAt: null,
                address: {
                    street: '321 Pixel Road',
                    city: 'Portland',
                    state: 'OR',
                    zip: '97201',
                    country: 'US'
                },
                skills: [
                    {
                        name: 'UI Design',
                        level: 9,
                        certifiedAt: '2019-04-01T00:00:00.000Z'
                    },
                    { name: 'Figma', level: 10, certifiedAt: '2020-07-01T00:00:00.000Z' }
                ],
                projects: [
                    {
                        id: 'proj_design_system',
                        name: 'Design System v3',
                        startedAt: '2024-03-01T08:00:00.000Z',
                        milestones: [
                            {
                                title: 'Token system',
                                dueDate: '2024-05-01T00:00:00.000Z',
                                completedAt: '2024-04-28T15:00:00.000Z'
                            },
                            {
                                title: 'Component library',
                                dueDate: '2024-09-01T00:00:00.000Z',
                                completedAt: null
                            }
                        ],
                        metadata: {
                            priority: {
                                value: 'medium',
                                updatedAt: '2024-03-01T08:00:00.000Z'
                            }
                        }
                    }
                ]
            },
            employees: []
        }
    ]
};
