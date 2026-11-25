import { useCallback, useState } from "react";

/** Runs a backend call and keeps its error for the UI; `null` means the call failed. */
export function useAction() {
  const [error, setError] = useState<string | null>(null);

  const run = useCallback(async <T>(action: () => Promise<T>): Promise<T | null> => {
    try {
      setError(null);
      return await action();
    } catch (e) {
      setError(String(e));
      return null;
    }
  }, []);

  const clearError = useCallback(() => setError(null), []);

  return { error, setError, clearError, run };
}
