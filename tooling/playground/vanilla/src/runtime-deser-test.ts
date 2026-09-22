/** @derive(Deserialize) */
export interface Inner {
    name: string;
    createdAt: Date;
    score: number;
}

/** @derive(Deserialize) */
export interface Container {
    items: Inner[];
}

export function testNormal() {
    return Container.deserialize({
        items: [
            { name: 'Alice', createdAt: '2024-01-15T10:30:00Z', score: 95 }
        ]
    });
}

export function testMissingFields() {
    return Container.deserialize({
        items: [
            { name: 'Bob' } // missing createdAt and score
        ]
    });
}

export function testNullElement() {
    return Container.deserialize({
        items: [null]
    });
}

export function testMixedElements() {
    return Container.deserialize({
        items: [
            { name: 'Alice', createdAt: '2024-01-15T10:30:00Z', score: 95 },
            { name: 'Bob' } // missing fields
        ]
    });
}

type ContainerFailure = Extract<ReturnType<typeof Container.deserialize>, { success: false }>;

export type RecursiveDeserResult =
    | { success: false; errors: ContainerFailure['errors'] }
    | {
        success: true;
        itemCount: number;
        firstIsDate: boolean;
        firstDateISO: string | undefined;
        secondIsDate: boolean;
        secondDateISO: string | undefined;
    };

/** The item's ISO date when deserialization produced a real `Date`. */
function isoDate(item: Inner | undefined): string | undefined {
    return item?.createdAt instanceof Date ? item.createdAt.toISOString() : undefined;
}

export function testRecursiveActual(): RecursiveDeserResult {
    const result = Container.deserialize({
        items: [
            { name: 'Alice', createdAt: '2024-01-15T10:30:00Z', score: 95 },
            { name: 'Bob', createdAt: '2024-06-20T14:00:00Z', score: 87 }
        ]
    });
    if (!result.success) return { success: false, errors: result.errors };
    const [first, second] = result.value.items;
    return {
        success: true,
        itemCount: result.value.items.length,
        firstIsDate: first?.createdAt instanceof Date,
        firstDateISO: isoDate(first),
        secondIsDate: second?.createdAt instanceof Date,
        secondDateISO: isoDate(second)
    };
}
