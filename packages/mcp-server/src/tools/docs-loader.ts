/**
 * @module docs-loader
 *
 * Documentation Loading and Search Utilities
 *
 * This module provides functionality for loading, searching, and retrieving
 * Macroforge documentation sections. Documentation is stored as markdown files
 * with metadata in `docs/sections.json`.
 *
 * Key features:
 * - Loads documentation sections from disk with lazy content loading
 * - Supports multiple search strategies (exact, partial, fuzzy, category)
 * - Handles chunked documentation for large sections
 * - Provides type-safe interfaces for documentation structure
 *
 * @example
 * ```typescript
 * import { loadSections, getSection, searchSections } from './docs-loader.js';
 *
 * const sections = loadSections();
 * const debug = getSection(sections, 'debug');
 * const matches = searchSections(sections, 'serialization');
 * ```
 */

import { readFileSync, existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

/** Current file path for ESM module resolution */
const __filename = fileURLToPath(import.meta.url);

/** Current directory path for ESM module resolution */
const __dirname = dirname(__filename);

/** Path to the docs directory containing sections.json and markdown files */
const docsDir = join(__dirname, '..', '..', 'docs');

/**
 * Represents a documentation section with metadata and optional content.
 *
 * Sections can be standalone or part of a chunked hierarchy for large documents.
 * Chunked sections have a parent entry with `is_chunked: true` and child entries
 * with `parent_id` pointing to the parent.
 *
 * @property id - Unique identifier for the section (e.g., "debug", "serde-validators")
 * @property title - Human-readable title displayed to users
 * @property category - Category slug for grouping (e.g., "macros", "guides")
 * @property category_title - Human-readable category name
 * @property path - Relative path to the markdown content file from docs directory
 * @property use_cases - Comma-separated keywords describing when this doc is useful
 * @property content - The actual markdown content (loaded lazily, undefined for chunked parents)
 * @property is_chunked - True if this section is split into multiple sub-chunks
 * @property parent_id - For sub-chunks, the ID of the parent section
 * @property chunk_ids - For chunked parents, ordered list of child chunk IDs
 */
export interface Section {
  id: string;
  title: string;
  category: string;
  category_title: string;
  path: string;
  use_cases: string;
  content?: string;
  is_chunked?: boolean;
  parent_id?: string;
  chunk_ids?: string[];
}

/**
 * Loads all documentation sections from the docs directory.
 *
 * This function reads `docs/sections.json` for metadata and loads the markdown
 * content for each section from disk. Chunked parent sections do not have their
 * content loaded directly - their content is accessed through child chunks.
 *
 * @returns Array of Section objects with content loaded. Returns empty array if
 *          sections.json is not found (with a warning logged to stderr).
 *
 * @example
 * ```typescript
 * const sections = loadSections();
 * console.log(`Loaded ${sections.length} sections`);
 * ```
 */
export function loadSections(): Section[] {
  const sectionsPath = join(docsDir, 'sections.json');

  if (!existsSync(sectionsPath)) {
    console.error('Warning: sections.json not found. Run "npm run build:docs" to generate documentation.');
    return [];
  }

  const sectionsData = JSON.parse(readFileSync(sectionsPath, 'utf-8')) as Section[];

  // Load content for each non-chunked section from its markdown file
  for (const section of sectionsData) {
    // Chunked parent sections don't have direct content - content is in child chunks
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
 * Finds a section by ID or title using progressive matching strategies.
 *
 * The function attempts to match in the following order (stopping at first match):
 * 1. Exact ID match (case-insensitive)
 * 2. Exact title match (case-insensitive)
 * 3. Partial ID match (query is substring of ID)
 * 4. Partial title match (query is substring of title)
 *
 * @param sections - Array of sections to search through
 * @param query - Search query (ID or title to find)
 * @returns The first matching Section, or undefined if no match found
 *
 * @example
 * ```typescript
 * const debug = getSection(sections, 'debug');        // Exact ID match
 * const serde = getSection(sections, 'Serialize');    // Exact title match
 * const partial = getSection(sections, 'valid');      // Partial match on "validators"
 * ```
 */
export function getSection(sections: Section[], query: string): Section | undefined {
  const normalizedQuery = query.toLowerCase().trim();

  // Priority 1: Exact ID match (most specific)
  let match = sections.find((s) => s.id.toLowerCase() === normalizedQuery);
  if (match) return match;

  // Priority 2: Exact title match
  match = sections.find((s) => s.title.toLowerCase() === normalizedQuery);
  if (match) return match;

  // Priority 3: Partial ID match (query is substring of ID)
  match = sections.find((s) => s.id.toLowerCase().includes(normalizedQuery));
  if (match) return match;

  // Priority 4: Partial title match (query is substring of title)
  match = sections.find((s) => s.title.toLowerCase().includes(normalizedQuery));
  if (match) return match;

  return undefined;
}

/**
 * Retrieves multiple sections by their IDs or titles.
 *
 * Uses {@link getSection} for each query, deduplicating results if the same
 * section matches multiple queries.
 *
 * @param sections - Array of sections to search through
 * @param queries - Array of search queries (IDs or titles)
 * @returns Array of unique matching Sections in the order they were found
 *
 * @example
 * ```typescript
 * const docs = getSections(sections, ['debug', 'serialize', 'clone']);
 * // Returns all three sections if found
 * ```
 */
export function getSections(sections: Section[], queries: string[]): Section[] {
  const results: Section[] = [];

  for (const query of queries) {
    const section = getSection(sections, query);
    // Deduplicate: only add if not already in results
    if (section && !results.includes(section)) {
      results.push(section);
    }
  }

  return results;
}

/**
 * Performs a fuzzy search across sections using a weighted scoring algorithm.
 *
 * The search splits the query into keywords and scores each section based on
 * where matches are found. Results are sorted by relevance score (highest first).
 *
 * ## Scoring Algorithm
 *
 * Each section is scored based on matches in different fields:
 * - **ID match** (+10): Full query found in section ID
 * - **Title match** (+10): Full query found in section title
 * - **Use cases match** (+5 per keyword): Each query keyword found in use_cases
 * - **Content match** (+1 per keyword): Each query keyword found in content
 *
 * The higher weights for ID/title ensure exact matches rank above content matches.
 * Use cases are weighted moderately as they indicate intended use.
 *
 * @param sections - Array of sections to search through
 * @param query - Search query (can contain multiple space-separated keywords)
 * @returns Array of matching Sections sorted by relevance score (highest first)
 *
 * @example
 * ```typescript
 * // Single keyword search
 * const results = searchSections(sections, 'validation');
 *
 * // Multi-keyword search (scores sections matching more keywords higher)
 * const results = searchSections(sections, 'email url length');
 * ```
 */
export function searchSections(sections: Section[], query: string): Section[] {
  const normalizedQuery = query.toLowerCase().trim();
  const keywords = normalizedQuery.split(/\s+/);

  // Score each section based on where query/keywords match
  const scored = sections.map((section) => {
    let score = 0;

    // High priority: ID contains full query (+10)
    if (section.id.toLowerCase().includes(normalizedQuery)) {
      score += 10;
    }

    // High priority: Title contains full query (+10)
    if (section.title.toLowerCase().includes(normalizedQuery)) {
      score += 10;
    }

    // Medium priority: Each keyword in use_cases (+5 each)
    const useCases = section.use_cases.toLowerCase();
    for (const keyword of keywords) {
      if (useCases.includes(keyword)) {
        score += 5;
      }
    }

    // Low priority: Each keyword in content (+1 each)
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

  // Return matching sections sorted by score (highest first)
  return scored
    .filter((s) => s.score > 0)
    .sort((a, b) => b.score - a.score)
    .map((s) => s.section);
}

/**
 * Filters sections by category slug or category title.
 *
 * Matches against both `category` (slug) and `category_title` (display name)
 * to support flexible lookups.
 *
 * @param sections - Array of sections to filter
 * @param category - Category slug or title to filter by (case-insensitive)
 * @returns Array of Sections belonging to the specified category
 *
 * @example
 * ```typescript
 * // Filter by category slug
 * const macros = getSectionsByCategory(sections, 'macros');
 *
 * // Filter by category title
 * const guides = getSectionsByCategory(sections, 'Getting Started');
 * ```
 */
export function getSectionsByCategory(sections: Section[], category: string): Section[] {
  const normalizedCategory = category.toLowerCase().trim();
  return sections.filter(
    (s) =>
      s.category.toLowerCase() === normalizedCategory ||
      s.category_title.toLowerCase() === normalizedCategory
  );
}
