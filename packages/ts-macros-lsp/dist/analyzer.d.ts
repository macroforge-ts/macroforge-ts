export interface DecoratedClassInfo {
    name: string;
    features: string[];
    range: {
        start: number;
        end: number;
    };
}
export interface DocumentAnalysis {
    classes: Map<string, DecoratedClassInfo>;
    identifiers: Map<string, string>;
}
export declare function analyzeDocument(text: string, fileName: string): DocumentAnalysis;
export declare function resolveClassForIdentifier(identifier: string, offset: number, analysis: DocumentAnalysis): DecoratedClassInfo | undefined;
