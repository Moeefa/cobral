import { createContext, useEffect, useState } from "react";
import { emit, listen } from "@tauri-apps/api/event";

import { Input } from "@/components/ui/input";

interface EditorContextProps {
  value: string;
  logs: JSX.Element[];
  setValue: (value: string) => void;
  addLog: (log: { level: string; message: string }) => void;
  addInput: (message: string) => void;
  clearLogs: () => void;
}

export const EditorContext = createContext({} as EditorContextProps);

export function EditorProvider({ children }: { children: React.ReactNode }) {
  const [value, setValue] = useState("");
  const [setup, setSetup] = useState(false);
  const [logs, setLogs] = useState<JSX.Element[]>([]);

  const addLog = (log: { level: string; message: string }) => {
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
          <p
            data-level={log.level}
            className="dark:data-[level=error]:bg-red-500 data-[level=error]:bg-red-400"
          >
            {log.message}
          </p>
        </div>
      </>,
    ]);
  };

  const addInput = (message: string) => {
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
          <p className="w-fit">{message} </p>
          <Input
            key={new Date().toISOString()}
            className="flex-1 bg-muted/50 text-foreground h-6 rounded-lg"
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
  };

  const clearLogs = () => setLogs([]);

  useEffect(() => {
    if (setup) return;

    listen<string>("read", (event) => {
      addInput(event.payload);
    });

    listen<{ message: string; level: string }>("log", (log) =>
      addLog(log.payload)
    );

    listen("clear", () => setLogs([]));

    setSetup(true);
  }, []);

  return (
    <EditorContext.Provider
      value={{ value, setValue, logs, addInput, addLog, clearLogs }}
    >
      {children}
    </EditorContext.Provider>
  );
}
