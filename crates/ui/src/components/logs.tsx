import { ReactNode, useContext, useEffect, useRef } from "react";

import AutoSizer from "react-virtualized-auto-sizer";
import { EditorContext } from "@/contexts/editor-context";
import { VariableSizeList } from "react-window";

export const Logs = () => {
  const { logs } = useContext(EditorContext);

  const rowHeights = useRef<{ [index: number]: number }>({});
  const listRef = useRef<VariableSizeList>(null);

  const scrollToBottom = () => {
    setTimeout(() => {
      listRef.current?.scrollToItem(logs.length - 1, "end");
    }, 0);
  };

  useEffect(() => {
    scrollToBottom();
  }, [logs]);

  const Row = ({
    index,
    style,
    data,
  }: {
    index: number;
    style: React.CSSProperties;
    data: ReactNode[];
  }) => {
    const rowRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
      if (!rowRef.current) return;

      const height =
        rowRef.current.getElementsByClassName("log-entry")[0]?.clientHeight;
      if (rowHeights.current[index] !== height) setRowHeight(index, height);
    }, [rowRef, index, data[index]]); // Ensure height is recalculated when row content changes.

    const setRowHeight = (index: number, height: number) => {
      rowHeights.current = { ...rowHeights.current, [index]: height };
      listRef.current?.resetAfterIndex(index); // Reset the size cache for the updated row.
    };

    return (
      <div ref={rowRef} className="px-2 min-h-9" style={{ ...style }}>
        {data[index]}
      </div>
    );
  };

  return (
    <AutoSizer>
      {({ height, width }) => (
        <VariableSizeList
          height={height - 48}
          itemCount={logs.length}
          estimatedItemSize={24}
          itemSize={(index) => rowHeights.current[index] || 24} // Default to 24px if not measured yet.
          width={width}
          itemData={logs}
          ref={listRef}
          className="bg-secondary border-t border-border"
        >
          {({ index, style }) => (
            <Row index={index} style={style} data={logs} />
          )}
        </VariableSizeList>
      )}
    </AutoSizer>
  );
};
