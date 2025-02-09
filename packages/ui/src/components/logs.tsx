import {
	AutoSizer,
	CellMeasurer,
	CellMeasurerCache,
	List,
	type ListRowProps,
} from "react-virtualized";
import React, { useCallback, useContext, useEffect, useRef } from "react";

import { EditorContext } from "@/contexts/editor-context";
import { listen } from "@tauri-apps/api/event";

const cache = new CellMeasurerCache({
	defaultHeight: 24,
	fixedWidth: true,
});

export const Logs = React.memo(() => {
	const { logs } = useContext(EditorContext);
	const listRef = useRef<List>(null);

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	useEffect(() => {
		cache.clearAll();
	}, [logs]);

	useEffect(() => {
		window.addEventListener("resize", () => {
			cache.clearAll();
		});

		console.log("sidebar", document.getElementById("sidebar"));

		const unlisten = listen("resize", () => {
			cache.clearAll();
		});

		return () => {
			window.removeEventListener("resize", () => {
				cache.clearAll();
			});

			unlisten.then((f) => f());
		};
	}, []);

	const rowRenderer = useCallback(
		({ index, key, parent, style }: ListRowProps) => {
			return (
				<CellMeasurer
					cache={cache}
					columnIndex={0}
					key={key}
					parent={parent}
					rowIndex={index}
				>
					{({ registerChild }) => (
						<div
							style={{
								...style,
								width: "100%",
							}}
							ref={registerChild}
							className="log-entry-container"
						>
							{logs[index]}
						</div>
					)}
				</CellMeasurer>
			);
		},
		[logs],
	);

	const scrollToBottom = useCallback(() => {
		const debounce = setTimeout(() => {
			listRef.current?.scrollToRow(logs.length - 1);
		}, 200);

		return () => clearTimeout(debounce);
	}, [logs]);

	useEffect(() => {
		scrollToBottom();
	}, [scrollToBottom]);

	return (
		<AutoSizer>
			{({ width, height }) => (
				<List
					ref={listRef}
					width={width}
					height={height - 48}
					rowCount={logs.length}
					rowHeight={cache.rowHeight}
					rowRenderer={rowRenderer}
					overscanRowCount={20}
					deferredMeasurementCache={cache}
					className="border-border monaco-editor pl-2 bg-[var(--vscode-editor-background)] ![font-family:'SF_Pro_Mono',monospace]"
				/>
			)}
		</AutoSizer>
	);
});
