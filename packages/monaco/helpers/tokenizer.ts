export type Token =
	| {
			type: "keyword";
			value:
				| "nao"
				| "verdadeiro"
				| "falso"
				| "constante"
				| "declare"
				| "funcao"
				| "se"
				| "senao"
				| "para"
				| "pare"
				| "enquanto"
				| "retorne"
				| "importe"
				| "escolha"
				| "caso"
				| "padrao"
				| "e"
				| "ou";
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
	private position = 0;

	constructor(text: string) {
		this.tokenize(text);
	}

	private tokenize(text: string) {
		const lines = text.split("\n");
		let inMultilineComment = false;

		lines.forEach((originalLine, lineIndex) => {
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
				// Skip whitespace
				const whitespaceMatch = /^\s+/.exec(line.slice(column));
				if (whitespaceMatch) {
					column += whitespaceMatch[0].length;
					continue;
				}

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
						this.tokens.push({
							type: "comment",
							value: line.slice(column, endCommentIndex + 2),
							line: lineIndex + 1,
							column: column + 1,
						});
						line = line.slice(endCommentIndex + 2).trim();
						column = 0;
					} else {
						this.tokens.push({
							type: "comment",
							value: line.slice(column),
							line: lineIndex + 1,
							column: column + 1,
						});
						inMultilineComment = true;
						break;
					}
					continue;
				}

				let matched = false;

				// Try to match each pattern
				const patterns = [
					{
						type: "keyword",
						regex:
							/^(nao|verdadeiro|falso|constante|pare|declare|funcao|se|senao|para|enquanto|retorne|importe|escolha|caso|padrao|e|ou)\b/,
					},
					{ type: "number", regex: /^\d+(\.\d+)?/ },
					{ type: "string", regex: /^"(?:[^"\\]|\\.)*"/ },
					{ type: "operator", regex: /^([+\-*/=<>!]=?|[<>])/ },
					{ type: "delimiter", regex: /^[{}()\[\],;]/ },
					{ type: "identifier", regex: /^[\p{L}_][\p{L}0-9_]*\b/u },
				];

				for (const { type, regex } of patterns) {
					const match = regex.exec(line.slice(column));
					if (match) {
						const value = match[0];
						const actualColumn = originalLine.indexOf(value, column) + 1;

						this.tokens.push({
							type,
							value,
							line: lineIndex + 1,
							column: actualColumn,
						} as Token);

						column += value.length;
						matched = true;
						break;
					}
				}

				if (!matched) {
					column++;
				}
			}
		});
	}

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
