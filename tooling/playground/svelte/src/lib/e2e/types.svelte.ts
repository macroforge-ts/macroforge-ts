/**
 * Hyper-complex nested type graph for deserialization e2e testing.
 *
 * Shape:
 *   Organization
 *   ├── departments: Department[]
 *   │   ├── lead: Employee
 *   │   ├── employees: Employee[]
 *   │   │   ├── address: Address
 *   │   │   ├── skills: Set<Skill>
 *   │   │   │   └── Skill { name, level, certifiedAt: Date }
 *   │   │   ├── projects: Project[]
 *   │   │   │   ├── milestones: Milestone[]
 *   │   │   │   │   └── Milestone { title, dueDate: Date, completedAt: Date | null }
 *   │   │   │   └── metadata: Map<string, MetadataValue>
 *   │   │   │       └── MetadataValue { value: string, updatedAt: Date }
 *   │   │   └── hireDate: Date
 *   │   ├── budget: Budget
 *   │   │   └── Budget { amount, currency, approvedAt: Date }
 *   │   └── createdAt: Date
 *   ├── tags: Tag[]
 *   │   └── Tag { label, color, createdAt: Date }
 *   ├── config: OrgConfig
 *   │   ├── features: Set<Feature>
 *   │   │   └── Feature { name, enabledAt: Date }
 *   │   └── limits: Map<string, Limit>
 *   │       └── Limit { max, resetAt: Date }
 *   └── foundedAt: Date
 */

// --- Leaf types ---

/** @derive(Serialize, Deserialize) */
export interface Address {
    street: string;
    city: string;
    state: string;
    zip: string;
    country: string;
}

/** @derive(Serialize, Deserialize) */
export interface Skill {
    name: string;
    level: number;
    certifiedAt: Date;
}

/** @derive(Serialize, Deserialize) */
export interface Milestone {
    title: string;
    dueDate: Date;
    completedAt: Date | null;
}

/** @derive(Serialize, Deserialize) */
export interface MetadataValue {
    value: string;
    updatedAt: Date;
}

/** @derive(Serialize, Deserialize) */
export interface Tag {
    label: string;
    color: string;
    createdAt: Date;
}

/** @derive(Serialize, Deserialize) */
export interface Feature {
    name: string;
    enabledAt: Date;
}

/** @derive(Serialize, Deserialize) */
export interface Limit {
    max: number;
    resetAt: Date;
}

/** @derive(Serialize, Deserialize) */
export interface Budget {
    amount: number;
    currency: string;
    approvedAt: Date;
}

// --- Mid-level types ---

/** @derive(Serialize, Deserialize) */
export interface Project {
    id: string;
    name: string;
    milestones: Milestone[];
    metadata: Map<string, MetadataValue>;
    startedAt: Date;
}

/** @derive(Serialize, Deserialize) */
export interface Employee {
    id: string;
    name: string;
    email: string;
    address: Address;
    skills: Set<Skill>;
    projects: Project[];
    hireDate: Date;
    terminatedAt: Date | null;
}

/** @derive(Serialize, Deserialize) */
export interface OrgConfig {
    features: Set<Feature>;
    limits: Map<string, Limit>;
}

// --- Top-level type ---

/** @derive(Serialize, Deserialize) */
export interface Department {
    id: string;
    name: string;
    lead: Employee;
    employees: Employee[];
    budget: Budget;
    createdAt: Date;
}

/** @derive(Serialize, Deserialize) */
export interface Organization {
    id: string;
    name: string;
    departments: Department[];
    tags: Tag[];
    config: OrgConfig;
    foundedAt: Date;
}
