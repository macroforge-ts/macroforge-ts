#!/usr/bin/env node

/**
 * Extract documentation from website Svelte pages into markdown files for MCP server.
 *
 * Usage: node scripts/extract-docs.cjs
 */

const fs = require('fs');
const path = require('path');

const websiteRoot = path.join(__dirname, '..', '..', '..', 'website');
const routesDir = path.join(websiteRoot, 'src', 'routes');
const outputDir = path.join(__dirname, '..', 'docs');

// Navigation structure with use_cases for each section
const navigation = [
  {
    category: 'getting-started',
    title: 'Getting Started',
    items: [
      {
        id: 'installation',
        title: 'Installation',
        href: '/docs/getting-started',
        use_cases: 'setup, install, npm, getting started, quick start, init'
      },
      {
        id: 'first-macro',
        title: 'First Macro',
        href: '/docs/getting-started/first-macro',
        use_cases: 'tutorial, example, hello world, beginner, learn'
      }
    ]
  },
  {
    category: 'concepts',
    title: 'Core Concepts',
    items: [
      {
        id: 'how-macros-work',
        title: 'How Macros Work',
        href: '/docs/concepts',
        use_cases: 'architecture, overview, understanding, basics, fundamentals'
      },
      {
        id: 'derive-system',
        title: 'The Derive System',
        href: '/docs/concepts/derive-system',
        use_cases: '@derive, decorator, annotation, derive macro'
      },
      {
        id: 'architecture',
        title: 'Architecture',
        href: '/docs/concepts/architecture',
        use_cases: 'internals, rust, swc, napi, how it works'
      }
    ]
  },
  {
    category: 'builtin-macros',
    title: 'Built-in Macros',
    items: [
      {
        id: 'macros-overview',
        title: 'Overview',
        href: '/docs/builtin-macros',
        use_cases: 'all macros, list, available macros, macro list'
      },
      {
        id: 'debug',
        title: 'Debug',
        href: '/docs/builtin-macros/debug',
        use_cases: 'toString, debugging, logging, output, print'
      },
      {
        id: 'clone',
        title: 'Clone',
        href: '/docs/builtin-macros/clone',
        use_cases: 'copy, clone, duplicate, shallow copy, immutable'
      },
      {
        id: 'default',
        title: 'Default',
        href: '/docs/builtin-macros/default',
        use_cases: 'default values, factory, initialization, constructor'
      },
      {
        id: 'hash',
        title: 'Hash',
        href: '/docs/builtin-macros/hash',
        use_cases: 'hashCode, hashing, hash map, equality, hash function'
      },
      {
        id: 'ord',
        title: 'Ord',
        href: '/docs/builtin-macros/ord',
        use_cases: 'compareTo, ordering, sorting, comparison, total order'
      },
      {
        id: 'partial-eq',
        title: 'PartialEq',
        href: '/docs/builtin-macros/partial-eq',
        use_cases: 'equals, equality, comparison, value equality'
      },
      {
        id: 'partial-ord',
        title: 'PartialOrd',
        href: '/docs/builtin-macros/partial-ord',
        use_cases: 'compareTo, partial ordering, sorting, nullable comparison'
      },
      {
        id: 'serialize',
        title: 'Serialize',
        href: '/docs/builtin-macros/serialize',
        use_cases: 'toJSON, serialization, json, api, data transfer'
      },
      {
        id: 'deserialize',
        title: 'Deserialize',
        href: '/docs/builtin-macros/deserialize',
        use_cases: 'fromJSON, deserialization, parsing, validation, json'
      }
    ]
  },
  {
    category: 'custom-macros',
    title: 'Custom Macros',
    items: [
      {
        id: 'custom-overview',
        title: 'Overview',
        href: '/docs/custom-macros',
        use_cases: 'custom, extending, creating macros, own macro'
      },
      {
        id: 'rust-setup',
        title: 'Rust Setup',
        href: '/docs/custom-macros/rust-setup',
        use_cases: 'rust, cargo, napi, compilation, building'
      },
      {
        id: 'ts-macro-derive',
        title: 'ts_macro_derive',
        href: '/docs/custom-macros/ts-macro-derive',
        use_cases: 'attribute, proc macro, derive attribute, rust macro'
      },
      {
        id: 'ts-quote',
        title: 'Template Syntax',
        href: '/docs/custom-macros/ts-quote',
        use_cases: 'ts_quote, template, code generation, interpolation'
      }
    ]
  },
  {
    category: 'integration',
    title: 'Integration',
    items: [
      {
        id: 'integration-overview',
        title: 'Overview',
        href: '/docs/integration',
        use_cases: 'setup, integration, tools, ecosystem'
      },
      {
        id: 'cli',
        title: 'CLI',
        href: '/docs/integration/cli',
        use_cases: 'command line, macroforge command, expand, terminal'
      },
      {
        id: 'typescript-plugin',
        title: 'TypeScript Plugin',
        href: '/docs/integration/typescript-plugin',
        use_cases: 'vscode, ide, language server, intellisense, autocomplete'
      },
      {
        id: 'vite-plugin',
        title: 'Vite Plugin',
        href: '/docs/integration/vite-plugin',
        use_cases: 'vite, build, bundler, react, svelte, sveltekit'
      },
      {
        id: 'configuration',
        title: 'Configuration',
        href: '/docs/integration/configuration',
        use_cases: 'macroforge.json, config, settings, options'
      }
    ]
  },
  {
    category: 'language-servers',
    title: 'Language Servers',
    items: [
      {
        id: 'ls-overview',
        title: 'Overview',
        href: '/docs/language-servers',
        use_cases: 'lsp, language server, editor support'
      },
      {
        id: 'svelte-ls',
        title: 'Svelte',
        href: '/docs/language-servers/svelte',
        use_cases: 'svelte, svelte language server, .svelte files'
      },
      {
        id: 'zed-extensions',
        title: 'Zed Extensions',
        href: '/docs/language-servers/zed',
        use_cases: 'zed, zed editor, extension'
      }
    ]
  },
  {
    category: 'api',
    title: 'API Reference',
    items: [
      {
        id: 'api-overview',
        title: 'Overview',
        href: '/docs/api',
        use_cases: 'api, functions, exports, programmatic'
      },
      {
        id: 'expand-sync',
        title: 'expandSync()',
        href: '/docs/api/expand-sync',
        use_cases: 'expandSync, expand, transform, macro expansion'
      },
      {
        id: 'transform-sync',
        title: 'transformSync()',
        href: '/docs/api/transform-sync',
        use_cases: 'transformSync, transform, metadata, low-level'
      },
      {
        id: 'native-plugin',
        title: 'NativePlugin',
        href: '/docs/api/native-plugin',
        use_cases: 'NativePlugin, caching, language server, stateful'
      },
      {
        id: 'position-mapper',
        title: 'PositionMapper',
        href: '/docs/api/position-mapper',
        use_cases: 'PositionMapper, source map, diagnostics, position'
      }
    ]
  }
];

/**
 * Convert href to file path
 */
function hrefToFilePath(href) {
  return path.join(routesDir, href, '+page.svelte');
}

/**
 * Extract content from Svelte file (remove script tags and convert to markdown)
 */
function extractContent(svelteContent) {
  let content = svelteContent.replace(/<script[^>]*>[\s\S]*?<\/script>/gi, '');
  content = content.replace(/<svelte:head>[\s\S]*?<\/svelte:head>/gi, '');
  content = content.replace(/<style[^>]*>[\s\S]*?<\/style>/gi, '');
  return content.trim();
}

/**
 * Convert HTML/Svelte content to Markdown
 */
function htmlToMarkdown(html) {
  let md = html.replace(/\t/g, '').replace(/\n\s*\n/g, '\n\n');

  // Handle CodeBlock components
  md = md.replace(/<CodeBlock\s+code=\{`([\s\S]*?)`\}\s+lang="([^"]+)"(?:\s+filename="([^"]+)")?\s*\/>/g,
    (_, code, lang, filename) => {
      const header = filename ? `\`${filename}\`\n` : '';
      return `${header}\`\`\`${lang}\n${code.trim()}\n\`\`\``;
    });

  md = md.replace(/<CodeBlock\s+code="([^"]+)"\s+lang="([^"]+)"(?:\s+filename="([^"]+)")?\s*\/>/g,
    (_, code, lang, filename) => {
      const header = filename ? `\`${filename}\`\n` : '';
      return `${header}\`\`\`${lang}\n${code.trim()}\n\`\`\``;
    });

  // Handle Alert components
  md = md.replace(/<Alert\s+type="([^"]+)">([\s\S]*?)<\/Alert>/g,
    (_, type, content) => {
      const prefix = type === 'warning' ? '> **Warning:**' :
                     type === 'info' ? '> **Note:**' :
                     type === 'danger' ? '> **Danger:**' : '>';
      const lines = content.trim().split('\n').map(l => `> ${l.trim()}`).join('\n');
      return `${prefix}\n${lines}`;
    });

  // Handle headings
  md = md.replace(/<h1[^>]*>(.*?)<\/h1>/gi, '# $1\n');
  md = md.replace(/<h2[^>]*>(.*?)<\/h2>/gi, '## $1\n');
  md = md.replace(/<h3[^>]*>(.*?)<\/h3>/gi, '### $1\n');
  md = md.replace(/<h4[^>]*>(.*?)<\/h4>/gi, '#### $1\n');

  // Handle paragraphs
  md = md.replace(/<p class="lead">([\s\S]*?)<\/p>/gi, (_, content) => {
    return '*' + content.replace(/\s+/g, ' ').trim() + '*\n';
  });
  md = md.replace(/<p>([\s\S]*?)<\/p>/gi, (_, content) => {
    return content.replace(/\s+/g, ' ').trim() + '\n';
  });

  // Handle inline code
  md = md.replace(/<code>(.*?)<\/code>/gi, '`$1`');

  // Handle links
  md = md.replace(/<a href="([^"]+)">(.*?)<\/a>/gi, '[$2]($1)');

  // Handle strong/bold
  md = md.replace(/<strong>(.*?)<\/strong>/gi, '**$1**');
  md = md.replace(/<b>(.*?)<\/b>/gi, '**$1**');

  // Handle emphasis/italic
  md = md.replace(/<em>(.*?)<\/em>/gi, '*$1*');
  md = md.replace(/<i>(.*?)<\/i>/gi, '*$1*');

  // Handle lists
  md = md.replace(/<ul[^>]*>([\s\S]*?)<\/ul>/gi, (_, items) => {
    return items.replace(/<li>([\s\S]*?)<\/li>/gi, (_, content) => {
      return '- ' + content.replace(/\s+/g, ' ').trim() + '\n';
    }).trim() + '\n';
  });

  md = md.replace(/<ol[^>]*>([\s\S]*?)<\/ol>/gi, (_, items) => {
    let counter = 0;
    return items.replace(/<li>([\s\S]*?)<\/li>/gi, (_, content) => {
      counter++;
      return `${counter}. ` + content.replace(/\s+/g, ' ').trim() + '\n';
    }).trim() + '\n';
  });

  // Handle tables (basic support)
  md = md.replace(/<table[^>]*>([\s\S]*?)<\/table>/gi, (_, content) => {
    // Simplified table handling - just extract text
    return content
      .replace(/<thead[^>]*>[\s\S]*?<\/thead>/gi, '')
      .replace(/<tbody[^>]*>/gi, '')
      .replace(/<\/tbody>/gi, '')
      .replace(/<tr[^>]*>/gi, '')
      .replace(/<\/tr>/gi, '\n')
      .replace(/<th[^>]*>([\s\S]*?)<\/th>/gi, '| $1 ')
      .replace(/<td[^>]*>([\s\S]*?)<\/td>/gi, '| $1 ')
      .trim() + '\n';
  });

  // Handle other elements
  md = md.replace(/<hr\s*\/?>/gi, '\n---\n');
  md = md.replace(/<br\s*\/?>/gi, '\n');
  md = md.replace(/<div[^>]*>([\s\S]*?)<\/div>/gi, '$1');
  md = md.replace(/<span[^>]*>([\s\S]*?)<\/span>/gi, '$1');

  // Clean up
  md = md.replace(/\n{3,}/g, '\n\n');
  md = md.replace(/^\s+|\s+$/g, '');

  // Decode HTML entities
  md = md.replace(/&lt;/g, '<');
  md = md.replace(/&gt;/g, '>');
  md = md.replace(/&amp;/g, '&');
  md = md.replace(/&quot;/g, '"');
  md = md.replace(/&#39;/g, "'");

  return md;
}

/**
 * Extract documentation from website and generate markdown files
 */
function extractDocs() {
  // Ensure output directory exists
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  const sections = [];

  for (const category of navigation) {
    // Create category directory
    const categoryDir = path.join(outputDir, category.category);
    if (!fs.existsSync(categoryDir)) {
      fs.mkdirSync(categoryDir, { recursive: true });
    }

    for (const item of category.items) {
      const filePath = hrefToFilePath(item.href);

      if (!fs.existsSync(filePath)) {
        console.warn(`Warning: File not found: ${filePath}`);
        continue;
      }

      console.log(`Processing: ${item.title} (${filePath})`);

      const svelteContent = fs.readFileSync(filePath, 'utf-8');
      const htmlContent = extractContent(svelteContent);
      const markdownContent = htmlToMarkdown(htmlContent);

      // Write markdown file
      const outputPath = path.join(categoryDir, `${item.id}.md`);
      fs.writeFileSync(outputPath, markdownContent);

      // Add to sections list
      sections.push({
        id: item.id,
        title: item.title,
        category: category.category,
        category_title: category.title,
        path: `${category.category}/${item.id}.md`,
        use_cases: item.use_cases
      });
    }
  }

  // Write sections.json
  const sectionsPath = path.join(outputDir, 'sections.json');
  fs.writeFileSync(sectionsPath, JSON.stringify(sections, null, 2));

  console.log(`\nExtracted ${sections.length} documentation sections`);
  console.log(`Output directory: ${outputDir}`);
}

// Run
extractDocs();
