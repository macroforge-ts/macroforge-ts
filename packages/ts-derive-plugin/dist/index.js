"use strict";
const DEFAULT_MACROS = ["Derive"];
const DEFAULT_MIXIN_TYPES = ["MacroDebug", "MacroJSON"];
const FILE_EXTENSIONS = [".ts", ".tsx"];
const AUGMENTATION_BANNER = "\n// @ts-macros/derive augmentations\n";
function shouldProcess(fileName) {
    return FILE_EXTENSIONS.some((ext) => fileName.endsWith(ext));
}
function getDecorators(tsModule, node) {
    if (!tsModule.canHaveDecorators(node)) {
        return undefined;
    }
    const decorators = tsModule.getDecorators(node);
    if (decorators?.length) {
        return decorators;
    }
    return undefined;
}
function hasDecorator(tsModule, node, macroNames) {
    const decorators = getDecorators(tsModule, node);
    if (!decorators?.length) {
        return false;
    }
    return decorators.some((decorator) => {
        const expr = decorator.expression;
        if (tsModule.isIdentifier(expr)) {
            return macroNames.has(expr.text);
        }
        if (tsModule.isCallExpression(expr) &&
            tsModule.isIdentifier(expr.expression)) {
            return macroNames.has(expr.expression.text);
        }
        return false;
    });
}
function collectDecoratedClasses(tsModule, source, macroNames) {
    const classNames = new Set();
    const visit = (node) => {
        if (tsModule.isClassDeclaration(node) &&
            node.name &&
            hasDecorator(tsModule, node, macroNames)) {
            classNames.add(node.name.text);
        }
        tsModule.forEachChild(node, visit);
    };
    visit(source);
    return classNames;
}
function hasInterfaceDeclaration(sourceText, name) {
    const pattern = new RegExp(`interface\\s+${name}\\b`);
    return pattern.test(sourceText);
}
function buildInterfaceBlock(name, mixinRefs) {
    if (!mixinRefs.length) {
        return "";
    }
    const aliasName = `__TsMacros${name}Mixin`;
    const intersection = mixinRefs.length === 1 ? mixinRefs[0] : mixinRefs.join(" & ");
    return `type ${aliasName} = ${intersection};\ninterface ${name} extends ${aliasName} {}\n`;
}
function augmentSource(tsModule, fileName, sourceText, macroNames, mixinModule, mixinTypes) {
    if (!sourceText.includes("@")) {
        return null;
    }
    const source = tsModule.createSourceFile(fileName, sourceText, tsModule.ScriptTarget.Latest, true, tsModule.ScriptKind.TSX);
    const decorated = collectDecoratedClasses(tsModule, source, macroNames);
    if (decorated.size === 0) {
        return null;
    }
    const mixinRefs = mixinTypes.map((type) => `import("${mixinModule}").${type}`);
    const additions = Array.from(decorated)
        .filter((name) => !hasInterfaceDeclaration(sourceText, name))
        .map((name) => buildInterfaceBlock(name, mixinRefs));
    if (!additions.length) {
        return null;
    }
    return `${sourceText}${AUGMENTATION_BANNER}${additions.join("")}`;
}
function init(modules) {
    function create(info) {
        const tsModule = modules.typescript;
        const config = info.config ?? {};
        const macroNames = new Set(config.macroNames ?? DEFAULT_MACROS);
        const mixinTypes = config.mixinTypes ?? DEFAULT_MIXIN_TYPES;
        const mixinModule = config.mixinModule ?? "$lib/macros";
        const originalGetScriptSnapshot = info.languageServiceHost.getScriptSnapshot?.bind(info.languageServiceHost);
        if (originalGetScriptSnapshot) {
            info.languageServiceHost.getScriptSnapshot = (fileName) => {
                if (!shouldProcess(fileName)) {
                    return originalGetScriptSnapshot(fileName);
                }
                const snapshot = originalGetScriptSnapshot(fileName);
                if (!snapshot) {
                    return snapshot;
                }
                const text = snapshot.getText(0, snapshot.getLength());
                const augmented = augmentSource(tsModule, fileName, text, macroNames, mixinModule, mixinTypes);
                if (!augmented) {
                    return snapshot;
                }
                return tsModule.ScriptSnapshot.fromString(augmented);
            };
        }
        return info.languageService;
    }
    return { create };
}
module.exports = init;
