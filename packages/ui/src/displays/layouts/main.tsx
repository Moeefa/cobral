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
import { resolveTheme } from "@/lib/utils";

export const Layout = () => {
  const { theme, editor } = useContext(EditorContext);

  useEffect(() => {
    editor.current?.updateOptions({
      theme: resolveTheme(theme),
    });

    resolveTheme(theme);
  }, [theme]);

  return (
    <div className="flex h-full main-wrapper !outline-none monaco-editor !bg-transparent">
      <ResizablePanelGroup direction="horizontal">
        <ResizablePanel defaultSize={18} minSize={18}>
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
    </div>
  );
};
