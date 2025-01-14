import { useEffect, useState } from "react";

import { ArrowLeftIcon } from "@radix-ui/react-icons";
import { Link } from "react-router-dom";
import { markdownToHTML } from "@/lib/utils";
import readme from "../../../../../README.md";

export function Docs() {
	const [markdown, setMarkdown] = useState<string | null>(null);

	const transform = async () => setMarkdown(await markdownToHTML(readme));

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
				// biome-ignore lint/security/noDangerouslySetInnerHtml: <explanation>
				dangerouslySetInnerHTML={{ __html: markdown || "" }}
			/>
		</>
	);
}
