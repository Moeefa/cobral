import { AutoSizer, IndexRange, InfiniteLoader, List } from "react-virtualized";
import React, {
  ReactNode,
  useCallback,
  useContext,
  useEffect,
  useLayoutEffect,
  useRef,
} from "react";

import { EditorContext } from "@/contexts/editor-context";

const Row = React.memo(
  ({
    index,
    style,
    data,
    rowHeights,
  }: {
    index: number;
    style: React.CSSProperties;
    data: ReactNode[];
    rowHeights: React.MutableRefObject<{ [index: number]: number }>;
  }) => {
    const rowRef = useRef<HTMLDivElement>(null);

    const updateRowHeight = (index: number, height: number) => {
      rowHeights.current[index] = height;
    };

    useLayoutEffect(() => {
      if (!rowRef.current) return;

      const logEntry = rowRef.current.getElementsByClassName("log-entry")[0];
      const measuredHeight = logEntry?.clientHeight;

      if (measuredHeight) updateRowHeight(index, measuredHeight);
    }, [index, data[index]]);

    return (
      <div
        ref={rowRef}
        style={style}
        data-index={index}
        className="log-entry-container"
      >
        {data[index]}
      </div>
    );
  },
  (prevProps, nextProps) => {
    return (
      prevProps.index === nextProps.index &&
      prevProps.data[prevProps.index] === nextProps.data[nextProps.index]
    );
  }
);

export const Logs = React.memo(() => {
  const { logs } = useContext(EditorContext);

  const rowHeights = useRef<{ [index: number]: number }>({});

  const isRowLoaded = ({ index }: { index: number }) => {
    return !!logs[index];
  };

  const loadMoreRows = async (params: IndexRange) => {
    return logs.slice(params.startIndex, params.stopIndex);
  };

  const rowRenderer = useCallback(
    ({
      index,
      key,
      style,
    }: {
      index: number;
      key: React.Key;
      style: React.CSSProperties;
    }) => (
      <Row
        index={index}
        key={key}
        style={style}
        data={logs}
        rowHeights={rowHeights}
      />
    ),
    [logs, rowHeights]
  );

  return (
    <AutoSizer>
      {({ width, height }) => (
        <InfiniteLoader
          isRowLoaded={isRowLoaded}
          loadMoreRows={loadMoreRows}
          rowCount={logs.length}
          minimumBatchSize={10}
        >
          {({ onRowsRendered, registerChild }) => (
            <List
              ref={registerChild}
              width={width}
              height={height - 48}
              rowCount={logs.length}
              rowHeight={({ index }) => rowHeights.current[index] || 26}
              rowRenderer={rowRenderer}
              onRowsRendered={onRowsRendered}
              scrollToIndex={logs.length - 1}
              scrollToAlignment="end"
              overscanRowCount={5}
              className="border-border px-2 bg-[var(--vscode-editor-background)] ![font-family:'SF_Pro_Mono',monospace]"
            />
          )}
        </InfiniteLoader>
      )}
    </AutoSizer>
  );
});
