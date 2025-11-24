"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.analyzeDocument = analyzeDocument;
exports.resolveClassForIdentifier = resolveClassForIdentifier;
const typescript_1 = __importDefault(require("typescript"));
const DEFAULT_MACROS = new Set(["Derive"]);
const DEFAULT_FEATURES = ["Debug", "JSON"];
function analyzeDocument(text, fileName) {
    const source = typescript_1.default.createSourceFile(fileName, text, typescript_1.default.ScriptTarget.Latest, true, detectScriptKind(fileName));
    const classes = collectDecoratedClasses(source);
    const identifiers = collectIdentifierBindings(source, classes);
    return { classes, identifiers };
}
function resolveClassForIdentifier(identifier, offset, analysis) {
    if (identifier === "this") {
        return findClassByOffset(offset, analysis.classes);
    }
    const className = analysis.identifiers.get(identifier);
    if (!className) {
        return undefined;
    }
    return analysis.classes.get(className);
}
function detectScriptKind(fileName) {
    if (fileName.endsWith(".tsx")) {
        return typescript_1.default.ScriptKind.TSX;
    }
    if (fileName.endsWith(".ts")) {
        return typescript_1.default.ScriptKind.TS;
    }
    if (fileName.endsWith(".jsx")) {
        return typescript_1.default.ScriptKind.JSX;
    }
    return typescript_1.default.ScriptKind.TS;
}
function collectDecoratedClasses(source) {
    const classes = new Map();
    const visit = (node) => {
        if (typescript_1.default.isClassDeclaration(node) && node.name) {
            const decorator = findMacroDecorator(node);
            if (decorator) {
                const features = extractFeatures(decorator);
                classes.set(node.name.text, {
                    name: node.name.text,
                    features,
                    range: {
                        start: node.getStart(),
                        end: node.getEnd(),
                    },
                });
            }
        }
        typescript_1.default.forEachChild(node, visit);
    };
    visit(source);
    return classes;
}
function collectIdentifierBindings(source, classes) {
    const identifiers = new Map();
    const visit = (node) => {
        if (typescript_1.default.isVariableDeclaration(node) && typescript_1.default.isIdentifier(node.name)) {
            const targetClass = resolveClassFromType(node.type, classes) ??
                resolveClassFromInitializer(node.initializer, classes);
            if (targetClass) {
                identifiers.set(node.name.text, targetClass);
            }
        }
        if (typescript_1.default.isParameter(node) && typescript_1.default.isIdentifier(node.name)) {
            const targetClass = resolveClassFromType(node.type, classes);
            if (targetClass) {
                identifiers.set(node.name.text, targetClass);
            }
        }
        if (typescript_1.default.isPropertyDeclaration(node) &&
            typescript_1.default.isIdentifier(node.name) &&
            node.type) {
            const targetClass = resolveClassFromType(node.type, classes);
            if (targetClass) {
                identifiers.set(node.name.text, targetClass);
            }
        }
        typescript_1.default.forEachChild(node, visit);
    };
    visit(source);
    return identifiers;
}
function resolveClassFromType(typeNode, classes) {
    if (!typeNode || !typescript_1.default.isTypeReferenceNode(typeNode)) {
        return undefined;
    }
    const typeName = typeNode.typeName;
    if (typescript_1.default.isIdentifier(typeName) && classes.has(typeName.text)) {
        return typeName.text;
    }
    return undefined;
}
function resolveClassFromInitializer(initializer, classes) {
    if (!initializer) {
        return undefined;
    }
    if (typescript_1.default.isNewExpression(initializer) &&
        initializer.expression &&
        typescript_1.default.isIdentifier(initializer.expression)) {
        const className = initializer.expression.text;
        if (classes.has(className)) {
            return className;
        }
    }
    return undefined;
}
function findMacroDecorator(node) {
    const decorators = getDecorators(node);
    if (!decorators?.length) {
        return undefined;
    }
    return decorators.find((decorator) => {
        const expression = decorator.expression;
        if (typescript_1.default.isIdentifier(expression)) {
            return DEFAULT_MACROS.has(expression.text);
        }
        if (typescript_1.default.isCallExpression(expression) &&
            typescript_1.default.isIdentifier(expression.expression)) {
            return DEFAULT_MACROS.has(expression.expression.text);
        }
        return false;
    });
}
function getDecorators(node) {
    if (!typescript_1.default.canHaveDecorators(node)) {
        return undefined;
    }
    const decorators = typescript_1.default.getDecorators(node);
    if (decorators?.length) {
        return decorators;
    }
    return undefined;
}
function extractFeatures(decorator) {
    const expression = decorator.expression;
    if (!typescript_1.default.isCallExpression(expression)) {
        return DEFAULT_FEATURES;
    }
    const features = expression.arguments
        .filter(typescript_1.default.isStringLiteral)
        .map((literal) => literal.text);
    return features.length ? Array.from(new Set(features)) : DEFAULT_FEATURES;
}
function findClassByOffset(offset, classes) {
    for (const info of classes.values()) {
        if (offset >= info.range.start && offset <= info.range.end) {
            return info;
        }
    }
    return undefined;
}
//# sourceMappingURL=analyzer.js.map