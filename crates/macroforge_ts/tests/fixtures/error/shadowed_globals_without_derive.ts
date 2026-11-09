class Map {
    size = 0;
}

interface Record {
    id: string;
}

export function sizeOf(map: Map, record: Record): number {
    return map.size + record.id.length;
}
