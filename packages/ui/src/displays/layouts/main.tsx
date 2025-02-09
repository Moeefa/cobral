import {
	ResizableHandle,
	ResizablePanel,
	ResizablePanelGroup,
} from "@/components/ui/resizable";
import { useContext, useEffect } from "react";

import { EditorContext } from "@/contexts/editor-context";
import { Outlet } from "react-router-dom";
import { Sidebar } from "@/components/sidebar";
import { Titlebar } from "@/components/titlebar";
import { Toaster } from "@/components/ui/toaster";
import { emit } from "@tauri-apps/api/event";
import { resolveTheme } from "@/lib/utils";

export const Layout = () => {
	const { theme, editor } = useContext(EditorContext);

	useEffect(() => {
		editor.current?.updateOptions({
			theme: resolveTheme(theme),
		});

		resolveTheme(theme);
	}, [theme, editor]);

	useEffect(() => {
		document.addEventListener("contextmenu", (e) => {
			e.preventDefault();
		});

		return () => {
			document.removeEventListener("contextmenu", (e) => {
				e.preventDefault();
			});
		};
	}, []);

	return (
		<div className="flex h-full main-wrapper !outline-none monaco-editor !bg-transparent">
			<ResizablePanelGroup direction="horizontal">
				<ResizablePanel
					defaultSize={16}
					minSize={16}
					onResize={async () => await emit("resize")}
				>
					<Sidebar />
				</ResizablePanel>
				<ResizableHandle />
				<ResizablePanel minSize={50}>
					<div
						className="flex flex-col h-full w-full bg-[var(--vscode-editor-background)]"
						id="main-layout"
					>
						{/* <Outlet /> */}
						<Titlebar>
							<Outlet />
						</Titlebar>
					</div>
				</ResizablePanel>
			</ResizablePanelGroup>
			<Toaster />
		</div>
	);
};
