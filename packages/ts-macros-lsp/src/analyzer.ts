import ts from "typescript";

const DEFAULT_MACROS = new Set(["Derive"]);
const DEFAULT_FEATURES = ["Debug", "JSON"];

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

export function analyzeDocument(
  text: string,
  fileName: string,
): DocumentAnalysis {
  const source = ts.createSourceFile(
    fileName,
    text,
    ts.ScriptTarget.Latest,
    true,
    detectScriptKind(fileName),
  );

  const classes = collectDecoratedClasses(source);
  const identifiers = collectIdentifierBindings(source, classes);

  return { classes, identifiers };
}

export function resolveClassForIdentifier(
  identifier: string,
  offset: number,
  analysis: DocumentAnalysis,
): DecoratedClassInfo | undefined {
  if (identifier === "this") {
    return findClassByOffset(offset, analysis.classes);
  }

  const className = analysis.identifiers.get(identifier);
  if (!className) {
    return undefined;
  }

  return analysis.classes.get(className);
}

function detectScriptKind(fileName: string) {
  if (fileName.endsWith(".tsx")) {
    return ts.ScriptKind.TSX;
  }

  if (fileName.endsWith(".ts")) {
    return ts.ScriptKind.TS;
  }

  if (fileName.endsWith(".jsx")) {
    return ts.ScriptKind.JSX;
  }

  return ts.ScriptKind.TS;
}

function collectDecoratedClasses(source: ts.SourceFile) {
  const classes = new Map<string, DecoratedClassInfo>();

  const visit = (node: ts.Node) => {
    if (ts.isClassDeclaration(node) && node.name) {
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

    ts.forEachChild(node, visit);
  };

  visit(source);
  return classes;
}

function collectIdentifierBindings(
  source: ts.SourceFile,
  classes: Map<string, DecoratedClassInfo>,
) {
  const identifiers = new Map<string, string>();

  const visit = (node: ts.Node) => {
    if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name)) {
      const targetClass =
        resolveClassFromType(node.type, classes) ??
        resolveClassFromInitializer(node.initializer, classes);
      if (targetClass) {
        identifiers.set(node.name.text, targetClass);
      }
    }

    if (ts.isParameter(node) && ts.isIdentifier(node.name)) {
      const targetClass = resolveClassFromType(node.type, classes);
      if (targetClass) {
        identifiers.set(node.name.text, targetClass);
      }
    }

    if (
      ts.isPropertyDeclaration(node) &&
      ts.isIdentifier(node.name) &&
      node.type
    ) {
      const targetClass = resolveClassFromType(node.type, classes);
      if (targetClass) {
        identifiers.set(node.name.text, targetClass);
      }
    }

    ts.forEachChild(node, visit);
  };

  visit(source);
  return identifiers;
}

function resolveClassFromType(
  typeNode: ts.TypeNode | undefined,
  classes: Map<string, DecoratedClassInfo>,
) {
  if (!typeNode || !ts.isTypeReferenceNode(typeNode)) {
    return undefined;
  }

  const typeName = typeNode.typeName;
  if (ts.isIdentifier(typeName) && classes.has(typeName.text)) {
    return typeName.text;
  }

  return undefined;
}

function resolveClassFromInitializer(
  initializer: ts.Expression | undefined,
  classes: Map<string, DecoratedClassInfo>,
) {
  if (!initializer) {
    return undefined;
  }

  if (
    ts.isNewExpression(initializer) &&
    initializer.expression &&
    ts.isIdentifier(initializer.expression)
  ) {
    const className = initializer.expression.text;
    if (classes.has(className)) {
      return className;
    }
  }

  return undefined;
}

function findMacroDecorator(node: ts.ClassDeclaration) {
  const decorators = getDecorators(node);
  if (!decorators?.length) {
    return undefined;
  }

  return decorators.find((decorator) => {
    const expression = decorator.expression;
    if (ts.isIdentifier(expression)) {
      return DEFAULT_MACROS.has(expression.text);
    }

    if (
      ts.isCallExpression(expression) &&
      ts.isIdentifier(expression.expression)
    ) {
      return DEFAULT_MACROS.has(expression.expression.text);
    }

    return false;
  });
}

function getDecorators(node: ts.Node) {
  if (!ts.canHaveDecorators(node)) {
    return undefined;
  }

  const decorators = ts.getDecorators(node);
  if (decorators?.length) {
    return decorators;
  }

  return undefined;
}

function extractFeatures(decorator: ts.Decorator) {
  const expression = decorator.expression;
  if (!ts.isCallExpression(expression)) {
    return DEFAULT_FEATURES;
  }

  const features = expression.arguments
    .filter(ts.isStringLiteral)
    .map((literal) => literal.text);

  return features.length ? Array.from(new Set(features)) : DEFAULT_FEATURES;
}

function findClassByOffset(
  offset: number,
  classes: Map<string, DecoratedClassInfo>,
) {
  for (const info of classes.values()) {
    if (offset >= info.range.start && offset <= info.range.end) {
      return info;
    }
  }

  return undefined;
}
