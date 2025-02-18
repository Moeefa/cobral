import {
	ResizableHandle,
	ResizablePanel,
	ResizablePanelGroup,
} from "@/components/ui/resizable";

import { Logs } from "@/components/logs";
import { MonacoEditor } from "@/components/editors/monaco-editor";
import { Terminal } from "@/components/terminal";

export function Page() {
	return (
		<div className="h-full w-full flex flex-col">
			<ResizablePanelGroup direction="vertical">
				<ResizablePanel>
					<MonacoEditor />
				</ResizablePanel>
				<ResizableHandle withHandle />
				<ResizablePanel minSize={12} className="bg-background/20">
					<Terminal />
					<Logs />
				</ResizablePanel>
			</ResizablePanelGroup>
		</div>
	);
}
