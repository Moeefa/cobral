import {
  ArrowLeftIcon,
  CheckIcon,
  CrumpledPaperIcon,
} from "@radix-ui/react-icons";
import CodeMirror, { ViewUpdate } from "@uiw/react-codemirror";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { emit, listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useState } from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import React from "react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { cobral } from "../language/cobral";
import { cobralLinter } from "@/language/linter";
import { invoke } from "@tauri-apps/api/core";
import { okaidia } from "@uiw/codemirror-theme-okaidia";
import { quietlight } from "@uiw/codemirror-theme-quietlight";

const CodeMirrorEditor = () => {
  const [value, setValue] = useState("");
  const [logs, setLogs] = useState<JSX.Element[]>([]);
  const [setup, setSetup] = useState(false);
  const [theme, setTheme] = useState(quietlight);

  const onChange = useCallback((val: string, _viewUpdate: ViewUpdate) => {
    setValue(val);
    invoke("update", { input: val });
  }, []);

  const handleParse = async () => {
    const elapsed = performance.now();
    setLogs([]);
    await invoke("parse", { input: value });

    // setLogs([
    //   <>
    //     <div className="flex items-center gap-4">
    //       <span className="text-muted-foreground">
    //         {new Date().toLocaleTimeString("pt-BR", {
    //           hour: "2-digit",
    //           minute: "2-digit",
    //           second: "2-digit",
    //         })}{" "}
    //       </span>
    //       <p>Elapsed time: {performance.now() - elapsed}ms</p>
    //     </div>
    //   </>,
    // ]);
  };

  const handleParseStep = async () => {
    const payload = await invoke<{ current: number; length: number }>("step", {
      input: value,
    });

    if (payload.length === 0) {
      invoke("update", { input: value });
      setLogs([]);
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

  useEffect(() => {
    if (setup) return;

    listen<string>("read", (event) => {
      setLogs((prev) => [
        ...prev,
        <div className="flex items-center gap-4">
          <span className="text-muted-foreground">
            {new Date().toLocaleTimeString("pt-BR", {
              hour: "2-digit",
              minute: "2-digit",
              second: "2-digit",
            })}{" "}
          </span>
          <div className="flex gap-1 items-center w-full">
            <p className="w-fit">{event.payload} </p>
            <Input
              key={new Date().toISOString()}
              className="flex-1 bg-foreground text-background"
              type="text"
              onKeyDown={async (event) => {
                if (event.key === "Enter") {
                  event.currentTarget.disabled = true;
                  await emit("read_input", event.currentTarget.value);
                }
              }}
            />
          </div>
        </div>,
      ]);
    });

    listen<string>("log", (log) =>
      setLogs((prev) => [
        ...prev,
        <>
          <div className="flex items-center gap-4">
            <span className="text-muted-foreground">
              {new Date().toLocaleTimeString("pt-BR", {
                hour: "2-digit",
                minute: "2-digit",
                second: "2-digit",
              })}{" "}
            </span>
            <p>{log.payload}</p>
          </div>
        </>,
      ])
    );
    listen("clear", () => setLogs([]));

    setSetup(true);
  }, []);

  return (
    <div className="h-full w-full flex flex-col">
      <ResizablePanelGroup direction="vertical">
        <ResizablePanel>
          <CodeMirror
            value={value}
            height="100%"
            width="100%"
            className="pt-8 h-full w-full font-mono"
            theme={theme}
            extensions={[cobral(), cobralLinter]}
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
              <Button variant="ghost" onClick={() => setLogs([])}>
                <CrumpledPaperIcon className="mr-2" /> Limpar
              </Button>
            </div>
          </div>
          <ScrollArea className="h-full px-2 pb-14">
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
