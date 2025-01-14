import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import rehypeStringify from "rehype-stringify";
import remarkParse from "remark-parse";
import remarkUnlink from "remark-unlink";
import remarkRehype from "remark-rehype";
import rehypeShiki from "@shikijs/rehype/core";
import remarkGfm from "remark-gfm";
import rehypeRaw from "rehype-raw";
import { unified } from "unified";
import { createHighlighterCore } from "shiki/core";

export function resolveTheme(theme: string) {
	switch (theme) {
		case "dark":
			document.body.classList.add("dark");
			return "vitesse-dark";
		case "light":
			document.body.classList.remove("dark");
			return "vitesse-light";
		default:
			document.body.classList.add("dark");
			return "vitesse-dark";
	}
}

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export async function markdownToHTML(markdown: string) {
	const highlighter = await createHighlighterCore({
		langs: [
			import("@packages/monaco/cobral.json"),
			import("shiki/langs/bash.mjs"),
		],
		loadWasm: import("shiki/wasm"),
		themes: [
			import("shiki/themes/vitesse-light.mjs"),
			import("shiki/themes/vitesse-dark.mjs"),
		],
	});

	const p = await unified()
		.use(remarkParse)
		.use(remarkUnlink)
		.use(remarkGfm)
		.use(remarkRehype, { allowDangerousHtml: true })
		.use(rehypeRaw)
		// @ts-ignore
		.use(rehypeShiki, highlighter, {
			keepBackground: true,
			theme: "vitesse-dark",
		})
		.use(rehypeStringify)
		.process(markdown);

	return p.toString();
}
