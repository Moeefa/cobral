import {
  AutoSizer,
  CellMeasurer,
  CellMeasurerCache,
  List,
  ListRowProps,
} from "react-virtualized";
import React, { useCallback, useContext, useEffect, useRef } from "react";

import { EditorContext } from "@/contexts/editor-context";

const cache = new CellMeasurerCache({
  minHeight: 24,
  fixedWidth: true,
});

export const Logs = React.memo(() => {
  const { logs } = useContext(EditorContext);
  const listRef = useRef<List>(null);

  useEffect(() => {
    cache.clearAll();
  }, [logs]);

  const rowRenderer = useCallback(
    ({ index, key, parent, style }: ListRowProps) => {
      return (
        <CellMeasurer
          cache={cache}
          columnIndex={0}
          key={key}
          parent={parent}
          rowIndex={index}
        >
          {({ registerChild }) => (
            <div
              style={{
                ...style,
                width: "100%",
              }}
              ref={registerChild}
              className="log-entry-container"
            >
              {logs[index]}
            </div>
          )}
        </CellMeasurer>
      );
    },
    [logs]
  );

  const scrollToBottom = useCallback(() => {
    const debounce = setTimeout(() => {
      listRef.current?.scrollToRow(logs.length - 1);
    }, 200);

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
          rowHeight={cache.rowHeight}
          rowRenderer={rowRenderer}
          overscanRowCount={20}
          deferredMeasurementCache={cache}
          className="border-border pl-2 bg-[var(--vscode-editor-background)] ![font-family:'SF_Pro_Mono',monospace]"
        />
      )}
    </AutoSizer>
  );
});
