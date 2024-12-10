import { ArchiveIcon, BanIcon, SparklesIcon } from "lucide-react";
import { emit, listen } from "@tauri-apps/api/event";
import { useContext, useEffect, useRef } from "react";

import { Button } from "@/components/ui/button";
import { EditorContext } from "@/contexts/editor-context";
import { Logs } from "@/components/logs";
import { invoke } from "@tauri-apps/api/core";

export const Terminal = () => {
  const { clearLogs, value } = useContext(EditorContext);

  const runButton = useRef<HTMLButtonElement>(null);
  const breakButton = useRef<HTMLButtonElement>(null);

  const handleParse = async () => {
    runButton.current!.disabled = true;
    breakButton.current!.disabled = false;
    clearLogs();
    await invoke("parse", { input: value });
  };

  const handleBreak = async () => {
    runButton.current!.disabled = false;
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

  useEffect(() => {
    const finish = listen("exec_finished", () => {
      runButton.current!.disabled = false;
      breakButton.current!.disabled = true;

      emit("break_read");
      emit("break_exec");
    });

    return () => {
      finish.then((f) => f());
    };
  }, []);

  return (
    <>
      <div className="flex justify-between gap-1 p-1 bg-[var(--vscode-editor-background)]">
        <div>
          <Button
            ref={runButton}
            variant="expandIcon"
            Icon={SparklesIcon}
            iconPlacement="left"
            onClick={handleParse}
          >
            Executar
          </Button>

          <Button
            ref={breakButton}
            variant="expandIcon"
            Icon={BanIcon}
            iconPlacement="left"
            onClick={handleBreak}
          >
            Interromper
          </Button>
        </div>

        <div>
          <Button
            variant="expandIcon"
            Icon={ArchiveIcon}
            iconPlacement="left"
            onClick={clearLogs}
          >
            Limpar
          </Button>
        </div>
      </div>
    </>
  );
};
