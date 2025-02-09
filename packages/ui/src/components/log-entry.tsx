import React from "react";

export const LogEntry = React.memo(
	({
		message,
		level,
		timestamp,
	}: {
		level: string;
		message: string;
		timestamp: string;
	}) => (
		<div className="flex items-start gap-4 group view-overlays !relative">
			<span className="group-hover:text-[var(--vscode-editorLineNumber-activeForeground)] text-muted-foreground select-none ![font-family:'SF_Pro_Mono',monospace]">
				{timestamp}
			</span>
			<p
				data-level={level}
				className="break-all relative w-full whitespace-pre-wrap ![font-family:'SF_Pro_Mono',monospace] log-entry dark:data-[level=error]:bg-red-500 data-[level=error]:bg-red-400"
			>
				<div className="current-line w-full invisible group-hover:visible -z-10" />
				{message}
			</p>
		</div>
	),
);
