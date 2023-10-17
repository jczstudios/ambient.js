declare module '@bytecodealliance/componentize-js' {

    type ComponentizeOptions = {
        debug?: boolean;
        sourceName?: string;
        engine?: string;
        preview2Adapter?: any;
        witPath?: string;
        worldName: string;
        witWorld?: any;
        enableStdout?: boolean;
    }

    type ComponentizeResult = {
        component: any;
        imports: [string, string, number][];
    }

    export function componentize(jsSource: string, witWorld: string, opts?: ComponentizeOptions): Promise<ComponentizeResult>;
    export function componentize(jsSource: string, opts?: ComponentizeOptions): Promise<ComponentizeResult>;
}