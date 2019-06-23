export abstract class Medium {
    public abstract send(ctx: any, msg: any): void;
    public abstract async request<T>(ctx: any, req: any): Promise<T>;
}