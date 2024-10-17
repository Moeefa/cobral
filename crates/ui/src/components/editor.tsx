import {
  ArrowLeftIcon,
  CheckIcon,
  CrumpledPaperIcon,
} from "@radix-ui/react-icons";
import CodeMirror, {
  EditorView,
  ViewUpdate,
  keymap,
} from "@uiw/react-codemirror";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { useCallback, useContext, useEffect, useState } from "react";

import { Button } from "@/components/ui/button";
import { EditorContext } from "@/contexts/editor-context";
import React from "react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { cobral } from "@/linter/cobral";
import { cobralLinter } from "@/linter/linter";
import { indentationMarkers } from "@replit/codemirror-indentation-markers";
import { invoke } from "@tauri-apps/api/core";
import { okaidia } from "@uiw/codemirror-theme-okaidia";
import { quietlight } from "@uiw/codemirror-theme-quietlight";
import { showMinimap } from "@replit/codemirror-minimap";
import { vscodeKeymap } from "@replit/codemirror-vscode-keymap";
import { wordHover } from "@/linter/hover";

const CodeMirrorEditor = () => {
  const { clearLogs, logs, value, setValue } = useContext(EditorContext);
  const [theme, setTheme] = useState(quietlight);

  const onChange = useCallback((val: string, _viewUpdate: ViewUpdate) => {
    setValue(val);
    invoke("update", { input: val });
  }, []);

  const handleParse = async () => {
    clearLogs();
    await invoke("parse", { input: value });
  };

  const handleParseStep = async () => {
    const payload = await invoke<{ current: number; length: number }>("step", {
      input: value,
    });

    if (payload.length === 0) {
      invoke("update", { input: value });
      clearLogs();
    }
  };

  useEffect(() => {
    setTheme(
      window.matchMedia("(prefers-color-scheme: dark)").matches
        ? okaidia
        : quietlight
    );

    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", (event) => {
        event.matches ? setTheme(okaidia) : setTheme(quietlight);
      });
  }, []);

  return (
    <div className="h-full w-full flex flex-col">
      <ResizablePanelGroup direction="vertical">
        <ResizablePanel>
          <CodeMirror
            value={value}
            height="100%"
            width="100%"
            className="h-full w-full font-mono"
            theme={theme}
            extensions={[
              cobral(),
              keymap.of(vscodeKeymap),
              indentationMarkers(),
              showMinimap.compute(["doc"], (_state) => {
                return {
                  create: (_v: EditorView) => {
                    const dom = document.createElement("div");
                    return { dom };
                  },
                  displayText: "blocks",
                  showOverlay: "always",
                  gutters: [{ 1: "#00FF00", 2: "#00FF00" }],
                };
              }),
              cobralLinter,
              wordHover,
            ]}
            onChange={onChange}
          />
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel minSize={12} className="bg-background/20">
          <div className="flex justify-between gap-1 p-1">
            <div>
              <Button variant="ghost" onClick={handleParse}>
                <CheckIcon className="mr-2" /> Executar
              </Button>

              <Button variant="ghost" onClick={handleParseStep}>
                <ArrowLeftIcon className="mr-2" /> Passo a passo
              </Button>
            </div>

            <div>
              <Button variant="ghost" onClick={clearLogs}>
                <CrumpledPaperIcon className="mr-2" /> Limpar
              </Button>
            </div>
          </div>
          <ScrollArea className="h-full px-2 pb-14 pt-1 flex flex-col [&>div>div>div]:mb-1.5">
            {logs.map((log, id) => {
              return <React.Fragment key={id}>{log}</React.Fragment>;
            })}
          </ScrollArea>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
};

export default CodeMirrorEditor;
