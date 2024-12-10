import React from "react";

export const LogEntry = React.memo(
  ({
    message,
    level,
    timestamp,
  }: {
    level: string;
    message: string;
    timestamp: string;
  }) => (
    <div className="flex items-start gap-4">
      <span className="text-muted-foreground select-none ![font-family:'SF_Pro_Mono',monospace]">
        {timestamp}
      </span>
      <pre
        data-level={level}
        className="![font-family:'SF_Pro_Mono',monospace] log-entry dark:data-[level=error]:bg-red-500 data-[level=error]:bg-red-400"
      >
        {message}
      </pre>
    </div>
  )
);
