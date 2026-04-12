/** @derive(Default, Serialize, Deserialize) */
/** @serde({ tag: "type", content: "value" }) */
export type PropValue =
    | /** @default */ { type: 'String'; value: string }
    | { type: 'Number'; value: number }
    | { type: 'Boolean'; value: boolean }
    | { type: 'Json'; value: string }
    | { type: 'Asset'; value: string }
    | { type: 'Page'; value: string }
    | { type: 'Expression'; value: string };
