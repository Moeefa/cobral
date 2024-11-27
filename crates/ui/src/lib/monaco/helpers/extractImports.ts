import { extractSymbols } from "@/lib/monaco/helpers/extractSymbols";
import { readTextFile } from "@tauri-apps/plugin-fs";

export const extractImports = async (
  lines: string[]
): Promise<{
  [scope: string]: {
    variables: Set<string>;
    functions: Set<string>;
  };
}> => {
  const declarations: {
    [scope: string]: {
      variables: Set<string>;
      functions: Set<string>;
    };
  } = {
    global: { variables: new Set(), functions: new Set() },
  };

  for (const [index, line] of lines.entries()) {
    const importMatch = /importe\s+"([^"]+)"/g.exec(line);
    if (!importMatch) continue;

    const importPath = importMatch[1];
    try {
      const importedContent = await readTextFile(importPath);

      // Extract symbols from imported content
      const symbols = extractSymbols(importedContent);

      // Merge imported global variables and functions
      symbols["global"].variables.forEach((variable) =>
        declarations.global.variables.add(variable)
      );
      symbols["global"].functions.forEach((func) =>
        declarations.global.functions.add(func)
      );
    } catch (error) {
      console.error(`Error reading import at line ${index + 1}: ${error}`);
    }
  }

  return declarations;
};
