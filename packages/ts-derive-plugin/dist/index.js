"use strict";
const DEFAULT_MACROS = ['Derive'];
const DEFAULT_MIXIN_TYPES = ['MacroDebug', 'MacroJSON'];
const FILE_EXTENSIONS = ['.ts', '.tsx'];
const AUGMENTATION_BANNER = '\n// @ts-macros/derive augmentations\n';
function shouldProcess(fileName) {
    return FILE_EXTENSIONS.some((ext) => fileName.endsWith(ext));
}
function getDecorators(tsModule, node) {
    if (!tsModule.canHaveDecorators(node)) {
        return undefined;
    }
    const decorators = tsModule.getDecorators(node);
    if (decorators && decorators.length) {
        return decorators;
    }
    return undefined;
}
function hasDecorator(tsModule, node, macroNames) {
    const decorators = getDecorators(tsModule, node);
    if (!(decorators === null || decorators === void 0 ? void 0 : decorators.length)) {
        return false;
    }
    return decorators.some((decorator) => {
        const expr = decorator.expression;
        if (tsModule.isIdentifier(expr)) {
            return macroNames.has(expr.text);
        }
        if (tsModule.isCallExpression(expr) && tsModule.isIdentifier(expr.expression)) {
            return macroNames.has(expr.expression.text);
        }
        return false;
    });
}
function collectDecoratedClasses(tsModule, source, macroNames) {
    const classNames = new Set();
    const visit = (node) => {
        if (tsModule.isClassDeclaration(node) && node.name && hasDecorator(tsModule, node, macroNames)) {
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
    const extendsClause = mixinRefs.length ? ` extends ${mixinRefs.join(', ')}` : '';
    return `interface ${name}${extendsClause} {}\n`;
}
function augmentSource(tsModule, fileName, sourceText, macroNames, mixinModule, mixinTypes) {
    if (!sourceText.includes('@')) {
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
    return `${sourceText}${AUGMENTATION_BANNER}${additions.join('')}`;
}
function init(modules) {
    function create(info) {
        var _a, _b, _c, _d, _e;
        const tsModule = modules.typescript;
        const config = (_a = info.config) !== null && _a !== void 0 ? _a : {};
        const macroNames = new Set((_b = config.macroNames) !== null && _b !== void 0 ? _b : DEFAULT_MACROS);
        const mixinTypes = (_c = config.mixinTypes) !== null && _c !== void 0 ? _c : DEFAULT_MIXIN_TYPES;
        const mixinModule = (_d = config.mixinModule) !== null && _d !== void 0 ? _d : '$lib/macros';
        const originalGetScriptSnapshot = (_e = info.languageServiceHost.getScriptSnapshot) === null || _e === void 0 ? void 0 : _e.bind(info.languageServiceHost);
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
