declare global {
    export type ApiResult<T> = {
        success: boolean;
        message: string;
        payload?: T;
    };
}

export {};
