import CodeMirror, {
  EditorView,
  ViewUpdate,
  keymap,
} from "@uiw/react-codemirror";
import { useCallback, useContext, useEffect, useState } from "react";

import { EditorContext } from "@/contexts/editor-context";
import { cobral } from "@/lib/language/cobral";
import { cobralLinter } from "@/lib/language/linter";
import { indentationMarkers } from "@replit/codemirror-indentation-markers";
import { invoke } from "@tauri-apps/api/core";
import { okaidia } from "@uiw/codemirror-theme-okaidia";
import { quietlight } from "@uiw/codemirror-theme-quietlight";
import { showMinimap } from "@replit/codemirror-minimap";
import { vscodeKeymap } from "@replit/codemirror-vscode-keymap";
import { wordHover } from "@/lib/language/hover";

export const Editor = () => {
  const { value, setValue } = useContext(EditorContext);
  const [theme, setTheme] = useState(quietlight);

  const onChange = useCallback((val: string, _viewUpdate: ViewUpdate) => {
    setValue(val);
    invoke("update", { input: val });
  }, []);

  useEffect(() => {
    setTheme(
      window.matchMedia("(prefers-color-scheme: dark)").matches
        ? okaidia
        : quietlight
    );

    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", (event) => {
        event.matches ? setTheme(okaidia) : setTheme(quietlight);
      });
  }, []);

  return (
    <CodeMirror
      value={value}
      height="100%"
      width="100%"
      className="h-full w-full font-mono"
      theme={theme}
      extensions={[
        cobral(),
        keymap.of(vscodeKeymap),
        indentationMarkers(),
        showMinimap.compute(["doc"], (_state) => {
          return {
            create: (_v: EditorView) => {
              const dom = document.createElement("div");
              return { dom };
            },
            displayText: "blocks",
            showOverlay: "always",
            gutters: [{ 1: "#00FF00", 2: "#00FF00" }],
          };
        }),
        cobralLinter,
        wordHover,
      ]}
      onChange={onChange}
    />
  );
};
