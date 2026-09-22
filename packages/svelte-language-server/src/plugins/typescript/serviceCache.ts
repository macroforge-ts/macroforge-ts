// abstracting the typescript-auto-import-cache package to support our use case

import {
    createProjectService as createProjectService50,
    type ProjectService
} from 'typescript-auto-import-cache/out/5_0/projectService.js';
import { createProject as createProject50 } from 'typescript-auto-import-cache/out/5_0/project.js';
import { createProject as createProject53 } from 'typescript-auto-import-cache/out/5_3/project.js';
import { createProject as createProject55 } from 'typescript-auto-import-cache/out/5_5/project.js';
import ts from 'typescript';
import type { ExportInfoMap } from 'typescript-auto-import-cache/out/5_0/exportInfoMap.js';
import type { ModuleSpecifierCache } from 'typescript-auto-import-cache/out/5_0/moduleSpecifierCache.js';
import type { SymlinkCache } from 'typescript-auto-import-cache/out/5_0/symlinkCache.js';
import type { ProjectPackageJsonInfo } from 'typescript-auto-import-cache/out/5_0/packageJsonCache.js';

export type { ProjectService };

// TypeScript internal LanguageServiceHost extensions
// (declare module not allowed on JSR - these are applied via (host as any) at runtime)

export function createProjectService(
    system: ts.System,
    hostConfiguration: {
        preferences: ts.UserPreferences;
    }
) {
    const version = ts.version.split('.');
    const major = parseInt(version[0]);

    if (major < 5) {
        return undefined;
    }

    const projectService = createProjectService50(
        ts,
        system,
        system.getCurrentDirectory(),
        hostConfiguration,
        ts.LanguageServiceMode.Semantic
    );

    return projectService;
}

export function createProject(
    host: ts.LanguageServiceHost,
    createLanguageService: (host: ts.LanguageServiceHost) => ts.LanguageService,
    options: {
        projectService: ProjectService;
        compilerOptions: ts.CompilerOptions;
        currentDirectory: string;
    }
) {
    const version = ts.version.split('.');
    const major = parseInt(version[0]);
    const minor = parseInt(version[1]);

    if (major < 5) {
        return undefined;
    }

    const factory = minor < 3 ? createProject50 : minor < 5 ? createProject53 : createProject55;
    const project = factory(ts, host, createLanguageService, options);

    const proxyMethods: (keyof typeof project)[] = [
        'getCachedExportInfoMap',
        'getModuleSpecifierCache',
        'getGlobalTypingsCacheLocation',
        'getSymlinkCache',
        'getPackageJsonsVisibleToFile',
        'getPackageJsonAutoImportProvider',
        'includePackageJsonAutoImports'
        // Volar doesn't have the "languageServiceReducedMode" support but we do
        // so don't proxy this method and implement this directly in the ts.LanguageServiceHost
        // 'useSourceOfProjectReferenceRedirect'
    ];
    proxyMethods.forEach((
        key
    ) => ((host as any)[key] = project[key].bind(project)));

    if (host.log) {
        project.log = host.log.bind(host);
    }

    return project;
}
