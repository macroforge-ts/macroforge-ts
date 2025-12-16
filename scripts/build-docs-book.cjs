#!/usr/bin/env node

/**
 * Build a markdown book from the website documentation pages.
 *
 * Reads from prerendered HTML (website/build/prerendered/) to get
 * all dynamic content including API data loaded at build time.
 */

const { program } = require('commander');
const fs = require('fs');
const path = require('path');

const websiteRoot = path.join(__dirname, '..', 'website');
const prerenderedDir = path.join(websiteRoot, 'build', 'prerendered');
const defaultOutput = path.join(__dirname, '..', 'docs', 'BOOK.md');

program
	.name('build-docs-book')
	.description('Build a markdown book from website documentation pages')
	.argument('[output-path]', 'Output file path', defaultOutput);

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
  // /docs/getting-started -> website/build/prerendered/docs/getting-started.html
  return path.join(prerenderedDir, href + '.html');
}

function hrefToSourceMarkdownPath(href) {
  const routePath = href.replace('/docs/', 'docs/');
  return path.join(websiteRoot, 'src', 'routes', routePath, '+page.svx');
}

function stripMdsvexBoilerplate(markdown) {
  let md = markdown;
  md = md.replace(/^<!--[\s\S]*?-->\s*/m, '');
  md = md.replace(/<svelte:head>[\s\S]*?<\/svelte:head>\s*/g, '');
  return md.trim() + '\n';
}

function readMarkdownForHref(href) {
  const sourceMdPath = hrefToSourceMarkdownPath(href);
  if (!fs.existsSync(sourceMdPath)) {
    return null;
  }
  const md = fs.readFileSync(sourceMdPath, 'utf-8');
  return stripMdsvexBoilerplate(md);
}

/**
 * Extract content from prerendered HTML
 * Gets the main prose content from <div class="prose">...</div>
 */
function extractContent(htmlContent) {
  // Extract the prose content area
  const proseMatch = htmlContent.match(/<div class="prose">([\s\S]*?)<\/div>\s*<nav class="mt-12/);
  if (!proseMatch) {
    // Fallback: try to find article content
    const articleMatch = htmlContent.match(/<article[^>]*>([\s\S]*?)<\/article>/);
    if (articleMatch) {
      return articleMatch[1];
    }
    return '';
  }
  return proseMatch[1];
}

/**
 * Convert rendered HTML content to Markdown
 */
function htmlToMarkdown(html) {
  // First, normalize whitespace in the HTML (remove tabs and excess whitespace)
  let md = html.replace(/\t/g, '').replace(/\n\s*\n/g, '\n\n');

  // Remove Svelte hydration comments
  md = md.replace(/<!--\[[\s\S]*?-->/g, '');
  md = md.replace(/<!--\]-->/g, '');
  md = md.replace(/<!---->/g, '');

  // Handle rendered code blocks: <pre class="..."><code class="...">
  // Extract text content from spans inside code blocks
  md = md.replace(/<div class="relative group[^"]*"[^>]*>[\s\S]*?<pre[^>]*><code[^>]*>([\s\S]*?)<\/code><\/pre>[\s\S]*?<\/div>/gi,
    (_, codeContent) => {
      // Extract text from spans, preserving newlines
      let code = codeContent
        .replace(/<span[^>]*>/gi, '')
        .replace(/<\/span>/gi, '')
        .replace(/\n/g, '\n')
        .trim();
      // Decode HTML entities
      code = code.replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&').replace(/&quot;/g, '"').replace(/&#39;/g, "'");
      return '```\n' + code + '\n```';
    });

  // Handle rendered Alert components (Note, Warning, Info, etc.)
  // Matches the rendered structure with svg icon, type label, and content div
  md = md.replace(/<div class="rounded-lg border[^"]*"[^>]*>[\s\S]*?<span class="[^"]*font-medium[^"]*">([\w\s]+)<\/span>[\s\S]*?<div class="[^"]*text-muted-foreground[^"]*">([\s\S]*?)<\/div>\s*<\/div>/gi,
    (_, type, content) => {
      const typeClean = type.toLowerCase().trim();
      const prefix = typeClean === 'warning' ? '> **Warning:**' :
                     typeClean === 'note' ? '> **Note:**' :
                     typeClean === 'info' ? '> **Info:**' :
                     typeClean === 'danger' ? '> **Danger:**' : '>';
      // Clean up the content
      const cleanContent = content.replace(/<[^>]+>/g, '').replace(/\s+/g, ' ').trim();
      return `${prefix} ${cleanContent}`;
    });

  // Handle "Tip" style alerts (lightbulb icon) - convert to blockquote with title
  md = md.replace(/<div class="flex items-center gap-2"><svg[^>]*>[\s\S]*?<\/svg>\s*([\w\s-]+)\s+([\s\S]*?)<\/div>/gi,
    (_, title, content) => {
      const titleClean = title.trim();
      const contentClean = content.replace(/<[^>]+>/g, '').replace(/\s+/g, ' ').trim();
      // If the title looks like a common alert type, format as such
      const typeWords = ['info', 'note', 'warning', 'danger', 'tip', 'important'];
      if (typeWords.some(t => titleClean.toLowerCase().includes(t))) {
        return `> **${titleClean}:** ${contentClean}`;
      }
      // Otherwise treat as a titled tip
      return `> **${titleClean}** ${contentClean}`;
    });

  // Remove version badge paragraphs (redundant in docs)
  md = md.replace(/<p class="version-badge[^"]*"[^>]*>.*?<\/p>/gi, '');

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

  // Handle tables - convert to markdown tables
  md = md.replace(/<table[^>]*>([\s\S]*?)<\/table>/gi, (_, tableContent) => {
    const rows = [];

    // Extract header row
    const theadMatch = tableContent.match(/<thead[^>]*>([\s\S]*?)<\/thead>/i);
    if (theadMatch) {
      const headerCells = [];
      const thMatches = theadMatch[1].matchAll(/<th[^>]*>([\s\S]*?)<\/th>/gi);
      for (const m of thMatches) {
        headerCells.push(m[1].replace(/<[^>]+>/g, '').replace(/\s+/g, ' ').trim());
      }
      if (headerCells.length > 0) {
        rows.push('| ' + headerCells.join(' | ') + ' |');
        rows.push('| ' + headerCells.map(() => '---').join(' | ') + ' |');
      }
    }

    // Extract body rows
    const tbodyMatch = tableContent.match(/<tbody[^>]*>([\s\S]*?)<\/tbody>/i);
    if (tbodyMatch) {
      const trMatches = tbodyMatch[1].matchAll(/<tr[^>]*>([\s\S]*?)<\/tr>/gi);
      for (const tr of trMatches) {
        const cells = [];
        const tdMatches = tr[1].matchAll(/<td[^>]*>([\s\S]*?)<\/td>/gi);
        for (const td of tdMatches) {
          cells.push(td[1].replace(/<[^>]+>/g, '').replace(/\s+/g, ' ').trim());
        }
        if (cells.length > 0) {
          rows.push('| ' + cells.join(' | ') + ' |');
        }
      }
    }

    return rows.join('\n') + '\n';
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

  // Remove SVG elements
  md = md.replace(/<svg[^>]*>[\s\S]*?<\/svg>/gi, '');

  // Remove buttons
  md = md.replace(/<button[^>]*>[\s\S]*?<\/button>/gi, '');

  // Clean up interactive code example panels (Before/After sections)
  // Convert Before/After/Source labels to markdown headers
  md = md.replace(/<div class="w-3 h-3 rounded-full bg-warning">\s*Before[^<]*<\/div>/gi, '\n**Before:**\n');
  md = md.replace(/<div class="w-3 h-3 rounded-full bg-success">\s*After[^<]*<\/div>/gi, '\n**After:**\n');
  md = md.replace(/<div class="w-3 h-3 rounded-full bg-[^"]*">\s*Source[^<]*<\/div>/gi, '\n**Source:**\n');
  // Remove remaining indicator dots
  md = md.replace(/<div class="w-3 h-3 rounded-full bg-[^"]*">[^<]*<\/div>/gi, '');
  // Clean up the flex containers used for Before/After labels
  md = md.replace(/<div class="flex items-center gap-2 mb-3">/gi, '');
  md = md.replace(/<div class="flex items-center justify-between mb-3">/gi, '');
  md = md.replace(/<div class="flex items-center gap-2">/gi, '');
  // Remove separator lines
  md = md.replace(/<div class="w-px h-\d+ bg-border">\s*<\/div>/gi, '');
  // Clean up other UI elements
  md = md.replace(/<div class="px-3[^"]*font-mono">/gi, '`');
  md = md.replace(/<div class="font-semibold[^"]*">/gi, '');
  md = md.replace(/<div class="font-medium[^"]*">/gi, '');
  md = md.replace(/<div class="text-sm[^"]*">/gi, '');
  // Remove interactive UI elements
  md = md.replace(/<div class="border border-[^"]*"[^>]*>[^<]*<\/div>/gi, '');
  md = md.replace(/<div class="flex justify-center">[^<]*<\/div>/gi, '');
  md = md.replace(/<div class="w-full[^"]*"[^>]*>[^<]*<\/div>/gi, '');
  // Remove error message divs
  md = md.replace(/<p class="text-red[^"]*"[^>]*>[^<]*<\/p>/gi, '');

  // Handle divs (just extract content)
  md = md.replace(/<div[^>]*>([\s\S]*?)<\/div>/gi, '$1');

  // Remove stray closing tags
  md = md.replace(/<\/div>/gi, '');
  md = md.replace(/<\/span>/gi, '');
  md = md.replace(/<\/p>/gi, '');

  // Remove span elements with specific classes (UI labels) - handle cases where content may not have closing tag nearby
  md = md.replace(/<span class="text-sm font-medium text-muted-foreground">(Before[^`]*)/gi, '\n**Before:**\n');
  md = md.replace(/<span class="text-sm font-medium text-muted-foreground">(After[^`]*)/gi, '\n**After:**\n');
  md = md.replace(/<span class="text-sm font-medium text-muted-foreground">(Source[^`]*)/gi, '\n**Source:**\n');
  md = md.replace(/<span class="text-lg font-medium[^"]*">[^<]*<\/span>/gi, '');
  md = md.replace(/<span class="text-[^"]*font-medium[^"]*">([^<]+)/gi, '**$1**');
  md = md.replace(/<span class="version-badge[^"]*">[^<]*/gi, '');
  // Remove div elements with text-xs classes (secondary UI text)
  md = md.replace(/<div class="text-xs[^"]*">[^<]*/gi, '');

  // Handle spans (just extract content)
  md = md.replace(/<span[^>]*>([\s\S]*?)<\/span>/gi, '$1');

  // Final cleanup of empty/stray HTML elements
  md = md.replace(/<div>\s*<\/div>/gi, '');
  md = md.replace(/<div>\s*$/gm, '');
  md = md.replace(/<div[^>]*>\s*$/gm, '');
  md = md.replace(/<span[^>]*>\s*$/gm, '');
  md = md.replace(/<span[^>]*>\s*<\/span>/gi, '');
  // Remove any remaining opening tags without content
  md = md.replace(/<div>\s*\n/gi, '');
  md = md.replace(/<span>\s*\n/gi, '');
  // Remove remaining UI elements (stats, layout containers)
  // These elements may have content that spans until next major element
  md = md.replace(/<span class="stats[^"]*">[\d\s\w]*<\/span>/gi, '');
  md = md.replace(/<span class="stats[^"]*">[^*]*/gi, ''); // content until next asterisk
  md = md.replace(/<div class="flex justify-center">[\s\S]*?<\/div>/gi, '');
  md = md.replace(/<div class="flex flex-col[^"]*">[\s\S]*?<\/div>/gi, '');
  md = md.replace(/<div class="border[^"]*">[\s\S]*?<\/div>/gi, '');
  md = md.replace(/<div class="absolute[^"]*"[^>]*>[\s\S]*?<\/div>/gi, '');

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
function buildBook(outputPath) {

  // Check if prerendered directory exists
  if (!fs.existsSync(prerenderedDir)) {
    console.error(`Error: Prerendered directory not found: ${prerenderedDir}`);
    console.error('Run "npm run build" in the website directory first.');
    process.exit(1);
  }

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
      const markdownFromSource = readMarkdownForHref(item.href);
      let markdownContent = markdownFromSource;

      if (markdownContent === null) {
        const filePath = hrefToFilePath(item.href);

        if (!fs.existsSync(filePath)) {
          console.warn(`Warning: File not found: ${filePath}`);
          continue;
        }

        console.log(`Processing: ${item.title} (${filePath})`);

        const rawHtml = fs.readFileSync(filePath, 'utf-8');
        const htmlContent = extractContent(rawHtml);
        markdownContent = htmlToMarkdown(htmlContent);
      } else {
        console.log(`Processing: ${item.title} (${item.href}) [source markdown]`);
      }

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

program.action(buildBook);
program.parse();
