import { AutoSizer, List } from "react-virtualized";
import React, { useCallback, useContext, useEffect, useRef } from "react";

import { EditorContext } from "@/contexts/editor-context";
import { listen } from "@tauri-apps/api/event";

export const Logs = React.memo(() => {
  const { logs } = useContext(EditorContext);
  const rowHeights = useRef<{ [index: number]: number }>({});
  const listRef = useRef<List>(null);

  const setRowHeight = useCallback((index: number, height: number) => {
    if (rowHeights.current[index] === height) return;
    rowHeights.current[index] = height;
    listRef.current?.recomputeRowHeights(index);
  }, []);

  const rowRenderer = useCallback(
    ({
      index,
      key,
      style,
    }: {
      index: number;
      key: React.Key;
      style: React.CSSProperties;
    }) => {
      return (
        <div
          key={key}
          style={style}
          className="log-entry-container"
          ref={(el) => {
            if (!el) return;
            const height = el
              .getElementsByClassName("log-entry")[0]
              .getBoundingClientRect().height;
            setRowHeight(index, height);
          }}
        >
          {logs[index]}
        </div>
      );
    },
    [logs, setRowHeight]
  );

  const scrollToBottom = useCallback(() => {
    const debounce = setTimeout(() => {
      listRef.current?.scrollToRow(logs.length - 1);
    }, 10);

    return () => clearTimeout(debounce);
  }, [logs]);

  useEffect(() => {
    scrollToBottom();
  }, [logs, scrollToBottom]);

  return (
    <AutoSizer>
      {({ width, height }) => (
        <List
          ref={listRef}
          width={width}
          height={height - 48}
          rowCount={logs.length}
          rowHeight={({ index }) => rowHeights.current[index] || 26}
          rowRenderer={rowRenderer}
          overscanRowCount={10}
          className="border-border pl-2 bg-[var(--vscode-editor-background)] ![font-family:'SF_Pro_Mono',monospace]"
        />
      )}
    </AutoSizer>
  );
});
