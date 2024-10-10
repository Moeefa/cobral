import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { DownloadIcon, FileIcon } from "@radix-ui/react-icons";
import {
  DropdownMenu,
  DropdownMenuArrow,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useEffect, useState } from "react";

import { Button } from "@/components/ui/button";
import ReactDOM from "react-dom/client";

// Trick to render custom titlebar in Decorum plugin for Tauri
export const Titlebar = () => {
  const [running, setRunning] = useState(true);

  useEffect(() => {
    if (!running) return;

    const element = document.querySelector<HTMLElement>(
      "div[data-tauri-decorum-tb] div[data-tauri-drag-region]"
    );

    if (element) {
      element.style.display = "flex";
      element.style.justifyContent = "space-between";

      // Render left content
      const left = document.createElement("div");
      element.insertAdjacentElement("afterbegin", left);

      const leftRoot = ReactDOM.createRoot(left);
      leftRoot.render(<Left />);

      // Render right content
      const right = document.createElement("div");
      element.insertAdjacentElement("beforeend", right);

      const rightRoot = ReactDOM.createRoot(right);
      rightRoot.render(<Right />);

      setRunning(false);
    }
  }, []);

  return <></>;
};

const Left = () => (
  <div data-tauri-drag-region className="flex items-center h-full px-4">
    <div data-tauri-drag-region className="flex items-center space-x-3">
      <small
        data-tauri-drag-region
        className="text-sm font-medium leading-none select-none"
      >
        arquivo.cl
      </small>
      <div className="flex gap-1">
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button size="icon" variant="ghost" className="size-7 rounded-sm">
                <DownloadIcon />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>Baixar arquivo</p>
            </TooltipContent>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button size="icon" variant="ghost" className="size-7 rounded-sm">
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

const Right = () => {
  return (
    <div data-tauri-drag-region className="flex items-center h-full px-4">
      <DropdownMenu>
        <DropdownMenuTrigger className="rounded-full">
          <Avatar className="size-6">
            <AvatarFallback className="text-xs">LH</AvatarFallback>
            <AvatarImage src="https://avatars.githubusercontent.com/u/32604322?v=4" />
          </Avatar>
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuLabel>Conta</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem>Configurações</DropdownMenuItem>
          <DropdownMenuItem>Documentação</DropdownMenuItem>
          <DropdownMenuItem>Sair</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
};
