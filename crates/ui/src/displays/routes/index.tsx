import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";

import { Editor } from "@/components/editor";
import { Terminal } from "@/components/terminal";

export function Page() {
  return (
    <div className="h-full w-full flex flex-col">
      <ResizablePanelGroup direction="vertical">
        <ResizablePanel>
          <Editor />
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel minSize={12} className="bg-background/20">
          <Terminal />
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}
