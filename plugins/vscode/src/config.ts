export interface Config {
    server: ServerConfig;
    checkOnSave: boolean;
};

export interface ServerConfig {
    extraEnv: Record<string, string> | null;
    path?: string;
};
