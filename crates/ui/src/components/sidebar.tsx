import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { File, Tree } from "@/components/ui/file-tree";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { open, save } from "@tauri-apps/plugin-dialog";
import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";

import { EditorContext } from "@/contexts/editor-context";
import { EllipsisIcon } from "lucide-react";
import { basename } from "@tauri-apps/api/path";
import { useContext } from "react";

export const Sidebar = () => {
  const isWeb = !("__TAURI_INTERNALS__" in window);

  return (
    <div
      data-web={isWeb}
      className="h-full data-[web=true]:bg-neutral-900 data-[web=false]:bg-[color-mix(in_srgb,var(--vscode-editor-background)_70%,transparent)]"
    >
      <div className="flex flex-col h-full">
        <div
          data-tauri-drag-region
          className="flex items-center justify-between w-full border-b border-border min-h-11 p-[3.5px]"
        >
          <h1 data-tauri-drag-region className="text-sm select-none px-4 p-2.5">
            Explorador
          </h1>
          <Actions />
        </div>
        <FileTreeDemo />
      </div>
    </div>
  );
};

const Actions = () => {
  const { value, setValue, setFile } = useContext(EditorContext);
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
    <div id="left" data-tauri-drag-region className="flex items-center px-4">
      <div data-tauri-drag-region className="flex items-center space-x-4">
        <div className="flex gap-1.5">
          <TooltipProvider>
            <DropdownMenu>
              <Tooltip>
                <TooltipTrigger asChild>
                  <DropdownMenuTrigger className="flex items-center justify-center">
                    <EllipsisIcon className="size-4" />
                  </DropdownMenuTrigger>
                </TooltipTrigger>
                <TooltipContent>Ações</TooltipContent>
              </Tooltip>
              <DropdownMenuContent className="w-24">
                <DropdownMenuItem onClick={downloadFile}>
                  Salvar arquivo
                </DropdownMenuItem>
                <DropdownMenuItem onClick={readFile}>
                  Abrir arquivo
                </DropdownMenuItem>
                <DropdownMenuItem>Abrir pasta</DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </TooltipProvider>
        </div>
      </div>
    </div>
  );
};

function FileTreeDemo() {
  const { file } = useContext(EditorContext);

  return (
    <Tree className="p-2 h-full overflow-hidden" elements={ELEMENTS}>
      <File value="3">
        <p className="select-none">{file}</p>
      </File>
    </Tree>
  );
}

const ELEMENTS = [
  {
    id: "1",
    isSelectable: true,
    name: "src",
    children: [
      {
        id: "2",
        isSelectable: true,
        name: "app",
        children: [
          {
            id: "3",
            isSelectable: true,
            name: "layout.tsx",
          },
          {
            id: "4",
            isSelectable: true,
            name: "page.tsx",
          },
        ],
      },
      {
        id: "5",
        isSelectable: true,
        name: "components",
        children: [
          {
            id: "6",
            isSelectable: true,
            name: "header.tsx",
          },
          {
            id: "7",
            isSelectable: true,
            name: "footer.tsx",
          },
        ],
      },
      {
        id: "8",
        isSelectable: true,
        name: "lib",
        children: [
          {
            id: "9",
            isSelectable: true,
            name: "utils.ts",
          },
        ],
      },
    ],
  },
];
