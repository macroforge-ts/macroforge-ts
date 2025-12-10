import { readFileSync, existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const docsDir = join(__dirname, '..', '..', 'docs');

export interface Section {
  id: string;
  title: string;
  category: string;
  category_title: string;
  path: string;
  use_cases: string;
  content?: string;
  is_chunked?: boolean;      // true if this section has sub-chunks
  parent_id?: string;        // for sub-chunks, points to parent section id
  chunk_ids?: string[];      // list of chunk IDs if is_chunked is true
}

/**
 * Load all documentation sections from the docs directory
 */
export function loadSections(): Section[] {
  const sectionsPath = join(docsDir, 'sections.json');

  if (!existsSync(sectionsPath)) {
    console.error('Warning: sections.json not found. Run "npm run build:docs" to generate documentation.');
    return [];
  }

  const sectionsData = JSON.parse(readFileSync(sectionsPath, 'utf-8')) as Section[];

  // Load content for each section
  for (const section of sectionsData) {
    // For chunked parent sections, we don't load content - it will be handled specially
    if (section.is_chunked) {
      continue;
    }

    const contentPath = join(docsDir, section.path);
    if (existsSync(contentPath)) {
      section.content = readFileSync(contentPath, 'utf-8');
    } else {
      section.content = `Documentation file not found: ${section.path}`;
    }
  }

  return sectionsData;
}

/**
 * Get a section by ID or title (case-insensitive)
 */
export function getSection(sections: Section[], query: string): Section | undefined {
  const normalizedQuery = query.toLowerCase().trim();

  // Try exact ID match first
  let match = sections.find((s) => s.id.toLowerCase() === normalizedQuery);
  if (match) return match;

  // Try exact title match
  match = sections.find((s) => s.title.toLowerCase() === normalizedQuery);
  if (match) return match;

  // Try partial ID match
  match = sections.find((s) => s.id.toLowerCase().includes(normalizedQuery));
  if (match) return match;

  // Try partial title match
  match = sections.find((s) => s.title.toLowerCase().includes(normalizedQuery));
  if (match) return match;

  return undefined;
}

/**
 * Get multiple sections by IDs or titles
 */
export function getSections(sections: Section[], queries: string[]): Section[] {
  const results: Section[] = [];

  for (const query of queries) {
    const section = getSection(sections, query);
    if (section && !results.includes(section)) {
      results.push(section);
    }
  }

  return results;
}

/**
 * Search sections by use_cases or content
 */
export function searchSections(sections: Section[], query: string): Section[] {
  const normalizedQuery = query.toLowerCase().trim();
  const keywords = normalizedQuery.split(/\s+/);

  const scored = sections.map((section) => {
    let score = 0;

    // Check ID
    if (section.id.toLowerCase().includes(normalizedQuery)) {
      score += 10;
    }

    // Check title
    if (section.title.toLowerCase().includes(normalizedQuery)) {
      score += 10;
    }

    // Check use_cases
    const useCases = section.use_cases.toLowerCase();
    for (const keyword of keywords) {
      if (useCases.includes(keyword)) {
        score += 5;
      }
    }

    // Check content (lower weight)
    if (section.content) {
      const content = section.content.toLowerCase();
      for (const keyword of keywords) {
        if (content.includes(keyword)) {
          score += 1;
        }
      }
    }

    return { section, score };
  });

  return scored
    .filter((s) => s.score > 0)
    .sort((a, b) => b.score - a.score)
    .map((s) => s.section);
}

/**
 * Get all sections in a category
 */
export function getSectionsByCategory(sections: Section[], category: string): Section[] {
  const normalizedCategory = category.toLowerCase().trim();
  return sections.filter(
    (s) =>
      s.category.toLowerCase() === normalizedCategory ||
      s.category_title.toLowerCase() === normalizedCategory
  );
}
