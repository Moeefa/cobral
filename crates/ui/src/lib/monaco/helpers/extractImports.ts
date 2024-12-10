import { Token, Tokenizer } from "@/lib/monaco/helpers/tokenizer";

import { extractSymbols } from "@/lib/monaco/helpers/extractSymbols";
import { readTextFile } from "@tauri-apps/plugin-fs";

export const extractImports = async (
  text: string
): Promise<{
  [scope: string]: {
    variables: Set<string>;
    functions: Set<string>;
  };
}> => {
  const tokenizer = new Tokenizer(text);
  const declarations: {
    [scope: string]: {
      variables: Set<string>;
      functions: Set<string>;
    };
  } = {
    global: { variables: new Set(), functions: new Set() },
  };

  let token: Token | null;
  while ((token = tokenizer.next())) {
    if (token.type === "keyword" && token.value === "importe") {
      // Peek to get the string literal for the import path
      const nextToken = tokenizer.next();
      if (nextToken?.type === "string") {
        const importPath = nextToken.value.slice(1, -1); // Remove quotes

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
          console.error(`Error reading import at line ${token.line}: ${error}`);
        }
      }
    }
  }

  return declarations;
};
