import "@/lib/monaco/setup";

import * as monaco from "monaco-editor-core";

import { useContext, useEffect, useLayoutEffect, useRef } from "react";

import { EditorContext } from "@/contexts/editor-context";
import { completionItemProvider } from "@/lib/monaco/completion";
import { linter } from "@/lib/monaco/linter";

export function MonacoEditor() {
  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor>();
  const completionItemProviderRef = useRef<monaco.IDisposable>();

  const { value, setValue } = useContext(EditorContext);

  const updateMarkers = async () => {
    const model = editorRef.current?.getModel();
    if (!model) return;

    const markers = await linter(model);
    monaco.editor.setModelMarkers(model, "cobral", markers);
    setValue(model.getValue() || "");
  };

  useLayoutEffect(() => {
    document.body.classList.add("dark");

    completionItemProviderRef.current?.dispose();

    completionItemProviderRef.current = completionItemProvider;
    editorRef.current = monaco.editor.create(
      document.getElementById("container") as HTMLElement,
      {
        value: value,
        language: "cobral",
        theme: "vitesse-dark",
        hover: {
          above: false,
        },

        showUnused: true,
        showDeprecated: true,
        showFoldingControls: "always",

        contextmenu: false,
        automaticLayout: true,
        smoothScrolling: true,
        cursorSmoothCaretAnimation: "on",

        autoClosingBrackets: "always",
        autoClosingQuotes: "always",
        autoClosingComments: "always",
        autoIndent: "full",

        fontFamily: "SF Pro Mono",
        tabSize: 4,
        fontSize: 16,
        lineHeight: 24,

        minimap: {
          enabled: true,
          autohide: true,
          renderCharacters: false,
        },
      }
    );

    document.fonts.ready.then(() => {
      monaco.editor.remeasureFonts();
    });

    editorRef.current?.getModel()?.onDidChangeContent(() => {
      updateMarkers();
    });

    updateMarkers();

    return () => {
      editorRef.current?.dispose();
    };
  }, []);

  useEffect(() => {
    updateMarkers();
    editorRef.current?.getModel()?.applyEdits([
      {
        range:
          editorRef.current.getModel()?.getFullModelRange() ||
          new monaco.Range(1, 1, 1, 1),
        text: value,
      },
    ]);
  }, [value]);

  return <div id="container" className="h-full w-full" />;
}
