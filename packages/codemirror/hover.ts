import { hoverTooltip } from "@codemirror/view";
import { markdownToHTML } from "@/lib/utils";
import { syntaxTree } from "@codemirror/language";
import { variableTypes } from "@/lib/codemirror/linter";

const functionDescriptions = new Map<string, string>([
	[
		"escrever",
		'Escreve um valor na saída padrão.\n\nExemplos:\n\n```escrever("Olá, mundo!")```',
	],
	[
		"ler",
		'Lê um valor da entrada padrão.\n\nExemplo:\n\n```declare x = ler("Digite um valor:")```',
	],
	[
		"int",
		"Converte um valor para inteiro.\n\nExemplo:\n\n```declare x = int(3.14)```",
	],
	[
		"real",
		"Converte um valor para real.\n\nExemplo:\n\n```declare x = real(42)```",
	],
]);

export const wordHover = hoverTooltip((view, pos, side) => {
	let { from, to, text } = view.state.doc.lineAt(pos);
	let start = pos,
		end = pos;

	// Find the word under the cursor
	while (start > from && /\w/.test(text[start - from - 1])) start--;
	while (end < to && /\w/.test(text[end - from])) end++;

	if ((start == pos && side < 0) || (end == pos && side > 0)) return null;

	const word = text.slice(start - from, end - from);

	// Analyze the context using the syntax tree
	const tree = syntaxTree(view.state);
	let typeInfo = "";

	tree.cursor().iterate((_node) => {
		// Variable hover: check if the word is a declared variable
		if (variableTypes.has(word)) {
			typeInfo = `Variável <span class="dark:text-blue-400 text-violet-500">${word}</span>: <span class="dark:text-rose-500 text-blue-500">${variableTypes.get(
				word,
			)}</span>`;
		}

		// Built-in function hover: check if the word matches a known function
		if (functionDescriptions.has(word)) {
			typeInfo = `${functionDescriptions.get(word)}`;
		}
	});

	// If no type info was found, show the word itself
	if (!typeInfo) return null;

	return {
		pos: start,
		end,
		above: true,
		create(_view) {
			let dom = document.createElement("div");
			dom.className = "cm-tooltip cm-tooltip-cursor";
			dom.style.paddingTop = "0.4rem";
			dom.style.paddingBottom = "0.4rem";
			dom.style.paddingLeft = "0.7rem";
			dom.style.paddingRight = "0.7rem";
			dom.style.backdropFilter = "blur(20px)";
			dom.style.zIndex = "1000";

			async function createHTML() {
				dom.innerHTML = await markdownToHTML(typeInfo);
			}

			createHTML();

			return { dom };
		},
	};
});
