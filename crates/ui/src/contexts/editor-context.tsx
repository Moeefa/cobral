import { createContext, useCallback, useEffect, useRef, useState } from "react";
import { emit, listen } from "@tauri-apps/api/event";

import { Input } from "@/components/ui/input";
import React from "react";

interface EditorContextProps {
  value: string;
  theme: string;
  logs: React.JSX.Element[];
  file: string | null;
  setFile: (file: string | null) => void;
  setValue: (value: string) => void;
  setTheme: (theme: string) => void;
  addLog: (log: { level: string; message: string }[]) => void;
  addInput: (message: string) => void;
  clearLogs: () => void;
}

export const EditorContext = createContext({} as EditorContextProps);

const LogEntry = React.memo(
  ({ log }: { log: { level: string; message: string } }) => (
    <div className="flex items-start gap-4" key={log.message}>
      <span className="text-muted-foreground select-none">
        {new Date().toLocaleTimeString("pt-BR", {
          hour: "2-digit",
          minute: "2-digit",
          second: "2-digit",
        })}{" "}
      </span>
      <p
        data-level={log.level}
        className="log-entry dark:data-[level=error]:bg-red-500 data-[level=error]:bg-red-400"
      >
        {log.message}
      </p>
    </div>
  )
);

const InputEntry = React.memo(
  ({ message }: { message: string; onSubmit: (value: string) => void }) => {
    return (
      <div className="flex items-start gap-4">
        <span className="text-muted-foreground select-none">
          {new Date().toLocaleTimeString("pt-BR", {
            hour: "2-digit",
            minute: "2-digit",
            second: "2-digit",
          })}{" "}
        </span>
        <div className="flex gap-1 items-start w-full log-entry">
          <p className="w-fit">{message} </p>
          <Input
            key={new Date().toISOString()}
            className="log-input bg-foreground text-background h-max flex-1 rounded-lg"
            type="text"
            onSubmit={async (event) => {
              event.currentTarget.disabled = true;
              await emit("read_input", event.currentTarget.value);
            }}
          />
        </div>
      </div>
    );
  }
);

export function EditorProvider({ children }: { children: React.ReactNode }) {
  const [value, setValue] = useState(
    `declare x = 0;
declare y = 0;
`
  );
  const [theme, setTheme] = useState("quietlight");
  const [logs, setLogs] = useState<React.JSX.Element[]>([]);
  const [file, setFile] = useState<string | null>("arquivo.cl");

  const logBufferRef = useRef<React.JSX.Element[]>([]); // Ref to hold buffered logs
  const updateRequestedRef = useRef(false); // Ref to track rAF updates

  const processLogs = () => {
    // This function is called in requestAnimationFrame
    setLogs((prev) => [...prev, ...logBufferRef.current]);
    logBufferRef.current = []; // Clear the buffer
    updateRequestedRef.current = false; // Reset update flag
  };

  const scheduleLogUpdate = () => {
    if (!updateRequestedRef.current) {
      updateRequestedRef.current = true;
      requestAnimationFrame(processLogs); // Schedule rAF update
    }
  };

  const addLog = useCallback(
    (newLogs: { level: string; message: string }[]) => {
      logBufferRef.current.push(
        ...newLogs.map((log) => <LogEntry log={log} key={log.message} />)
      );
      scheduleLogUpdate();
    },
    []
  );

  const addInput = useCallback((message: string) => {
    logBufferRef.current.push(
      <InputEntry key={message} message={message} onSubmit={emit} />
    );
    scheduleLogUpdate();
  }, []);

  const clearLogs = useCallback(() => setLogs([]), []);

  useEffect(() => {
    const clearListener = listen("clear", () => setLogs([]));
    const readListener = listen<string>("read", (event) =>
      addInput(event.payload)
    );
    const logListener = listen<{ message: string; level: string }[]>(
      "log_batch",
      (log) => addLog(log.payload)
    );

    return () => {
      clearListener.then((unlisten) => unlisten());
      readListener.then((unlisten) => unlisten());
      logListener.then((unlisten) => unlisten());
    };
  }, [addLog, addInput]);

  return (
    <EditorContext.Provider
      value={{
        theme,
        setTheme,
        file,
        setFile,
        value,
        setValue,
        logs,
        addInput,
        addLog,
        clearLogs,
      }}
    >
      {children}
    </EditorContext.Provider>
  );
}
