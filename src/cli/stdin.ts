/**
 * Stdin detection and reading.
 * Reads piped input when available, returns null for TTY.
 */

/**
 * Check if stdin has piped content (not a TTY).
 */
export function hasStdin(): boolean {
  try {
    return !process.stdin.isTTY;
  } catch {
    return false;
  }
}

/**
 * Read all of stdin asynchronously.
 * Returns the content as a string, or null if stdin is a TTY.
 */
export async function readStdin(): Promise<string | null> {
  if (!hasStdin()) return null;

  try {
    const chunks: Buffer[] = [];
    for await (const chunk of process.stdin) {
      chunks.push(Buffer.from(chunk));
    }
    const content = Buffer.concat(chunks).toString("utf-8");
    return content.length > 0 ? content : null;
  } catch {
    return null;
  }
}
