import "@packages/monaco/setup";
import "@packages/monaco/userWorker";

import * as monaco from "monaco-editor-core";

import { useContext, useEffect, useLayoutEffect, useRef } from "react";

import { EditorContext } from "@/contexts/editor-context";
import { completionItemProvider } from "@packages/monaco/completion";
import { linter } from "@packages/monaco/linter";
import { resolveTheme } from "@/lib/utils";

export function MonacoEditor() {
	const editorRef = useRef<monaco.editor.IStandaloneCodeEditor>();
	const completionItemProviderRef = useRef<monaco.IDisposable>();

	const { value, setValue, editor, theme } = useContext(EditorContext);

	const updateMarkers = async () => {
		const model = editorRef.current?.getModel();
		if (!model) return;

		const markers = await linter(model);
		monaco.editor.setModelMarkers(model, "cobral", markers);
		setValue(model.getValue() || "");
	};

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	useEffect(() => {
		editor.current?.updateOptions({
			theme: resolveTheme(theme),
		});

		resolveTheme(theme);
	}, [theme]);

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	useLayoutEffect(() => {
		completionItemProviderRef.current?.dispose();

		completionItemProviderRef.current = completionItemProvider;
		editorRef.current = monaco.editor.create(
			document.getElementById("container") as HTMLElement,
			{
				value: value,
				language: "cobral",
				theme: "vitesse-dark",

				showUnused: true,
				showDeprecated: true,
				showFoldingControls: "always",

				fixedOverflowWidgets: true,
				contextmenu: false,
				automaticLayout: true,
				smoothScrolling: true,
				cursorSmoothCaretAnimation: "on",

				autoClosingBrackets: "always",
				autoClosingQuotes: "always",
				autoClosingComments: "always",
				autoIndent: "full",

				fontFamily: "SF Pro Mono",
				tabSize: 4,
				fontSize: 16,
				lineHeight: 24,
				stickyScroll: {
					enabled: false,
				},

				quickSuggestions: {
					other: "inline",
					comments: true,
					strings: true,
				},

				minimap: {
					enabled: true,
					autohide: true,
					renderCharacters: false,
				},
			},
		);

		editor.current = editorRef.current;

		document.fonts.ready.then(() => {
			monaco.editor.remeasureFonts();
		});

		editorRef.current?.getModel()?.onDidChangeContent(() => {
			updateMarkers();
		});

		updateMarkers();

		return () => {
			editorRef.current?.dispose();
		};
	}, []);

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	useEffect(() => {
		updateMarkers();
		editorRef.current?.getModel()?.applyEdits([
			{
				range:
					editorRef.current.getModel()?.getFullModelRange() ||
					new monaco.Range(1, 1, 1, 1),
				text: value,
			},
		]);
	}, [value]);

	return <div id="container" className="h-full w-full" />;
}
