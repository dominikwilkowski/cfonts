export type WidthDetection = () => number | undefined;

/**
 * The width detection the active entry point wired up
 *
 * Node wires window-size in node.ts; the browser entry wires nothing,
 * so the browser bundle never touches any terminal code path
 */
export const detection: { width: WidthDetection } = {
	width: () => undefined,
};
