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
      <p
        data-level={level}
        className="break-all whitespace-pre-wrap ![font-family:'SF_Pro_Mono',monospace] log-entry dark:data-[level=error]:bg-red-500 data-[level=error]:bg-red-400"
      >
        {message}
      </p>
    </div>
  )
);
