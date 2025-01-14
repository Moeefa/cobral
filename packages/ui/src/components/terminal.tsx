import { ArchiveIcon, BanIcon, SparklesIcon } from "lucide-react";
import { emit, listen } from "@tauri-apps/api/event";
import { useContext, useEffect, useRef } from "react";

import { Button } from "@/components/ui/button";
import { EditorContext } from "@/contexts/editor-context";
import { invoke } from "@tauri-apps/api/core";

export const Terminal = () => {
	const { clearLogs, value } = useContext(EditorContext);

	const runButton = useRef<HTMLButtonElement>(null);
	const breakButton = useRef<HTMLButtonElement>(null);

	const handleRun = async () => {
		if (runButton.current) runButton.current.disabled = true;
		if (breakButton.current) breakButton.current.disabled = false;

		const inputs =
			document.querySelectorAll<HTMLInputElement>("input.log-input");
		for (const input of inputs) {
			input.disabled = false;
		}

		clearLogs();

		await emit("break_exec");

		await invoke("eval", { input: value });
	};

	const handleBreak = async () => {
		if (runButton.current) runButton.current.disabled = false;
		if (breakButton.current) breakButton.current.disabled = true;

		// Get all inputs and disable them
		const inputs =
			document.querySelectorAll<HTMLInputElement>("input.log-input");
		for (const input of inputs) {
			input.disabled = true;
		}

		await emit("break_exec");
	};

	useEffect(() => {
		if (runButton.current) runButton.current.disabled = false;
		if (breakButton.current) breakButton.current.disabled = true;

		const finish = listen("exec_finished", async () => {
			if (runButton.current) runButton.current.disabled = false;
			if (breakButton.current) breakButton.current.disabled = true;

			await emit("break_exec");
		});

		return () => {
			finish.then((f) => f());
		};
	}, []);

	return (
		<>
			<div className="flex justify-between gap-1 p-1 bg-[var(--vscode-editor-background)]">
				<div>
					<Button
						ref={runButton}
						variant="expandIcon"
						Icon={SparklesIcon}
						iconPlacement="left"
						onClick={handleRun}
					>
						Executar
					</Button>

					<Button
						ref={breakButton}
						variant="expandIcon"
						Icon={BanIcon}
						iconPlacement="left"
						onClick={handleBreak}
					>
						Interromper
					</Button>
				</div>

				<div>
					<Button
						variant="expandIcon"
						Icon={ArchiveIcon}
						iconPlacement="left"
						onClick={clearLogs}
					>
						Limpar
					</Button>
				</div>
			</div>
		</>
	);
};
