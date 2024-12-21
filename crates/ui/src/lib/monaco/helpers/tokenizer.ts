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
      value: string;
      line: number;
      column: number;
    }
  | {
      type: "number";
      value: string;
      line: number;
      column: number;
    }
  | {
      type: "string";
      value: string;
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
      value: string;
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
    const lines = text.split("\n");
    let inMultilineComment = false;

    lines.forEach((originalLine, lineIndex) => {
      // Handle empty or whitespace-only lines
      let line = originalLine.trim();
      if (line === "") return;

      let column = 0;

      // Handle multiline comment state
      if (inMultilineComment) {
        const endCommentIndex = line.indexOf("*/");
        if (endCommentIndex !== -1) {
          this.tokens.push({
            type: "comment",
            value: line.slice(0, endCommentIndex + 2),
            line: lineIndex + 1,
            column: 1,
          });
          line = line.slice(endCommentIndex + 2).trim();
          column = 0;
          inMultilineComment = false;
        } else {
          this.tokens.push({
            type: "comment",
            value: line,
            line: lineIndex + 1,
            column: 1,
          });
          return;
        }
      }

      while (column < line.length) {
        // Check for single-line comment
        if (line.slice(column).startsWith("//")) {
          this.tokens.push({
            type: "comment",
            value: line.slice(column),
            line: lineIndex + 1,
            column: column + 1,
          });
          break;
        }

        // Check for multiline comment start
        if (line.slice(column).startsWith("/*")) {
          const endCommentIndex = line.indexOf("*/", column);
          if (endCommentIndex !== -1) {
            // Complete multiline comment on the same line
            this.tokens.push({
              type: "comment",
              value: line.slice(column, endCommentIndex + 2),
              line: lineIndex + 1,
              column: column + 1,
            });
            line = line.slice(endCommentIndex + 2).trim();
            column = 0;
          } else {
            // Multiline comment continues on next lines
            this.tokens.push({
              type: "comment",
              value: line.slice(column),
              line: lineIndex + 1,
              column: column + 1,
            });
            inMultilineComment = true;
            break;
          }
        }

        // Match other token types
        const patterns = [
          {
            type: "keyword",
            regex:
              /\b(constante|declare|funcao|se|senao|para|enquanto|retorne|importe)\b/,
          },
          { type: "identifier", regex: /[\p{L}_][\p{L}0-9_]*/u },
          { type: "number", regex: /\d+(\.\d+)?/ },
          { type: "string", regex: /"(?:[^"\\]|\\.)*"/ },
          { type: "operator", regex: /[+\-*/=<>!]+/ },
          { type: "delimiter", regex: /[{}()\[\],;]/ },
        ];

        let matched = false;
        for (const { type, regex } of patterns) {
          const match = regex.exec(line.slice(column));
          if (match && match.index === 0) {
            this.tokens.push({
              type,
              value: match[0],
              line: lineIndex + 1,
              column: originalLine.indexOf(match[0]) + 1,
            } as Token);
            column += match[0].length;
            matched = true;
            break;
          }
        }

        if (!matched) {
          // Skip any remaining whitespace or unrecognized characters
          column++;
        }
      }
    });
  }

  // Existing traversal methods remain the same
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
