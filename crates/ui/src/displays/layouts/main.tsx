import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";

import { CommandMenu } from "@/components/command";
import { Outlet } from "react-router-dom";
import { Sidebar } from "@/components/sidebar";
import { Titlebar } from "@/components/titlebar";

export const Layout = () => {
  return (
    <div className="flex h-full">
      <CommandMenu />
      <ResizablePanelGroup direction="horizontal">
        <ResizablePanel defaultSize={18} minSize={18}>
          <Sidebar />
        </ResizablePanel>
        <ResizableHandle />
        <ResizablePanel minSize={50}>
          <div
            className="flex flex-col h-full w-full bg-background"
            id="main-layout"
          >
            <Titlebar>
              <Outlet />
            </Titlebar>
          </div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
};
