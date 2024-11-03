import CodeMirror, {
  EditorView,
  ViewUpdate,
  keymap,
} from "@uiw/react-codemirror";
import { useCallback, useContext, useEffect } from "react";

import { EditorContext } from "@/contexts/editor-context";
import { cobral } from "@/lib/language/cobral";
import { cobralLinter } from "@/lib/language/linter";
import { indentationMarkers } from "@replit/codemirror-indentation-markers";
import { invoke } from "@tauri-apps/api/core";
import { resolveTheme } from "@/lib/utils";
import { showMinimap } from "@replit/codemirror-minimap";
import { vscodeKeymap } from "@replit/codemirror-vscode-keymap";
import { wordHover } from "@/lib/language/hover";

export const Editor = () => {
  const { value, theme, setTheme, setValue } = useContext(EditorContext);

  const onChange = useCallback((val: string, _viewUpdate: ViewUpdate) => {
    setValue(val);
    invoke("update", { input: val });
  }, []);

  useEffect(() => {
    setTheme(
      window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "okaidia"
        : "quietlight"
    );

    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", (event) => {
        event.matches ? setTheme("okaidia") : setTheme("quietlight");
      });
  }, []);

  return (
    <CodeMirror
      value={value}
      height="100%"
      width="100%"
      className="h-full w-full font-mono"
      theme={resolveTheme(theme)}
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
            showOverlay: "mouse-over",
          };
        }),
        cobralLinter,
        wordHover,
      ]}
      onChange={onChange}
    />
  );
};
