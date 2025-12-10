#!/usr/bin/env node

/**
 * Extract documentation from website Svelte pages into markdown files for MCP server.
 * Auto-discovers all pages from the website's navigation.ts config.
 * Large documents are automatically chunked at H2 headers for better AI consumption.
 *
 * Usage: node scripts/extract-docs.cjs
 */

const fs = require('fs');
const path = require('path');

const websiteRoot = path.join(__dirname, '..', '..', '..', 'website');
const routesDir = path.join(websiteRoot, 'src', 'routes');
const navigationPath = path.join(websiteRoot, 'src', 'lib', 'config', 'navigation.ts');
const outputDir = path.join(__dirname, '..', 'docs');

// Chunking thresholds
// 6KB threshold means only the largest 3-4 docs get chunked (ts-quote, ts-macro-derive, deserialize)
// This balances context efficiency with keeping most docs intact
const CHUNK_SIZE_THRESHOLD = 6000; // bytes - chunk docs larger than this
const MIN_CHUNK_SIZE = 500; // bytes - don't create chunks smaller than this

// Use cases for each section (keyed by href)
const useCasesMap = {
  // Getting Started
  '/docs/getting-started': 'setup, install, npm, getting started, quick start, init',
  '/docs/getting-started/first-macro': 'tutorial, example, hello world, beginner, learn',

  // Core Concepts
  '/docs/concepts': 'architecture, overview, understanding, basics, fundamentals',
  '/docs/concepts/derive-system': '@derive, decorator, annotation, derive macro',
  '/docs/concepts/architecture': 'internals, rust, swc, napi, how it works',

  // Built-in Macros
  '/docs/builtin-macros': 'all macros, list, available macros, macro list',
  '/docs/builtin-macros/debug': 'toString, debugging, logging, output, print',
  '/docs/builtin-macros/clone': 'copy, clone, duplicate, shallow copy, immutable',
  '/docs/builtin-macros/default': 'default values, factory, initialization, constructor',
  '/docs/builtin-macros/hash': 'hashCode, hashing, hash map, equality, hash function',
  '/docs/builtin-macros/ord': 'compareTo, ordering, sorting, comparison, total order',
  '/docs/builtin-macros/partial-eq': 'equals, equality, comparison, value equality',
  '/docs/builtin-macros/partial-ord': 'compareTo, partial ordering, sorting, nullable comparison',
  '/docs/builtin-macros/serialize': 'toJSON, serialization, json, api, data transfer',
  '/docs/builtin-macros/deserialize': 'fromJSON, deserialization, parsing, validation, json',

  // Custom Macros
  '/docs/custom-macros': 'custom, extending, creating macros, own macro',
  '/docs/custom-macros/rust-setup': 'rust, cargo, napi, compilation, building',
  '/docs/custom-macros/ts-macro-derive': 'attribute, proc macro, derive attribute, rust macro',
  '/docs/custom-macros/ts-quote': 'ts_quote, template, code generation, interpolation',

  // Integration
  '/docs/integration': 'setup, integration, tools, ecosystem',
  '/docs/integration/cli': 'command line, macroforge command, expand, terminal',
  '/docs/integration/typescript-plugin': 'vscode, ide, language server, intellisense, autocomplete',
  '/docs/integration/vite-plugin': 'vite, build, bundler, react, svelte, sveltekit',
  '/docs/integration/svelte-preprocessor': 'svelte, preprocessor, svelte components, .svelte files, sveltekit',
  '/docs/integration/mcp-server': 'mcp, ai, claude, llm, model context protocol, assistant',
  '/docs/integration/configuration': 'macroforge.json, config, settings, options',

  // Language Servers
  '/docs/language-servers': 'lsp, language server, editor support',
  '/docs/language-servers/svelte': 'svelte, svelte language server, .svelte files',
  '/docs/language-servers/zed': 'zed, zed editor, extension',

  // API Reference
  '/docs/api': 'api, functions, exports, programmatic',
  '/docs/api/expand-sync': 'expandSync, expand, transform, macro expansion',
  '/docs/api/transform-sync': 'transformSync, transform, metadata, low-level',
  '/docs/api/native-plugin': 'NativePlugin, caching, language server, stateful',
  '/docs/api/position-mapper': 'PositionMapper, source map, diagnostics, position',

  // Roadmap
  '/docs/roadmap': 'roadmap, future, planned features, upcoming'
};

/**
 * Parse the navigation.ts file to extract the navigation structure
 */
function parseNavigation() {
  const content = fs.readFileSync(navigationPath, 'utf-8');
  const sections = [];

  // Match each section block
  const sectionRegex = /\{\s*title:\s*['"]([^'"]+)['"]\s*,\s*items:\s*\[([\s\S]*?)\]\s*\}/g;
  let sectionMatch;

  while ((sectionMatch = sectionRegex.exec(content)) !== null) {
    const sectionTitle = sectionMatch[1];
    const itemsContent = sectionMatch[2];

    const items = [];
    const itemRegex = /\{\s*title:\s*['"]([^'"]+)['"]\s*,\s*href:\s*['"]([^'"]+)['"]\s*\}/g;
    let itemMatch;

    while ((itemMatch = itemRegex.exec(itemsContent)) !== null) {
      items.push({
        title: itemMatch[1],
        href: itemMatch[2]
      });
    }

    if (items.length > 0) {
      sections.push({
        title: sectionTitle,
        items
      });
    }
  }

  return sections;
}

/**
 * Convert href to category slug
 */
function hrefToCategory(href) {
  // /docs/getting-started/first-macro -> getting-started
  const parts = href.replace('/docs/', '').split('/');
  return parts[0];
}

/**
 * Convert href to item ID
 */
function hrefToId(href) {
  // /docs/getting-started/first-macro -> first-macro
  // /docs/getting-started -> installation (special case for index pages)
  const parts = href.replace('/docs/', '').split('/');
  if (parts.length === 1) {
    // Index page - use a descriptive name based on the category
    const categoryIdMap = {
      'getting-started': 'installation',
      'concepts': 'how-macros-work',
      'builtin-macros': 'macros-overview',
      'custom-macros': 'custom-overview',
      'integration': 'integration-overview',
      'language-servers': 'ls-overview',
      'api': 'api-overview',
      'roadmap': 'roadmap'
    };
    return categoryIdMap[parts[0]] || parts[0];
  }
  return parts[parts.length - 1];
}

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
 * Convert a header text to a URL-friendly slug
 */
function headerToSlug(header) {
  return header
    .toLowerCase()
    // Remove common code syntax characters but keep meaningful words
    .replace(/`([^`]+)`/g, '$1')  // Remove backticks but keep content
    .replace(/&#\d+;/g, '')       // Remove HTML entities like &#123;
    .replace(/&[a-z]+;/g, '')     // Remove named HTML entities
    .replace(/[^a-z0-9\s-]/g, '') // Remove other special characters
    .replace(/\s+/g, '-')         // Replace spaces with hyphens
    .replace(/-+/g, '-')          // Collapse multiple hyphens
    .replace(/^-|-$/g, '');       // Remove leading/trailing hyphens
}

/**
 * Extract use cases keywords from chunk content
 */
function extractChunkUseCases(content, parentUseCases) {
  // Extract key terms from the chunk content
  const keywords = [];

  // Get code-related terms (function names, decorators, etc.)
  const codeMatches = content.match(/`([^`]+)`/g) || [];
  for (const match of codeMatches.slice(0, 5)) {
    const term = match.replace(/`/g, '').toLowerCase();
    if (term.length > 2 && term.length < 30 && !term.includes(' ')) {
      keywords.push(term);
    }
  }

  // Include some parent use cases for context
  const parentKeywords = parentUseCases.split(',').map(k => k.trim()).slice(0, 2);

  return [...new Set([...parentKeywords, ...keywords])].slice(0, 6).join(', ');
}

/**
 * Split markdown content into chunks at H2 headers
 * Returns array of { slug, title, content } objects
 */
function chunkMarkdown(markdown, parentTitle) {
  const chunks = [];

  // Split at ## headers (H2)
  const h2Regex = /^## (.+)$/gm;
  const parts = markdown.split(h2Regex);

  // First part is content before any H2 (overview/intro)
  if (parts[0] && parts[0].trim().length >= MIN_CHUNK_SIZE) {
    chunks.push({
      slug: 'overview',
      title: `${parentTitle}: Overview`,
      content: parts[0].trim()
    });
  } else if (parts[0]) {
    // If intro is too small, prepend it to first chunk
    chunks.push({
      slug: '_intro',
      title: '',
      content: parts[0].trim()
    });
  }

  // Process remaining parts (alternating: header, content, header, content...)
  for (let i = 1; i < parts.length; i += 2) {
    const header = parts[i];
    const content = parts[i + 1] || '';

    if (!header) continue;

    const slug = headerToSlug(header);
    const fullContent = `## ${header}\n\n${content.trim()}`;

    // Check if we should merge with previous chunk (if content is too small)
    if (fullContent.length < MIN_CHUNK_SIZE && chunks.length > 0) {
      const lastChunk = chunks[chunks.length - 1];
      if (lastChunk.slug !== '_intro') {
        lastChunk.content += '\n\n' + fullContent;
        continue;
      }
    }

    chunks.push({
      slug,
      title: `${parentTitle}: ${header}`,
      content: fullContent
    });
  }

  // Handle the intro content - merge it into first real chunk
  if (chunks.length > 1 && chunks[0].slug === '_intro') {
    const intro = chunks.shift();
    chunks[0].content = intro.content + '\n\n' + chunks[0].content;
    chunks[0].slug = 'overview';
    chunks[0].title = `${parentTitle}: Overview`;
  } else if (chunks.length === 1 && chunks[0].slug === '_intro') {
    // Only intro content, make it the overview
    chunks[0].slug = 'overview';
    chunks[0].title = `${parentTitle}: Overview`;
  }

  return chunks;
}

/**
 * Determine if content should be chunked based on size
 */
function shouldChunk(content) {
  return content.length > CHUNK_SIZE_THRESHOLD;
}

/**
 * Extract documentation from website and generate markdown files
 */
function extractDocs() {
  console.log('Auto-discovering pages from navigation.ts...\n');

  // Parse navigation from website
  const navigation = parseNavigation();

  console.log(`Found ${navigation.length} sections in navigation.ts\n`);

  // Ensure output directory exists
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  const sections = [];

  for (const section of navigation) {
    const category = hrefToCategory(section.items[0]?.href || '');

    // Create category directory
    const categoryDir = path.join(outputDir, category);
    if (!fs.existsSync(categoryDir)) {
      fs.mkdirSync(categoryDir, { recursive: true });
    }

    for (const item of section.items) {
      const filePath = hrefToFilePath(item.href);
      const itemId = hrefToId(item.href);

      if (!fs.existsSync(filePath)) {
        console.warn(`Warning: File not found: ${filePath}`);
        continue;
      }

      console.log(`Processing: ${item.title} (${item.href})`);

      const svelteContent = fs.readFileSync(filePath, 'utf-8');
      const htmlContent = extractContent(svelteContent);
      const markdownContent = htmlToMarkdown(htmlContent);

      // Get use_cases from map or generate default
      const useCases = useCasesMap[item.href] || item.title.toLowerCase();

      // Check if we need to chunk this document
      if (shouldChunk(markdownContent)) {
        const chunks = chunkMarkdown(markdownContent, item.title);

        if (chunks.length > 1) {
          console.log(`  → Chunking into ${chunks.length} parts`);

          // Create subdirectory for chunks
          const chunkDir = path.join(categoryDir, itemId);
          if (!fs.existsSync(chunkDir)) {
            fs.mkdirSync(chunkDir, { recursive: true });
          }

          const chunkIds = [];

          // Write each chunk
          for (const chunk of chunks) {
            const chunkId = `${itemId}/${chunk.slug}`;
            const chunkPath = path.join(chunkDir, `${chunk.slug}.md`);
            fs.writeFileSync(chunkPath, chunk.content);

            chunkIds.push(chunkId);

            // Add chunk to sections
            sections.push({
              id: chunkId,
              title: chunk.title,
              category: category,
              category_title: section.title,
              path: `${category}/${itemId}/${chunk.slug}.md`,
              use_cases: extractChunkUseCases(chunk.content, useCases),
              parent_id: itemId
            });
          }

          // Add parent entry (marked as chunked, no content file)
          sections.push({
            id: itemId,
            title: item.title,
            category: category,
            category_title: section.title,
            path: `${category}/${itemId}.md`,
            use_cases: useCases,
            is_chunked: true,
            chunk_ids: chunkIds
          });

          // Also write the full file for reference (but loader won't use it for chunked sections)
          const outputPath = path.join(categoryDir, `${itemId}.md`);
          fs.writeFileSync(outputPath, markdownContent);

          continue;
        }
      }

      // Not chunked - write as single file
      const outputPath = path.join(categoryDir, `${itemId}.md`);
      fs.writeFileSync(outputPath, markdownContent);

      // Add to sections list
      sections.push({
        id: itemId,
        title: item.title,
        category: category,
        category_title: section.title,
        path: `${category}/${itemId}.md`,
        use_cases: useCases
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
