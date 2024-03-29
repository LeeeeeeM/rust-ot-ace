export const getWsUri = (): string => {
  return (
    (window.location.origin.startsWith("https") ? "wss://" : "ws://") +
    window.location.host +
    `/ws/socket`
  );
};
