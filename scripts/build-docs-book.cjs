#!/usr/bin/env node

/**
 * Build a markdown book from the website documentation pages.
 *
 * Usage: node scripts/build-docs-book.cjs [output-path]
 *
 * Output defaults to: docs/BOOK.md
 */

const fs = require('fs');
const path = require('path');

const websiteRoot = path.join(__dirname, '..', 'website');
const routesDir = path.join(websiteRoot, 'src', 'routes');
const defaultOutput = path.join(__dirname, '..', 'docs', 'BOOK.md');

// Navigation structure (mirrored from website/src/lib/config/navigation.ts)
const navigation = [
  {
    title: 'Getting Started',
    items: [
      { title: 'Installation', href: '/docs/getting-started' },
      { title: 'First Macro', href: '/docs/getting-started/first-macro' }
    ]
  },
  {
    title: 'Core Concepts',
    items: [
      { title: 'How Macros Work', href: '/docs/concepts' },
      { title: 'The Derive System', href: '/docs/concepts/derive-system' },
      { title: 'Architecture', href: '/docs/concepts/architecture' }
    ]
  },
  {
    title: 'Built-in Macros',
    items: [
      { title: 'Overview', href: '/docs/builtin-macros' },
      { title: 'Debug', href: '/docs/builtin-macros/debug' },
      { title: 'Clone', href: '/docs/builtin-macros/clone' },
      { title: 'Default', href: '/docs/builtin-macros/default' },
      { title: 'Hash', href: '/docs/builtin-macros/hash' },
      { title: 'Ord', href: '/docs/builtin-macros/ord' },
      { title: 'PartialEq', href: '/docs/builtin-macros/partial-eq' },
      { title: 'PartialOrd', href: '/docs/builtin-macros/partial-ord' },
      { title: 'Serialize', href: '/docs/builtin-macros/serialize' },
      { title: 'Deserialize', href: '/docs/builtin-macros/deserialize' }
    ]
  },
  {
    title: 'Custom Macros',
    items: [
      { title: 'Overview', href: '/docs/custom-macros' },
      { title: 'Rust Setup', href: '/docs/custom-macros/rust-setup' },
      { title: 'ts_macro_derive', href: '/docs/custom-macros/ts-macro-derive' },
      { title: 'Template Syntax', href: '/docs/custom-macros/ts-quote' }
    ]
  },
  {
    title: 'Integration',
    items: [
      { title: 'Overview', href: '/docs/integration' },
      { title: 'CLI', href: '/docs/integration/cli' },
      { title: 'TypeScript Plugin', href: '/docs/integration/typescript-plugin' },
      { title: 'Vite Plugin', href: '/docs/integration/vite-plugin' },
      { title: 'Configuration', href: '/docs/integration/configuration' }
    ]
  },
  {
    title: 'Language Servers',
    items: [
      { title: 'Overview', href: '/docs/language-servers' },
      { title: 'Svelte', href: '/docs/language-servers/svelte' },
      { title: 'Zed Extensions', href: '/docs/language-servers/zed' }
    ]
  },
  {
    title: 'API Reference',
    items: [
      { title: 'Overview', href: '/docs/api' },
      { title: 'expandSync()', href: '/docs/api/expand-sync' },
      { title: 'transformSync()', href: '/docs/api/transform-sync' },
      { title: 'NativePlugin', href: '/docs/api/native-plugin' },
      { title: 'PositionMapper', href: '/docs/api/position-mapper' }
    ]
  }
];

/**
 * Convert href to file path
 */
function hrefToFilePath(href) {
  // /docs/getting-started -> website/src/routes/docs/getting-started/+page.svelte
  return path.join(routesDir, href, '+page.svelte');
}

/**
 * Extract content from Svelte file (remove script tags and convert to markdown)
 */
function extractContent(svelteContent) {
  // Remove <script> blocks
  let content = svelteContent.replace(/<script[^>]*>[\s\S]*?<\/script>/gi, '');

  // Remove <svelte:head> blocks
  content = content.replace(/<svelte:head>[\s\S]*?<\/svelte:head>/gi, '');

  // Remove <style> blocks
  content = content.replace(/<style[^>]*>[\s\S]*?<\/style>/gi, '');

  return content.trim();
}

/**
 * Convert HTML/Svelte content to Markdown
 */
function htmlToMarkdown(html) {
  // First, normalize whitespace in the HTML (remove tabs and excess whitespace)
  let md = html.replace(/\t/g, '').replace(/\n\s*\n/g, '\n\n');

  // Handle CodeBlock components - extract code and lang
  md = md.replace(/<CodeBlock\s+code=\{`([\s\S]*?)`\}\s+lang="([^"]+)"(?:\s+filename="([^"]+)")?\s*\/>/g,
    (_, code, lang, filename) => {
      const header = filename ? `\`${filename}\`\n` : '';
      return `${header}\`\`\`${lang}\n${code.trim()}\n\`\`\``;
    });

  // Handle CodeBlock with single-line code attribute
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

  // Handle paragraphs with class (normalize internal whitespace)
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

  // Handle unordered lists
  md = md.replace(/<ul[^>]*>([\s\S]*?)<\/ul>/gi, (_, items) => {
    return items.replace(/<li>([\s\S]*?)<\/li>/gi, (_, content) => {
      return '- ' + content.replace(/\s+/g, ' ').trim() + '\n';
    }).trim() + '\n';
  });

  // Handle ordered lists
  md = md.replace(/<ol[^>]*>([\s\S]*?)<\/ol>/gi, (_, items) => {
    let counter = 0;
    return items.replace(/<li>([\s\S]*?)<\/li>/gi, (_, content) => {
      counter++;
      return `${counter}. ` + content.replace(/\s+/g, ' ').trim() + '\n';
    }).trim() + '\n';
  });

  // Handle definition lists / description lists
  md = md.replace(/<dl[^>]*>([\s\S]*?)<\/dl>/gi, (_, content) => {
    let result = content;
    result = result.replace(/<dt>([\s\S]*?)<\/dt>/gi, '**$1**\n');
    result = result.replace(/<dd>([\s\S]*?)<\/dd>/gi, ': $1\n');
    return result;
  });

  // Handle blockquotes
  md = md.replace(/<blockquote>([\s\S]*?)<\/blockquote>/gi, (_, content) => {
    return content.trim().split('\n').map(l => `> ${l.trim()}`).join('\n') + '\n';
  });

  // Handle pre/code blocks (non-component)
  md = md.replace(/<pre><code class="language-(\w+)">([\s\S]*?)<\/code><\/pre>/gi,
    '```$1\n$2\n```');
  md = md.replace(/<pre>([\s\S]*?)<\/pre>/gi, '```\n$1\n```');

  // Handle horizontal rules
  md = md.replace(/<hr\s*\/?>/gi, '\n---\n');

  // Handle breaks
  md = md.replace(/<br\s*\/?>/gi, '\n');

  // Handle divs (just extract content)
  md = md.replace(/<div[^>]*>([\s\S]*?)<\/div>/gi, '$1');

  // Handle spans (just extract content)
  md = md.replace(/<span[^>]*>([\s\S]*?)<\/span>/gi, '$1');

  // Clean up extra whitespace
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
 * Build the markdown book
 */
function buildBook() {
  const outputPath = process.argv[2] || defaultOutput;

  // Ensure output directory exists
  const outputDir = path.dirname(outputPath);
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  let book = [];

  // Add title page
  book.push('# Macroforge Documentation');
  book.push('');
  book.push('*TypeScript Macros - Rust-Powered Code Generation*');
  book.push('');
  book.push('---');
  book.push('');

  // Add table of contents
  book.push('## Table of Contents');
  book.push('');

  for (const section of navigation) {
    book.push(`### ${section.title}`);
    for (const item of section.items) {
      const anchor = item.title.toLowerCase().replace(/[^a-z0-9]+/g, '-');
      book.push(`- [${item.title}](#${anchor})`);
    }
    book.push('');
  }

  book.push('---');
  book.push('');

  // Process each section
  for (const section of navigation) {
    book.push(`# ${section.title}`);
    book.push('');

    for (const item of section.items) {
      const filePath = hrefToFilePath(item.href);

      if (!fs.existsSync(filePath)) {
        console.warn(`Warning: File not found: ${filePath}`);
        continue;
      }

      console.log(`Processing: ${item.title} (${filePath})`);

      const svelteContent = fs.readFileSync(filePath, 'utf-8');
      const htmlContent = extractContent(svelteContent);
      const markdownContent = htmlToMarkdown(htmlContent);

      // The page already has its own h1, so we don't need to add another
      // But we do need to adjust heading levels if needed
      book.push(markdownContent);
      book.push('');
      book.push('---');
      book.push('');
    }
  }

  // Write the book
  const bookContent = book.join('\n');
  fs.writeFileSync(outputPath, bookContent);

  console.log(`\nBook generated: ${outputPath}`);
  console.log(`Total size: ${(bookContent.length / 1024).toFixed(1)} KB`);
}

// Run
buildBook();
