import React, { useCallback } from "react";

import { Input } from "@/components/ui/input";

export const InputEntry = React.memo(
	({
		message,
		timestamp,
		onSubmit,
	}: {
		message: string;
		timestamp: string;
		onSubmit: (value: string) => void;
	}) => {
		const handleKeyDown = useCallback(
			(e: React.KeyboardEvent<HTMLInputElement>) => {
				if (e.key !== "Enter") return;

				const inputElement = e.currentTarget;

				inputElement.disabled = true;

				onSubmit(inputElement.value);
			},
			[onSubmit],
		);

		return (
			<div
				className="flex items-start gap-4 group view-overlays !relative"
				data-key={`input-${message}`}
			>
				<span className="group-hover:text-[var(--vscode-editorLineNumber-activeForeground)] text-muted-foreground select-none ![font-family:'SF_Pro_Mono',monospace]">
					{timestamp}
				</span>
				<div className="w-full log-entry !relative">
					<span className="break-all whitespace-pre-wrap w-full [font-family:'SF_Pro_Mono',monospace] inline-block">
						<div className="current-line invisible group-hover:visible w-full h-full -z-10" />
						{message}
						<Input
							key={`input-${message}`}
							className="inline-block !w-max [font-family:'SF_Pro_Mono',monospace] log-input h-max rounded-none p-0 border-0 outline-none ring-0 focus-visible:ring-0 focus:ring-0 align-baseline"
							type="text"
							autoFocus
							spellCheck={false}
							onKeyDown={handleKeyDown}
						/>
					</span>
				</div>
			</div>
		);
	},
);
