import Builder from "../../src/builder";

describe("Builder Tests", () => {

    it("can build a JS file into an Ambient WASM component", async () => {
        const builder = new Builder({
            js: new URL("../utils/mock-component.js", import.meta.url).pathname,
            out: "../test_data/out.wasm"
        });

        await expect(builder.build()).resolves.not.toThrow();
    });

});