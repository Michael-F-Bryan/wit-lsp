export type Config = {
    updates: UpdatesConfig;
    server: ServerConfig;
    checkOnSave: boolean;
};

export type UpdatesConfig = {
    channel: "nightly" | "stable";
    onStartup: boolean;
};

export type ServerConfig = {
    extraEnv: Record<string, string> | null;
    path?: string;
};
