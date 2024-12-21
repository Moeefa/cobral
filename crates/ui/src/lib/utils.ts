import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import rehypePrettyCode from "rehype-pretty-code";
import rehypeStringify from "rehype-stringify";
import remarkParse from "remark-parse";
import remarkUnlink from "remark-unlink";
import remarkRehype from "remark-rehype";
import rehypeShiki from "@shikijs/rehype/core";
import remarkGfm from "remark-gfm";
import rehypeRaw from "rehype-raw";
import { unified } from "unified";
import { quietlight } from "@uiw/codemirror-theme-quietlight";
import { okaidia } from "@uiw/codemirror-theme-okaidia";
import { githubDark, githubLight } from "@uiw/codemirror-theme-github";
import { vscodeDark, vscodeLight } from "@uiw/codemirror-theme-vscode";
import { createHighlighterCore } from "shiki/core";
import cobral from "./monaco/cobral.json";

export function resolveTheme(theme: string) {
  switch (theme) {
    case "vscode-dark":
      document.body.classList.add("dark");
      return vscodeDark;
    case "vscode-light":
      document.body.classList.remove("dark");
      return vscodeLight;
    case "github-dark":
      document.body.classList.add("dark");
      return githubDark;
    case "github-light":
      document.body.classList.remove("dark");
      return githubLight;
    case "quietlight":
      document.body.classList.remove("dark");
      return quietlight;
    case "okaidia":
      document.body.classList.add("dark");
      return okaidia;
    default:
      return quietlight;
  }
}

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

const highlighter = await createHighlighterCore({
  langs: [import("./monaco/cobral.json"), import("shiki/langs/bash.mjs")],
  loadWasm: import("shiki/wasm"),
  themes: [
    import("shiki/themes/vitesse-light.mjs"),
    import("shiki/themes/vitesse-dark.mjs"),
  ],
});

export async function markdownToHTML(markdown: string) {
  const p = await unified()
    .use(remarkParse)
    .use(remarkUnlink)
    .use(remarkGfm)
    .use(remarkRehype, { allowDangerousHtml: true })
    .use(rehypeRaw)
    .use(rehypeShiki, highlighter, {
      // or `theme` for a single theme
      keepBackground: false,
      theme: "vitesse-dark",
    })
    .use(rehypeStringify)
    .process(markdown);

  return p.toString();
}
