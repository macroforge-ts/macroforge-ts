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
      { name: "Alice", createdAt: "2024-01-15T10:30:00Z", score: 95 },
    ],
  });
}

export function testMissingFields() {
  return Container.deserialize({
    items: [
      { name: "Bob" }, // missing createdAt and score
    ],
  });
}

export function testNullElement() {
  return Container.deserialize({
    items: [null],
  });
}

export function testMixedElements() {
  return Container.deserialize({
    items: [
      { name: "Alice", createdAt: "2024-01-15T10:30:00Z", score: 95 },
      { name: "Bob" }, // missing fields
    ],
  });
}

export function testRecursiveActual() {
  const result = Container.deserialize({
    items: [
      { name: "Alice", createdAt: "2024-01-15T10:30:00Z", score: 95 },
      { name: "Bob", createdAt: "2024-06-20T14:00:00Z", score: 87 },
    ],
  });
  if (!result.success) return { success: false, errors: result.errors };
  const c = result.value;
  return {
    success: true,
    itemCount: c.items.length,
    firstIsDate: c.items[0].createdAt instanceof Date,
    firstDateISO: c.items[0].createdAt?.toISOString?.(),
    secondIsDate: c.items[1].createdAt instanceof Date,
    secondDateISO: c.items[1].createdAt?.toISOString?.(),
  };
}
