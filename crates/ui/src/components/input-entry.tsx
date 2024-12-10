import React, { useCallback } from "react";

import { Input } from "@/components/ui/input";

export const InputEntry = React.memo(
  ({
    message,
    timestamp,
    onSubmit,
  }: {
    message: string;
    timestamp: string;
    onSubmit: (value: string) => void;
  }) => {
    const handleKeyUp = useCallback(
      (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key !== "Enter") return;

        const inputElement = e.currentTarget;

        inputElement.disabled = true;

        onSubmit(inputElement.value);
      },
      [onSubmit]
    );

    return (
      <div className="flex items-start gap-4" data-key={`input-${message}`}>
        <span className="text-muted-foreground select-none">{timestamp}</span>
        <div className="flex gap-1 items-center w-full log-entry">
          <pre className="w-fit ![font-family:'SF_Pro_Mono',monospace]">
            {message}
          </pre>
          <Input
            key={`input-${message}`}
            className="![font-family:'SF_Pro_Mono',monospace] items-center log-input bg-foreground text-background h-max flex-1 rounded-none p-0 border-0 outline-none ring-0 focus-visible:ring-0 focus:ring-0"
            type="text"
            onKeyUp={handleKeyUp}
          />
        </div>
      </div>
    );
  }
);
