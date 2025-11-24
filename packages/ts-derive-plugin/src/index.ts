import type ts from "typescript/lib/tsserverlibrary";

interface PluginConfig {
  macroNames?: string[];
  mixinModule?: string;
  mixinTypes?: string[];
}

const DEFAULT_MACROS = ["Derive"];
const DEFAULT_MIXIN_TYPES = ["MacroDebug", "MacroJSON"];

const FILE_EXTENSIONS = [".ts", ".tsx", ".svelte"];

const AUGMENTATION_BANNER = "\n// @ts-macros/derive augmentations\n";

function shouldProcess(fileName: string) {
  return FILE_EXTENSIONS.some((ext) => fileName.endsWith(ext));
}

function getDecorators(tsModule: typeof ts, node: ts.Node) {
  if (!tsModule.canHaveDecorators(node)) {
    return undefined;
  }

  const decorators = tsModule.getDecorators(node);
  if (decorators?.length) {
    return decorators;
  }

  return undefined;
}

function hasDecorator(
  tsModule: typeof ts,
  node: ts.ClassDeclaration,
  macroNames: Set<string>,
) {
  const decorators = getDecorators(tsModule, node);
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

interface DecoratedClassMeta {
  name: string;
  isExported: boolean;
}

function collectDecoratedClasses(
  tsModule: typeof ts,
  source: ts.SourceFile,
  macroNames: Set<string>,
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
        isExported: isNodeExported(tsModule, node),
      });
    }

    tsModule.forEachChild(node, visit);
  };

  visit(source);
  return classes;
}

function hasInterfaceDeclaration(sourceText: string, name: string) {
  const pattern = new RegExp(`interface\\s+${name}\\b`);
  return pattern.test(sourceText);
}

function buildInterfaceBlock(
  name: string,
  mixinRefs: string[],
  isExported: boolean,
) {
  if (!mixinRefs.length) {
    return "";
  }

  const aliasName = `__TsMacros${name}Mixin`;
  const intersection =
    mixinRefs.length === 1 ? mixinRefs[0] : mixinRefs.join(" & ");
  const exportPrefix = isExported ? "export " : "";

  return `${exportPrefix}type ${aliasName} = ${intersection};\n${exportPrefix}interface ${name} extends ${aliasName} {}\n`;
}

function isNodeExported(tsModule: typeof ts, node: ts.ClassDeclaration) {
  const flags = tsModule.getCombinedModifierFlags(node);
  return (flags & tsModule.ModifierFlags.Export) !== 0;
}

function augmentSource(
  tsModule: typeof ts,
  fileName: string,
  sourceText: string,
  macroNames: Set<string>,
  mixinModule: string,
  mixinTypes: string[],
) {
  if (!sourceText.includes("@")) {
    return null;
  }

  const source = tsModule.createSourceFile(
    fileName,
    sourceText,
    tsModule.ScriptTarget.Latest,
    true,
    tsModule.ScriptKind.TSX,
  );

  const decorated = collectDecoratedClasses(tsModule, source, macroNames);
  if (decorated.length === 0) {
    return null;
  }

  const mixinRefs = mixinTypes.map(
    (type) => `import("${mixinModule}").${type}`,
  );
  const additions = decorated
    .filter((meta) => !hasInterfaceDeclaration(sourceText, meta.name))
    .map((meta) => buildInterfaceBlock(meta.name, mixinRefs, meta.isExported));

  if (!additions.length) {
    return null;
  }

  return `${sourceText}${AUGMENTATION_BANNER}${additions.join("")}`;
}

function init(modules: { typescript: typeof ts }) {
  function create(info: ts.server.PluginCreateInfo) {
    const tsModule = modules.typescript;

    const config: PluginConfig = info.config ?? {};
    const macroNames = new Set(config.macroNames ?? DEFAULT_MACROS);
    const mixinTypes = config.mixinTypes ?? DEFAULT_MIXIN_TYPES;
    const mixinModule = config.mixinModule ?? "$lib/macros";

    const originalGetScriptSnapshot =
      info.languageServiceHost.getScriptSnapshot?.bind(
        info.languageServiceHost,
      );

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
        const augmented = augmentSource(
          tsModule,
          fileName,
          text,
          macroNames,
          mixinModule,
          mixinTypes,
        );

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

export = init;
