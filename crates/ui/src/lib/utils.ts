import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import rehypePrettyCode from "rehype-pretty-code";
import rehypeStringify from "rehype-stringify";
import remarkParse from "remark-parse";
import remarkUnlink from "remark-unlink";
import remarkRehype from "remark-rehype";
import remarkGfm from "remark-gfm";
import { unified } from "unified";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export async function markdownToHTML(markdown: string) {
  const p = await unified()
    .use(remarkParse)
    .use(remarkUnlink)
    .use(remarkGfm)
    .use(remarkRehype)
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
