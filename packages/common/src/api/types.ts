export type ApiResponse<T> = {
    success: boolean;
    message: string;
    payload?: T;
};

export type ApiResult<T> = {
    response: ApiResponse<T>;
    status: number;
};
