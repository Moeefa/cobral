import { DownloadIcon, FileIcon } from "lucide-react";
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
import { basename } from "@tauri-apps/api/path";
import { useContext } from "react";

export const Sidebar = () => {
  return (
    <div className="h-full bg-background">
      <div className="flex flex-col h-full">
        <div
          data-tauri-drag-region
          className="flex items-center justify-between w-full border-b border-border min-h-11 p-[3.5px]"
        >
          <h1
            data-tauri-drag-region
            className="text-sm select-none font-semibold px-4 p-2.5"
          >
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
        <div className="flex gap-3">
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger onClick={downloadFile}>
                <DownloadIcon className="size-4" />
              </TooltipTrigger>
              <TooltipContent collisionPadding={5}>
                <p>Salvar arquivo</p>
              </TooltipContent>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger onClick={readFile}>
                <FileIcon className="size-4" />
              </TooltipTrigger>
              <TooltipContent collisionPadding={5}>
                <p>Abrir um arquivo</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>
      </div>
    </div>
  );
};

function FileTreeDemo() {
  const { file } = useContext(EditorContext);

  return (
    <Tree
      className="p-2 h-full overflow-hidden rounded-md bg-background"
      elements={ELEMENTS}
    >
      <File value="3">
        <p>{file}</p>
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
