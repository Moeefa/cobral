// @ts-ignore

import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";

// @ts-ignore
self.MonacoEnvironment = {
	getWorker(_: unknown, _label: string) {
		return new editorWorker();
	},
};
