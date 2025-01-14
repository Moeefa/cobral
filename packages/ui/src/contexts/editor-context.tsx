import type * as monaco from "monaco-editor-core";

import { createContext, useCallback, useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";

import type { TreeViewElement } from "@/components/ui/file-tree";
import { type Entry, useLogManager } from "@/hooks/use-logmanager";

// Types
export interface EditorFile extends TreeViewElement {
	path?: string;
	content?: string;
	isDirectory?: boolean;
	children?: EditorFile[];
}

interface EditorContextState {
	editor: React.MutableRefObject<
		monaco.editor.IStandaloneCodeEditor | undefined
	>;
	value: string;
	theme: string;
	logs: React.JSX.Element[];
	files: EditorFile[];
	absolutePath: string | null;
	currentFile: EditorFile | null;
}

interface EditorContextActions {
	setCurrentFile: (file: EditorFile | null) => void;
	setAbsolutePath: (path: string | null) => void;
	setFiles: (files: EditorFile[]) => void;
	setValue: (value: string) => void;
	setTheme: (theme: string) => void;
	addLog: (logs: Entry[]) => void;
	addInput: (message: string) => void;
	clearLogs: () => void;
}

export type EditorContextValue = EditorContextState & EditorContextActions;

// Initial state
const initialState: EditorContextState = {
	editor: { current: undefined },
	value: "",
	theme: "dark",
	logs: [],
	files: [],
	absolutePath: null,
	currentFile: null,
};

// Context
export const EditorContext = createContext<EditorContextValue>(
	{} as EditorContextValue,
);

// Provider Component
export function EditorProvider({ children }: { children: React.ReactNode }) {
	const editor = useRef<monaco.editor.IStandaloneCodeEditor>();
	const [state, setState] = useState<EditorContextState>({
		...initialState,
		editor,
	});
	const { logs, addLog, addInput, clearLogs } = useLogManager();

	// State update functions
	const setCurrentFile = useCallback((file: EditorFile | null) => {
		setState((prev) => ({ ...prev, currentFile: file }));
	}, []);

	const setAbsolutePath = useCallback((path: string | null) => {
		setState((prev) => ({ ...prev, absolutePath: path }));
	}, []);

	const setFiles = useCallback((files: EditorFile[]) => {
		setState((prev) => ({ ...prev, files }));
	}, []);

	const setValue = useCallback((value: string) => {
		setState((prev) => ({ ...prev, value }));
	}, []);

	const setTheme = useCallback((theme: string) => {
		setState((prev) => ({ ...prev, theme }));
	}, []);

	// Event listeners
	useEffect(() => {
		const clearListener = listen("clear", clearLogs);
		const readListener = listen<string>("read", (event) =>
			addInput(event.payload),
		);
		const logListener = listen<Entry[]>("log_batch", (event) =>
			addLog(event.payload),
		);

		return () => {
			clearListener.then((unlisten) => unlisten());
			readListener.then((unlisten) => unlisten());
			logListener.then((unlisten) => unlisten());
		};
	}, [addLog, addInput, clearLogs]);

	const contextValue: EditorContextValue = {
		...state,
		logs,
		setCurrentFile,
		setAbsolutePath,
		setFiles,
		setValue,
		setTheme,
		addLog: (logs) =>
			addLog(
				logs.map((log) => ({
					...log,
					timestamp: new Date().toLocaleTimeString("pt-BR", {
						hour: "2-digit",
						minute: "2-digit",
						second: "2-digit",
					}),
				})),
			),
		addInput,
		clearLogs,
	};

	return (
		<EditorContext.Provider value={contextValue}>
			{children}
		</EditorContext.Provider>
	);
}
