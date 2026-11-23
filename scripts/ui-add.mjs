// Usage: npm run ui:add -- sheet textarea
// The shadcn CLI (radix-nova style) writes `import { cn } from "cn"` and installs an
// unrelated `cn` package, and it prompts before overwriting shared deps like button.tsx.
import { execSync } from "node:child_process";
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const components = process.argv.slice(2);
if (components.length === 0) {
  console.error("Usage: npm run ui:add -- <component> [...]");
  process.exit(1);
}

execSync(`npx shadcn@latest add ${components.join(" ")} --yes`, {
  input: "n\n".repeat(20),
  stdio: ["pipe", "inherit", "inherit"],
});

const uiDir = join("src", "components", "ui");
for (const file of readdirSync(uiDir)) {
  const path = join(uiDir, file);
  const source = readFileSync(path, "utf8");
  const fixed = source.replace('from "cn"', 'from "@/lib/utils"');
  if (fixed !== source) writeFileSync(path, fixed);
}

const pkg = JSON.parse(readFileSync("package.json", "utf8"));
if (pkg.dependencies?.cn) execSync("npm uninstall cn", { stdio: "inherit" });
