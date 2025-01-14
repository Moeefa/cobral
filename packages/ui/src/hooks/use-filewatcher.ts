import { FileSystemUtils } from "@/components/sidebar";
import { EditorContext, type EditorFile } from "@/contexts/editor-context";
import { basename } from "@tauri-apps/api/path";
import {
	readDir,
	readFile,
	type UnwatchFn,
	watchImmediate,
} from "@tauri-apps/plugin-fs";
import { useContext, useEffect, useRef } from "react";

export const useFileWatcher = () => {
	const { setFiles, absolutePath, currentFile, setValue, setCurrentFile } =
		useContext(EditorContext);

	const lastKnownPath = useRef<string>();

	const updateCurrentFile = async (newFiles: EditorFile[]) => {
		if (!currentFile?.path) return;

		try {
			let updatedFile = await FileSystemUtils.findFile(
				newFiles,
				currentFile.path,
			);

			if (!updatedFile && lastKnownPath.current) {
				const potentialFiles: EditorFile[] = newFiles.reduce(
					(acc: EditorFile[], file) => {
						if (file.children) {
							return acc.concat(file.children);
						}
						acc.push(file);
						return acc;
					},
					[],
				);

				updatedFile = potentialFiles.find((file) => {
					const wasRenamed =
						file.path !== lastKnownPath.current &&
						file.content === currentFile.content;
					return wasRenamed;
				});

				if (updatedFile) {
					const newName = await basename(updatedFile.path || "");
					updatedFile = {
						...updatedFile,
						name: newName,
						id: updatedFile.path || updatedFile.id,
					};
				}
			}

			if (updatedFile) {
				lastKnownPath.current = updatedFile.path;
				setCurrentFile({
					...updatedFile,
					content: currentFile.content,
				});
				setValue(currentFile.content || "");
			} else if (!currentFile.content) {
				setValue("");
				setCurrentFile(null);
			}
		} catch (error) {
			console.error("Error updating current file:", error);
		}
	};

	const refreshFileTree = async () => {
		if (!absolutePath) return;

		try {
			const entries = await readDir(absolutePath);
			const newFiles = await FileSystemUtils.processEntriesRecursively(
				absolutePath,
				entries,
			);
			setFiles(newFiles);
			await updateCurrentFile(newFiles);
		} catch (error) {
			console.error("Error refreshing file tree:", error);
		}
	};

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	useEffect(() => {
		let unwatch: UnwatchFn | null = null;
		const setupWatcher = async () => {
			if (!absolutePath) return;

			try {
				unwatch = await watchImmediate(
					absolutePath,
					async (event) => {
						if (
							currentFile?.path &&
							event.paths.includes(currentFile.path) &&
							event.type === "any"
						) {
							try {
								const content = await readFile(currentFile.path);
								const decodedContent = new TextDecoder().decode(content);
								setCurrentFile({
									...currentFile,
									content: decodedContent,
								});
								setValue(decodedContent);
							} catch (error) {
								console.error("Error updating file content:", error);
							}
						}
						await refreshFileTree();
					},
					{ recursive: true },
				);

				if (currentFile?.path) {
					lastKnownPath.current = currentFile.path;
				}
			} catch (error) {
				console.error("Error setting up file watcher:", error);
			}
		};

		setupWatcher();

		return () => {
			unwatch?.();
		};
	}, [absolutePath, currentFile]);

	return { refreshFileTree };
};
