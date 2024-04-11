export interface Config {
    server: ServerConfig;
    checkOnSave: boolean;
};

export interface ServerConfig {
    /**
     * Extra environment variables to be passed to the language server.
     */
    extraEnv: Record<string, string> | null;
    /**
     * The server binary to use.
     */
    path?: string;
};
