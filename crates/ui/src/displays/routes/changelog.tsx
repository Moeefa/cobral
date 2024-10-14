import { useEffect, useState } from "react";

import { ArrowLeftIcon } from "@radix-ui/react-icons";
import { Link } from "react-router-dom";
import changelog from "../../../../../CHANGELOG.md";
import { markdownToHTML } from "@/lib/utils";

export function Changelog() {
  const [markdown, setMarkdown] = useState<string | null>(null);

  const transform = async () => setMarkdown(await markdownToHTML(changelog));

  useEffect(() => {
    transform();
  });

  return (
    <>
      <Link className="px-5 pt-4 flex items-center gap-1.5 w-min" to="/">
        <ArrowLeftIcon /> Voltar
      </Link>
      <article
        className="p-5 prose dark:prose-invert sm:mb-0 mb-12"
        dangerouslySetInnerHTML={{ __html: markdown || "" }}
      ></article>
    </>
  );
}
