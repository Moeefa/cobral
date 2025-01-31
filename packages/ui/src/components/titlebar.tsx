import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuLabel,
	DropdownMenuPortal,
	DropdownMenuRadioGroup,
	DropdownMenuRadioItem,
	DropdownMenuSeparator,
	DropdownMenuShortcut,
	DropdownMenuSub,
	DropdownMenuSubContent,
	DropdownMenuSubTrigger,
	DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useContext, useEffect, useState } from "react";

import { Button } from "@/components/ui/button";
import { EditorContext } from "@/contexts/editor-context";
import { Label } from "@/components/ui/label";
import { Link } from "react-router-dom";
import { SettingsContext } from "@/contexts/settings-context";
import { SettingsIcon } from "lucide-react";
import { open } from "@tauri-apps/plugin-shell";

// Trick to render custom titlebar in Decorum plugin for Tauri
export const Titlebar = ({ children }: { children: React.ReactNode }) => {
	const isWeb = !("__TAURI_INTERNALS__" in window);
	if (isWeb)
		return (
			<>
				<div className="flex h-11 justify-between items-center w-full monaco-editor bg-[var(--vscode-editor-background)]">
					<Left />
					<Right />
				</div>
				<div className="w-full overflow-auto h-full">{children}</div>
			</>
		);

	const [running, setRunning] = useState(true);

	useEffect(() => {
		const element = document.querySelector<HTMLElement>(
			"div[data-tauri-decorum-tb]",
		) as HTMLElement;
		element.classList.add(
			"!outline-none",
			"monaco-editor",
			"!bg-[var(--vscode-editor-background)]",
			"!text-foreground",
			'![font-family:"SF_Pro",sans-serif]',
			"!text-base",
		);
		const root = document.getElementById("main-layout") as HTMLElement;

		root.insertBefore(element, root.firstChild);
	}, []);

	useEffect(() => {
		if (!running) return;

		const element = document.querySelector<HTMLElement>(
			"div[data-tauri-decorum-tb] div[data-tauri-drag-region]",
		);

		const btns =
			document.querySelectorAll<HTMLButtonElement>(".decorum-tb-btn");

		for (const btn of btns) {
			btn.className = "decorum-tb-btn";
		}

		if (element) {
			element.style.display = "flex";
			element.style.justifyContent = "justify-between";
			if (element.parentElement) {
				element.parentElement.style.position = "static";
			}

			// Render titlebar content
			const titlebar = document.querySelector("#titlebar") as HTMLElement;
			element.insertAdjacentElement("beforeend", titlebar);

			setRunning(false);
		}
	}, [running]);

	return (
		<>
			<div
				data-tauri-drag-region
				id="titlebar"
				className="flex h-11 !outline-none justify-between items-center w-full monaco-editor bg-[var(--vscode-editor-background)]"
			>
				<Left />
				<Right />
			</div>
			<div className="w-full overflow-auto h-full">{children}</div>
		</>
	);
};

const Left = () => {
	const { currentFile } = useContext(EditorContext);

	return (
		<div
			id="left"
			data-tauri-drag-region
			className="flex items-center h-full gap-2.5 py-1"
		>
			<h4
				data-tauri-drag-region
				className="text-sm font-medium select-none px-4 p-2.5"
			>
				{currentFile?.name || "arquivo.cl"}
			</h4>
		</div>
	);
};

const Right = () => {
	const { theme, setTheme } = useContext(EditorContext);
	const { fontSize, setFontSize } = useContext(SettingsContext);

	return (
		<div
			id="right"
			data-tauri-drag-region
			className="flex items-center h-full px-4"
		>
			<DropdownMenu>
				<DropdownMenuTrigger className="outline-none group size-6 flex items-center justify-center">
					<SettingsIcon className="group-hover:rotate-[55deg] duration-300 ease-in-out transition-transform size-[1.05rem]" />
				</DropdownMenuTrigger>
				<DropdownMenuContent
					collisionPadding={2}
					className="[&_*[role=menuitem]]:h-10"
				>
					<Link to="/docs">
						<DropdownMenuItem>Documentação</DropdownMenuItem>
					</Link>
					<Link to="/changelog">
						<DropdownMenuItem>Novidades</DropdownMenuItem>
					</Link>
					<DropdownMenuItem
						onClick={() => open("https://discord.gg/Gpy5kUTFay")}
					>
						Comunidade
						<DropdownMenuShortcut>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 127.14 96.36"
								height={16}
								width={16}
							>
								<title>Discord</title>
								<path
									fill="#5865f2"
									d="M107.7,8.07A105.15,105.15,0,0,0,81.47,0a72.06,72.06,0,0,0-3.36,6.83A97.68,97.68,0,0,0,49,6.83,72.37,72.37,0,0,0,45.64,0,105.89,105.89,0,0,0,19.39,8.09C2.79,32.65-1.71,56.6.54,80.21h0A105.73,105.73,0,0,0,32.71,96.36,77.7,77.7,0,0,0,39.6,85.25a68.42,68.42,0,0,1-10.85-5.18c.91-.66,1.8-1.34,2.66-2a75.57,75.57,0,0,0,64.32,0c.87.71,1.76,1.39,2.66,2a68.68,68.68,0,0,1-10.87,5.19,77,77,0,0,0,6.89,11.1A105.25,105.25,0,0,0,126.6,80.22h0C129.24,52.84,122.09,29.11,107.7,8.07ZM42.45,65.69C36.18,65.69,31,60,31,53s5-12.74,11.43-12.74S54,46,53.89,53,48.84,65.69,42.45,65.69Zm42.24,0C78.41,65.69,73.25,60,73.25,53s5-12.74,11.44-12.74S96.23,46,96.12,53,91.08,65.69,84.69,65.69Z"
								/>
							</svg>
						</DropdownMenuShortcut>
					</DropdownMenuItem>
					<DropdownMenuSeparator />
					<DropdownMenuLabel>Configurações</DropdownMenuLabel>
					<DropdownMenuItem onSelect={(e) => e.preventDefault()}>
						<div className="flex w-full items-center justify-between gap-2">
							Escala
							<div className="flex gap-1 p-0.5 items-center bg-background rounded-sm">
								<Button
									size="icon"
									className="size-7 rounded-sm"
									variant="ghost"
									onClick={() =>
										setFontSize(
											(prev) => +Number.parseFloat((prev - 0.01).toFixed(2)),
										)
									}
								>
									-
								</Button>
								<Label htmlFor="font-size">{Math.floor(fontSize * 100)}%</Label>
								<Button
									size="icon"
									className="size-7 rounded-sm"
									variant="ghost"
									onClick={() =>
										setFontSize(
											(prev) => +Number.parseFloat((prev + 0.01).toFixed(2)),
										)
									}
								>
									+
								</Button>
							</div>
						</div>
					</DropdownMenuItem>
					<DropdownMenuSub>
						<DropdownMenuSubTrigger>Temas</DropdownMenuSubTrigger>
						<DropdownMenuPortal>
							<DropdownMenuSubContent>
								<DropdownMenuRadioGroup value={theme} onValueChange={setTheme}>
									<DropdownMenuRadioItem value="dark">
										Escuro
									</DropdownMenuRadioItem>
									<DropdownMenuRadioItem value="light">
										Claro
									</DropdownMenuRadioItem>
								</DropdownMenuRadioGroup>
							</DropdownMenuSubContent>
						</DropdownMenuPortal>
					</DropdownMenuSub>
				</DropdownMenuContent>
			</DropdownMenu>
		</div>
	);
};
