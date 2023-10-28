import { IPointerBacked } from "./IPointerBacked";

export class AbstractPointerBacked implements IPointerBacked {

    protected pointer: number;

    constructor(pointer: number) {
        this.pointer = pointer;
    }

    destroy(): void {
        throw new Error("Method not implemented. Destroy() must be implemented by the extending class.");
    }
    
}