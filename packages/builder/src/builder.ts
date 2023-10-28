import { componentize } from '@bytecodealliance/componentize-js';
import { readFile, writeFile } from 'node:fs/promises';
import yargs from 'yargs/yargs';



export type BuilderArguments = {
    /**
     * Path to the JS source code for your component.
     */
    js: string;

    /**
     * The path/filename for the generated WASM component
     */
    out: string;
}

export default class Builder {

    
    private builderArgs: BuilderArguments | Promise<BuilderArguments>;

    constructor(builderArgs?: BuilderArguments) {
        this.builderArgs = builderArgs ? {
            ...builderArgs,
            out: builderArgs.out || "./",
        } : this.constructArgs();
    }

    private async constructArgs(): Promise<BuilderArguments> {
        const args = await yargs(process.argv.slice(2)).options({
            js: {
                type: 'string',
                demandOption: true,
                describe: "Path to the JS source code for your component"
            },
            out: {
                type: 'string',
                default: "./",
                describe: "The path/filename for the generated WASM component"
            }
        }).argv;

        return {
            js: new URL(args.js, process.cwd()).pathname,
            out: new URL(args.out, process.cwd()).pathname
        };
    }

    /**
     * Builds the Ambient WASM component.
     */
    async build(): Promise<void> {
        const args = await this.builderArgs;

        let jsSource: string | null = null;
        
        try {
            jsSource = await readFile(args.js, 'utf8');
        } catch {
            console.error("Invalid JS source provided.");
            return;
        }
        
        const witPath = await readFile(new URL('../vendor/ambient/wit/root.wit', import.meta.url).pathname, 'utf8');
        
        const adapterPath = new URL('../vendor/ambient/wasi_snapshot_preview1.command.wasm', import.meta.url);
        
        const { component } = await componentize(jsSource, {
            // witWorld: witPath,
            witPath: new URL('../vendor/ambient/wit/', import.meta.url).pathname,
            worldName: "root",
            enableStdout: true,
            debug: true,
            preview2Adapter: adapterPath
        });
        
        await writeFile(new URL(args.out, import.meta.url).pathname, component);
    }

}