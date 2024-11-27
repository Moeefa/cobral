import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";

import { CodemirrorEditor } from "@/components/editors/codemirror-editor";
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
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}
