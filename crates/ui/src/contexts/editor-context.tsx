import { createContext, useCallback, useEffect, useRef, useState } from "react";
import { emit, listen } from "@tauri-apps/api/event";

import { InputEntry } from "@/components/input-entry";
import { LogEntry } from "@/components/log-entry";
import React from "react";

interface EditorContextProps {
  value: string;
  theme: string;
  logs: React.JSX.Element[];
  file: string | null;
  setFile: (file: string | null) => void;
  setValue: (value: string) => void;
  setTheme: (theme: string) => void;
  addLog: (logs: { level: string; message: string }[]) => void;
  addInput: (message: string) => void;
  clearLogs: () => void;
}

export const EditorContext = createContext({} as EditorContextProps);

export function EditorProvider({ children }: { children: React.ReactNode }) {
  const [value, setValue] = useState(
    `declare x = 0;
declare y = ler("Teste: ");
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
      const timestamp = new Date().toLocaleTimeString("pt-BR", {
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      });

      logBufferRef.current.push(
        ...newLogs.map((log) => (
          <LogEntry
            key={log.message}
            message={log.message}
            level={log.level}
            timestamp={timestamp}
          />
        ))
      );
      scheduleLogUpdate();
    },
    []
  );

  const addInput = useCallback((message: string) => {
    const timestamp = new Date().toLocaleTimeString("pt-BR", {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    });

    logBufferRef.current.push(
      <InputEntry
        key={message}
        message={message}
        timestamp={timestamp}
        onSubmit={async (value) => {
          await emit("read_input", value);
        }}
      />
    );
    scheduleLogUpdate();
  }, []);

  const clearLogs = useCallback(() => {
    logBufferRef.current = [];
    updateRequestedRef.current = false;

    setLogs([]);
  }, []);

  useEffect(() => {
    const clearListener = listen("clear", () => setLogs([]));

    const readListener = listen<string>("read", (event) =>
      addInput(event.payload)
    );

    const logListener = listen<{ message: string; level: string }[]>(
      "log_batch",
      (event) => addLog(event.payload)
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
