import {
  ArchiveIcon,
  BanIcon,
  FootprintsIcon,
  SparklesIcon,
} from "lucide-react";
import { useContext, useRef } from "react";

import { Button } from "@/components/ui/button";
import { EditorContext } from "@/contexts/editor-context";
import { Logs } from "@/components/logs";
import { emit } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export const Terminal = () => {
  const { clearLogs, value } = useContext(EditorContext);

  const runButton = useRef<HTMLButtonElement>(null);
  const stepButton = useRef<HTMLButtonElement>(null);
  const breakButton = useRef<HTMLButtonElement>(null);

  const handleParse = async () => {
    runButton.current!.disabled = true;
    stepButton.current!.disabled = true;
    breakButton.current!.disabled = false;
    clearLogs();
    await invoke("parse", { input: value });
  };

  const handleParseStep = async () => {
    runButton.current!.disabled = true;
    stepButton.current!.disabled = false;
    breakButton.current!.disabled = false;
    const payload = await invoke<{ current: number; length: number }>("step", {
      input: value,
    });

    if (payload.length === 0) {
      invoke("update", { input: value });
      clearLogs();
    }
  };

  const handleBreak = async () => {
    runButton.current!.disabled = false;
    stepButton.current!.disabled = false;
    breakButton.current!.disabled = true;

    // Get all inputs and disable them
    const inputs =
      document.querySelectorAll<HTMLInputElement>("input.log-input");
    inputs.forEach((input) => {
      input.disabled = true;
    });

    await emit("break_read");
    await emit("break_exec");
  };

  return (
    <>
      <div className="flex justify-between gap-1 p-1 bg-background">
        <div>
          <Button ref={runButton} variant="ghost" onClick={handleParse}>
            <SparklesIcon className="mr-2 size-4" /> Executar
          </Button>

          <Button ref={stepButton} variant="ghost" onClick={handleParseStep}>
            <FootprintsIcon className="mr-2 size-4" /> Passo a passo
          </Button>

          <Button ref={breakButton} variant="ghost" onClick={handleBreak}>
            <BanIcon className="mr-2 size-4" /> Interromper
          </Button>
        </div>

        <div>
          <Button variant="ghost" onClick={clearLogs}>
            <ArchiveIcon className="mr-2 size-4" /> Limpar
          </Button>
        </div>
      </div>
      <Logs />
    </>
  );
};
