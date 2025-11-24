import ts from 'typescript';

const DEFAULT_MACRO_NAMES = ['Derive'];
const DEFAULT_MIXIN_TYPES = ['MacroDebug', 'MacroJSON'];
const FILE_EXTENSIONS = ['.ts', '.tsx', '.svelte', '.svelte.ts', '.svelte.tsx'];
const AUGMENTATION_BANNER = '\n// @ts-macros/derive augmentations\n';

export interface TsMacrosAugmentationConfig {
    macroNames: Set<string>;
    mixinModule: string;
    mixinTypes: string[];
}

export interface TsMacrosAugmentationSettings {
    macroNames?: string[];
    mixinModule?: string;
    mixinTypes?: string[];
}

export function createTsMacrosAugmentationConfig(
    settings?: TsMacrosAugmentationSettings
): TsMacrosAugmentationConfig {
    return {
        macroNames: new Set(settings?.macroNames ?? DEFAULT_MACRO_NAMES),
        mixinModule: settings?.mixinModule ?? '$lib/macros',
        mixinTypes: settings?.mixinTypes ?? DEFAULT_MIXIN_TYPES
    };
}

export function augmentWithTsMacros(
    tsModule: typeof ts,
    fileName: string,
    sourceText: string,
    config?: TsMacrosAugmentationConfig
): string | null {
    if (!config || !shouldProcess(fileName) || !sourceText.includes('@')) {
        return null;
    }

    const source = tsModule.createSourceFile(
        fileName,
        sourceText,
        tsModule.ScriptTarget.Latest,
        true,
        ts.ScriptKind.TSX
    );

    const decorated = collectDecoratedClasses(tsModule, source, config.macroNames);
    if (decorated.length === 0) {
        return null;
    }

    const mixinRefs = config.mixinTypes.map(
        (type) => `import("${config.mixinModule}").${type}`
    );
    const additions = decorated
        .filter((meta) => !hasInterfaceDeclaration(sourceText, meta.name))
        .map((meta) => buildInterfaceBlock(meta.name, mixinRefs, meta.isExported));

    if (!additions.length) {
        return null;
    }

    return `${sourceText}${AUGMENTATION_BANNER}${additions.join('')}`;
}

function shouldProcess(fileName: string) {
    return FILE_EXTENSIONS.some((ext) => fileName.endsWith(ext));
}

interface DecoratedClassMeta {
    name: string;
    isExported: boolean;
}

function collectDecoratedClasses(
    tsModule: typeof ts,
    source: ts.SourceFile,
    macroNames: Set<string>
) {
    const classes: DecoratedClassMeta[] = [];

    const visit = (node: ts.Node) => {
        if (
            tsModule.isClassDeclaration(node) &&
            node.name &&
            hasDecorator(tsModule, node, macroNames)
        ) {
            classes.push({
                name: node.name.text,
                isExported: isNodeExported(tsModule, node)
            });
        }

        tsModule.forEachChild(node, visit);
    };

    visit(source);
    return classes;
}

function hasDecorator(
    tsModule: typeof ts,
    node: ts.ClassDeclaration,
    macroNames: Set<string>
) {
    if (!tsModule.canHaveDecorators(node)) {
        return false;
    }

    const decorators = tsModule.getDecorators(node);
    if (!decorators?.length) {
        return false;
    }

    return decorators.some((decorator) => {
        const expr = decorator.expression;
        if (tsModule.isIdentifier(expr)) {
            return macroNames.has(expr.text);
        }

        if (
            tsModule.isCallExpression(expr) &&
            tsModule.isIdentifier(expr.expression)
        ) {
            return macroNames.has(expr.expression.text);
        }

        return false;
    });
}

function isNodeExported(tsModule: typeof ts, node: ts.ClassDeclaration) {
    const flags = tsModule.getCombinedModifierFlags(node);
    return (flags & tsModule.ModifierFlags.Export) !== 0;
}

function hasInterfaceDeclaration(sourceText: string, name: string) {
    const pattern = new RegExp(`interface\\s+${name}\\b`);
    return pattern.test(sourceText);
}

function buildInterfaceBlock(
    name: string,
    mixinRefs: string[],
    isExported: boolean
) {
    if (!mixinRefs.length) {
        return '';
    }

    const aliasName = `__TsMacros${name}Mixin`;
    const intersection =
        mixinRefs.length === 1 ? mixinRefs[0] : mixinRefs.join(' & ');
    const exportPrefix = isExported ? 'export ' : '';

    return `${exportPrefix}type ${aliasName} = ${intersection};\n${exportPrefix}interface ${name} extends ${aliasName} {}\n`;
}
