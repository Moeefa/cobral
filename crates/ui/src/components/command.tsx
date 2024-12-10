import * as React from "react";

import { ArchiveIcon, FootprintsIcon, SparklesIcon } from "lucide-react";
import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";

export function CommandMenu() {
  const [open, setOpen] = React.useState(false);

  React.useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "P" && (e.metaKey || e.ctrlKey) && e.shiftKey) {
        e.preventDefault();
        setOpen((open) => !open);
      }
    };

    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, []);

  return (
    <>
      <CommandDialog open={open} onOpenChange={setOpen}>
        <CommandInput placeholder="Procurar..." />
        <CommandList>
          <CommandEmpty>Nenhum resultado encontrado</CommandEmpty>
          <CommandGroup heading="Ações">
            <CommandItem>
              <SparklesIcon />
              <span>Executar</span>
            </CommandItem>
            <CommandItem>
              <FootprintsIcon />
              <span>Passo a passo</span>
            </CommandItem>
            <CommandItem>
              <ArchiveIcon />
              <span>Limpar</span>
            </CommandItem>
          </CommandGroup>
        </CommandList>
      </CommandDialog>
    </>
  );
}
