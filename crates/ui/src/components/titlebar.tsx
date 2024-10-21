import { CogIcon, SettingsIcon } from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useContext, useEffect, useState } from "react";

import { Button } from "@/components/ui/button";
import { EditorContext } from "@/contexts/editor-context";
import { Link } from "react-router-dom";

// Trick to render custom titlebar in Decorum plugin for Tauri
export const Titlebar = ({ children }: { children: React.ReactNode }) => {
  const [running, setRunning] = useState(true);

  useEffect(() => {
    const element = document.querySelector<HTMLElement>(
      "div[data-tauri-decorum-tb]"
    ) as HTMLElement;

    const root = document.getElementById("main-layout") as HTMLElement;

    root.insertBefore(element, root.firstChild);
  }, []);

  useEffect(() => {
    if (!running) return;

    const element = document.querySelector<HTMLElement>(
      "div[data-tauri-decorum-tb] div[data-tauri-drag-region]"
    );

    if (element) {
      element.style.display = "flex";
      element.style.justifyContent = "justify-between";
      element.parentElement!.style.position = "static";

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
        className="flex justify-between items-center w-full bg-background"
      >
        <Left />
        <Right />
      </div>
      <div className="w-full overflow-auto h-full">{children}</div>
    </>
  );
};

const Left = () => {
  const { file } = useContext(EditorContext);

  return (
    <div
      id="left"
      data-tauri-drag-region
      className="flex items-center h-full gap-1 px-4 p-1"
    >
      <div
        data-tauri-drag-region
        className="border border-border aspect-square rounded-lg bg-background"
      >
        <h3 data-tauri-drag-region className="select-none p-1">
          <img src="/static/cobral.png" height={24} width={24} />
        </h3>
      </div>
      <h4
        data-tauri-drag-region
        className="text-base font-medium leading-none select-none"
      >
        {file || "arquivo.cl"}
      </h4>
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
      <Dialog>
        <DropdownMenu>
          <DropdownMenuTrigger className="outline-none group size-6 flex items-center justify-center">
            <SettingsIcon className="group-hover:rotate-[55deg] ease-in-out transition-transform size-[1.05rem]" />
          </DropdownMenuTrigger>
          <DropdownMenuContent>
            <DialogTrigger asChild>
              <DropdownMenuItem>Configurações</DropdownMenuItem>
            </DialogTrigger>
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
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Are you absolutely sure?</DialogTitle>
            <DialogDescription>
              This action cannot be undone. Are you sure you want to permanently
              delete this file from our servers?
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button type="submit">Confirm</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
};
