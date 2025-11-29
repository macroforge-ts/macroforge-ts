/**
 * Test utilities for position mapping tests.
 * Loads fixtures and calculates source mappings automatically.
 */

const fs = require('fs');
const path = require('path');

/**
 * Load a fixture pair (original.test.ts and expanded.test.ts) and calculate source mapping.
 *
 * @param {string} fixtureName - Name of the fixture directory under tests/fixtures/
 * @returns {{ original: string, expanded: string, sourceMapping: object, insertionPoint: number }}
 */
function loadFixture(fixtureName) {
  const fixtureDir = path.join(__dirname, 'fixtures', fixtureName);
  const original = fs.readFileSync(path.join(fixtureDir, 'original.test.ts'), 'utf-8');
  const expanded = fs.readFileSync(path.join(fixtureDir, 'expanded.test.ts'), 'utf-8');

  // Calculate the insertion point by finding where the files diverge
  const { insertionPoint, insertionLength } = findInsertion(original, expanded);

  // Build source mapping
  const sourceMapping = {
    segments: [
      {
        originalStart: 0,
        originalEnd: insertionPoint,
        expandedStart: 0,
        expandedEnd: insertionPoint
      },
      {
        originalStart: insertionPoint,
        originalEnd: original.length,
        expandedStart: insertionPoint + insertionLength,
        expandedEnd: expanded.length
      }
    ],
    generatedRegions: [
      {
        start: insertionPoint,
        end: insertionPoint + insertionLength,
        sourceMacro: 'Debug'
      }
    ]
  };

  return {
    original,
    expanded,
    sourceMapping,
    insertionPoint,
    insertionLength
  };
}

/**
 * Find where content was inserted by comparing original and expanded.
 * Assumes a single contiguous insertion.
 */
function findInsertion(original, expanded) {
  // Find the first character where they differ
  let insertionPoint = 0;
  const minLen = Math.min(original.length, expanded.length);

  while (insertionPoint < minLen && original[insertionPoint] === expanded[insertionPoint]) {
    insertionPoint++;
  }

  // The insertion length is the difference in file lengths
  const insertionLength = expanded.length - original.length;

  // Verify the rest matches after the insertion
  const afterOriginal = original.slice(insertionPoint);
  const afterExpanded = expanded.slice(insertionPoint + insertionLength);

  if (afterOriginal !== afterExpanded) {
    // Files don't match after insertion - might be replacement instead
    // Try to find matching suffix
    let suffixLen = 0;
    while (suffixLen < minLen &&
           original[original.length - 1 - suffixLen] === expanded[expanded.length - 1 - suffixLen]) {
      suffixLen++;
    }

    // Recalculate insertion point based on matching suffix
    const originalEndOfChange = original.length - suffixLen;
    const expandedEndOfChange = expanded.length - suffixLen;

    return {
      insertionPoint: insertionPoint,
      insertionLength: expandedEndOfChange - originalEndOfChange + (originalEndOfChange - insertionPoint)
    };
  }

  return { insertionPoint, insertionLength };
}

/**
 * Find a marker string in source and return its byte offset.
 */
function findMarker(source, marker) {
  const index = source.indexOf(marker);
  if (index === -1) {
    throw new Error(`Marker "${marker}" not found in source`);
  }
  return index;
}

/**
 * Convert byte offset to line:column.
 */
function offsetToLineColumn(source, offset) {
  const lines = source.split('\n');
  let currentOffset = 0;

  for (let line = 0; line < lines.length; line++) {
    const lineLength = lines[line].length + 1; // +1 for newline
    if (currentOffset + lineLength > offset) {
      return { line: line + 1, column: offset - currentOffset + 1 };
    }
    currentOffset += lineLength;
  }

  return { line: lines.length, column: 1 };
}

/**
 * Find the line number containing a marker.
 */
function findLineNumber(source, marker) {
  const offset = findMarker(source, marker);
  return offsetToLineColumn(source, offset).line;
}

module.exports = {
  loadFixture,
  findInsertion,
  findMarker,
  offsetToLineColumn,
  findLineNumber
};
