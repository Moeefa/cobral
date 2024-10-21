import { useContext, useEffect, useRef } from "react";

import AutoSizer from "react-virtualized-auto-sizer";
import { EditorContext } from "@/contexts/editor-context";
import { FixedSizeList } from "react-window";

const Row = ({
  index,
  style,
  ...props
}: {
  index: number;
  style: React.CSSProperties | undefined;
  [index: string]: any;
}) => {
  const item = props.data[index];

  return (
    <div className="px-2" style={style}>
      {item}
    </div>
  );
};

export const Logs = () => {
  const { logs } = useContext(EditorContext);

  const listRef = useRef<FixedSizeList>(null);

  const scrollToBottom = () => {
    setTimeout(() => {
      listRef.current?.scrollToItem(logs.length - 1);
    }, 0);
  };

  useEffect(() => {
    scrollToBottom();
  }, [logs]);

  return (
    <AutoSizer>
      {({ height, width }) => (
        <FixedSizeList
          height={height - 47.2}
          itemCount={logs.length}
          itemSize={24}
          width={width}
          itemData={logs}
          ref={listRef}
          className="bg-secondary border-t border-border pb-1"
        >
          {Row}
        </FixedSizeList>
      )}
    </AutoSizer>
  );
};
