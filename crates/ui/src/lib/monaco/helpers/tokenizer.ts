export type Token =
  | {
      type: "keyword";
      value:
        | "constante"
        | "declare"
        | "funcao"
        | "se"
        | "senao"
        | "para"
        | "enquanto"
        | "retorne"
        | "importe";
      line: number;
      column: number;
    }
  | {
      type: "identifier";
      value: string; // Any valid identifier
      line: number;
      column: number;
    }
  | {
      type: "number";
      value: string; // Numeric value as string (e.g., "42", "3.14")
      line: number;
      column: number;
    }
  | {
      type: "string";
      value: string; // Any string literal (e.g., '"hello"')
      line: number;
      column: number;
    }
  | {
      type: "operator";
      value:
        | "+"
        | "-"
        | "*"
        | "/"
        | "="
        | "<"
        | ">"
        | "=="
        | "!="
        | "<="
        | ">=";
      line: number;
      column: number;
    }
  | {
      type: "delimiter";
      value: "{" | "}" | "(" | ")" | "[" | "]" | "," | ";";
      line: number;
      column: number;
    }
  | {
      type: "comment";
      value: string; // Comment content (e.g., "// comment" or "/* comment */")
      line: number;
      column: number;
    };

export class Tokenizer {
  private tokens: Token[] = [];
  private position: number = 0;

  constructor(text: string) {
    this.tokenize(text);
  }

  private tokenize(text: string) {
    const stripped = text.replace(/\/\*[\s\S]*?\*\/|\/\/.*/g, ""); // Remove comments
    const lines = stripped.split("\n");
    const patterns: { type: string; regex: RegExp }[] = [
      {
        type: "keyword",
        regex:
          /\b(constante|declare|funcao|se|senao|para|enquanto|retorne|importe)\b/g,
      },
      { type: "identifier", regex: /[\p{L}_][\p{L}0-9_]*/gu },
      { type: "number", regex: /\b\d+(\.\d+)?\b/g },
      { type: "string", regex: /"(?:[^"\\]|\\.)*"/g },
      { type: "operator", regex: /[+\-*/=<>!]+/g },
      { type: "delimiter", regex: /[{}()\[\],;]/g },
      { type: "comment", regex: /\/\/.*|\/\*[\s\S]*?\*\//g },
    ];

    lines.forEach((line, lineIndex) => {
      let column = 0;

      while (column < line.length) {
        let matched = false;

        for (const { type, regex } of patterns) {
          regex.lastIndex = column; // Set regex starting position
          const match = regex.exec(line);

          if (match && match.index === column) {
            this.tokens.push({
              type,
              value: match[0],
              line: lineIndex + 1,
              column: column + 1,
            } as Token);

            column += match[0].length;
            matched = true;
            break;
          }
        }

        if (!matched) {
          if (line[column].trim()) {
            throw new Error(
              `Unexpected character '${line[column]}' at line ${
                lineIndex + 1
              }, column ${column + 1}`
            );
          }
          column++; // Skip whitespace
        }
      }
    });
  }

  // Traversal methods
  next(): Token | null {
    return this.position < this.tokens.length
      ? this.tokens[this.position++]
      : null;
  }

  peek(): Token | null {
    return this.position < this.tokens.length
      ? this.tokens[this.position]
      : null;
  }

  rewind(): Token | null {
    return this.position > 0 ? this.tokens[--this.position] : null;
  }

  reset(): void {
    this.position = 0;
  }
}
