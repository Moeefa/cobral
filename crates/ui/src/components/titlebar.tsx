import { DownloadIcon, FileIcon, GearIcon } from "@radix-ui/react-icons";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Link, Outlet } from "react-router-dom";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { open, save } from "@tauri-apps/plugin-dialog";
import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
import { useContext, useEffect, useState } from "react";

import { Button } from "@/components/ui/button";
import { EditorContext } from "@/contexts/editor-context";
import { basename } from "@tauri-apps/api/path";

// Trick to render custom titlebar in Decorum plugin for Tauri
export const Titlebar = () => {
  const [running, setRunning] = useState(true);

  useEffect(() => {
    const element = document.querySelector<HTMLElement>(
      "div[data-tauri-decorum-tb]"
    ) as HTMLElement;

    const root = document.getElementById("root") as HTMLElement;

    root.insertBefore(element, root.firstChild);
  }, []);

  useEffect(() => {
    if (!running) return;

    const element = document.querySelector<HTMLElement>(
      "div[data-tauri-decorum-tb] div[data-tauri-drag-region]"
    );

    if (element) {
      element.style.display = "flex";
      element.style.justifyContent = "space-between";

      // Render titlebar content
      const titlebar = document.querySelector("#titlebar") as HTMLElement;
      element.insertAdjacentElement("beforeend", titlebar);

      setRunning(false);
    }
  }, []);

  return (
    <>
      <div
        data-tauri-drag-region
        id="titlebar"
        className="flex justify-between items-center w-full"
      >
        <Left />
        <Right />
      </div>
      <div className="w-full overflow-auto h-[calc(100%-2rem)] mt-8">
        <Outlet />
      </div>
    </>
  );
};

const Left = () => {
  const { value, setValue, file, setFile } = useContext(EditorContext);
  const downloadFile = async () => {
    const file = await save({
      filters: [
        {
          name: "arquivo",
          extensions: ["cl"],
        },
      ],
    });

    await writeTextFile(file || "", value);
  };

  const readFile = async () => {
    const file = await open({
      multiple: false,
      directory: false,
      canCreateDirectories: true,
      filters: [
        {
          name: "arquivo",
          extensions: ["cl"],
        },
      ],
    });

    const content = await readTextFile(file || "");

    setValue(content || "");
    setFile((await basename(file!)) || "");
  };

  return (
    <div
      id="left"
      data-tauri-drag-region
      className="flex items-center h-full px-4"
    >
      <div data-tauri-drag-region className="flex items-center space-x-3">
        <small
          data-tauri-drag-region
          className="text-sm font-medium leading-none select-none"
        >
          {file || "arquivo.cl"}
        </small>
        <div className="flex gap-1">
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  size="icon"
                  variant="ghost"
                  className="size-7 rounded-sm"
                  onClick={downloadFile}
                >
                  <DownloadIcon />
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                <p>Baixar arquivo</p>
              </TooltipContent>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  size="icon"
                  variant="ghost"
                  className="size-7 rounded-sm"
                  onClick={readFile}
                >
                  <FileIcon />
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                <p>Abrir um arquivo</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>
      </div>
    </div>
  );
};

const Right = () => {
  return (
    <div
      id="right"
      data-tauri-drag-region
      className="flex items-center h-full px-4"
    >
      <DropdownMenu>
        <DropdownMenuTrigger className="outline-none rounded-full group size-6 flex items-center justify-center">
          <GearIcon className="group-hover:rotate-[55deg] transition-transform size-[1.05rem]" />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem>Configurações</DropdownMenuItem>
          <Link to="/docs">
            <DropdownMenuItem>Documentação</DropdownMenuItem>
          </Link>
          <Link to="/changelog">
            <DropdownMenuItem>Novidades</DropdownMenuItem>
          </Link>
          <DropdownMenuSeparator />
          <DropdownMenuItem>Sair</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
};
