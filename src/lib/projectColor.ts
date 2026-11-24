/** Project color: stored hue, lightness and chroma from the theme (`--project-l`, `--project-c`). */
export const projectColor = (hue: number) =>
  `oklch(var(--project-l) var(--project-c) ${hue})`;
