import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import rehypePrettyCode from "rehype-pretty-code";
import rehypeStringify from "rehype-stringify";
import remarkParse from "remark-parse";
import remarkUnlink from "remark-unlink";
import remarkRehype from "remark-rehype";
import remarkGfm from "remark-gfm";
import rehypeRaw from "rehype-raw";
import { unified } from "unified";
import { quietlight } from "@uiw/codemirror-theme-quietlight";
import { okaidia } from "@uiw/codemirror-theme-okaidia";
import { githubDark, githubLight } from "@uiw/codemirror-theme-github";
import { vscodeDark, vscodeLight } from "@uiw/codemirror-theme-vscode";

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

export async function markdownToHTML(markdown: string) {
  const p = await unified()
    .use(remarkParse)
    .use(remarkUnlink)
    .use(remarkGfm)
    .use(remarkRehype, { allowDangerousHtml: true })
    .use(rehypeRaw)
    .use(rehypePrettyCode, {
      theme: {
        light: "min-light",
        dark: "min-dark",
      },
      keepBackground: false,
    })
    .use(rehypeStringify)
    .process(markdown);

  return p.toString();
}
