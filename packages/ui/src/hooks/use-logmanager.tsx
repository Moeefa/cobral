import { useCallback, useRef, useState } from "react";

import { InputEntry } from "@/components/input-entry";
import { LogEntry } from "@/components/log-entry";
import { emit } from "@tauri-apps/api/event";

export interface Entry {
	level: "info" | "warn" | "error";
	message: string;
}

export const useLogManager = () => {
	const [logs, setLogs] = useState<React.JSX.Element[]>([]);
	const logBufferRef = useRef<React.JSX.Element[]>([]);
	const updateRequestedRef = useRef(false);

	const processLogs = useCallback(() => {
		setLogs((prev) => [...prev, ...logBufferRef.current]);
		logBufferRef.current = [];
		updateRequestedRef.current = false;
	}, []);

	const scheduleLogUpdate = useCallback(() => {
		if (!updateRequestedRef.current) {
			updateRequestedRef.current = true;
			requestAnimationFrame(processLogs);
		}
	}, [processLogs]);

	const addLog = useCallback(
		(newLogs: Entry[]) => {
			const timestamp = new Date().toLocaleTimeString("pt-BR", {
				hour: "2-digit",
				minute: "2-digit",
				second: "2-digit",
			});

			const entries = newLogs.map((log) => (
				<LogEntry
					key={`${log.message}-${timestamp}`}
					message={log.message}
					level={log.level}
					timestamp={timestamp}
				/>
			));
			logBufferRef.current.push(...entries);
			scheduleLogUpdate();
		},
		[scheduleLogUpdate],
	);

	const addInput = useCallback(
		(message: string) => {
			const timestamp = new Date().toLocaleTimeString("pt-BR", {
				hour: "2-digit",
				minute: "2-digit",
				second: "2-digit",
			});

			const entry = (
				<InputEntry
					key={`${message}-${timestamp}`}
					message={message}
					timestamp={timestamp}
					onSubmit={async (value) => {
						await emit("read_input", value);
					}}
				/>
			);
			logBufferRef.current.push(entry);
			scheduleLogUpdate();
		},
		[scheduleLogUpdate],
	);

	const clearLogs = useCallback(() => {
		logBufferRef.current = [];
		updateRequestedRef.current = false;
		setLogs([]);
	}, []);

	return { logs, addLog, addInput, clearLogs };
};
